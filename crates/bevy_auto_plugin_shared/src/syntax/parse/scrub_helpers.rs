#![allow(dead_code)]
use crate::syntax::extensions::item::ItemAttrsExt;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{Attribute, Ident, Item, Meta};

/// Where an attribute was attached.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrSite {
    Item,
    /// struct Foo { #[helper] x: T }
    StructFieldNamed {
        field: Ident,
    },
    /// struct Foo( #[helper] T, ... )
    StructFieldUnnamed {
        index: usize,
    },
    /// enum Foo { #[helper] A, ... }
    Variant {
        variant: Ident,
    },
    /// enum Foo { #[helper] x: T, ... }
    VariantFieldNamed {
        variant: Ident,
        field: Ident,
    },
    /// enum Foo( #[helper] T, ... )
    VariantFieldUnnamed {
        variant: Ident,
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SiteClass {
    Item,
    StructFieldNamed,
    StructFieldUnnamed,
    Variant,
    VariantFieldNamed,
    VariantFieldUnnamed,
}

impl AttrSite {
    #[rustfmt::skip]
    pub fn matches_class(&self, class: &SiteClass) -> bool {
        matches!(
            (self, class),
            (AttrSite::Item, SiteClass::Item)
            | (AttrSite::StructFieldNamed { .. }, SiteClass::StructFieldNamed)
            | (AttrSite::StructFieldUnnamed { .. }, SiteClass::StructFieldUnnamed)
            | (AttrSite::Variant { .. }, SiteClass::Variant)
            | (AttrSite::VariantFieldNamed { .. }, SiteClass::VariantFieldNamed)
            | (AttrSite::VariantFieldUnnamed { .. }, SiteClass::VariantFieldUnnamed)
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum SiteFilter {
    #[default]
    Any,
    Only(Vec<SiteClass>),
}
impl SiteFilter {
    pub fn any() -> Self {
        Self::Any
    }
    pub fn only(classes: impl Into<Vec<SiteClass>>) -> Self {
        Self::Only(classes.into())
    }
    pub fn allows(&self, site: &AttrSite) -> bool {
        match self {
            SiteFilter::Any => true,
            SiteFilter::Only(list) => list.iter().any(|c| site.matches_class(c)),
        }
    }
}

/// Attributes found at a single site.
#[derive(Debug, Clone, PartialEq)]
pub struct SiteAttrs {
    pub site: AttrSite,
    pub attrs: Vec<Attribute>,
}

/// Thin wrapper: avoids `helpers.helpers` and adds utilities.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SiteAttrsVec(pub Vec<SiteAttrs>);

impl SiteAttrsVec {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn push(&mut self, site: AttrSite, attrs: Vec<Attribute>) {
        self.0.push(SiteAttrs { site, attrs });
    }
    pub fn iter(&self) -> impl Iterator<Item = &SiteAttrs> {
        self.0.iter()
    }

    /// Convenience: parse each attr group to `T: FromMeta`, yielding `(site, parsed)` pairs.
    pub fn parse_as<T: FromMeta>(&self) -> darling::Result<Vec<(AttrSite, T)>> {
        let mut out = Vec::new();
        for sa in &self.0 {
            for a in &sa.attrs {
                out.push((sa.site.clone(), T::from_meta(&a.meta)?));
            }
        }
        Ok(out)
    }
}

pub struct ScrubOutcome {
    /// scrubbed item (no helpers remain)
    pub item: Item,
    /// struct/enum ident
    pub ident: Ident,
    /// attrs kept per site (non-helpers or empty)
    pub observed: SiteAttrsVec,
    /// helper attrs removed per site
    pub removed: SiteAttrsVec,
    /// Errors encountered while scrubbing (we still emit the scrubbed item)
    pub errors: Vec<syn::Error>,
}

impl ScrubOutcome {
    pub fn all_with_removed_attrs(&self) -> Vec<SiteAttrs> {
        let mut out = Vec::with_capacity(self.observed.len());
        for group in self.observed.iter() {
            let mut attrs = vec![];
            // linear search is fine here, since we expect the number of groups to be small
            if let Some(removed_attrs) = self
                .removed
                .iter()
                .find(|g| g.site == group.site)
                .map(|g| g.attrs.clone())
            {
                attrs.extend(removed_attrs);
            }
            out.push(SiteAttrs {
                site: group.site.clone(),
                attrs,
            });
        }
        out
    }
    pub fn write_back(&self, token_stream: &mut TokenStream) -> syn::Result<()> {
        let item = &self.item;
        *token_stream = quote! {
            #item
        };

        // inject any scrub errors as compile_error! right here.
        if !self.errors.is_empty() {
            let err_ts = self.errors.iter().map(syn::Error::to_compile_error);
            *token_stream = quote! {
                #( #err_ts )*
                #token_stream
            };
            let mut err = syn::Error::new(item.span(), "failed to scrub helpers");
            err.extend(self.errors.clone());
            return Err(err);
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct KeepSplit {
    keep: Vec<Attribute>,
    removed: Vec<Attribute>,
}

#[derive(Debug, Default)]
struct ScrubberOut {
    observed: SiteAttrsVec,
    removed: SiteAttrsVec,
}

struct Scrubber {
    is_helper: fn(&Attribute) -> bool,
    out: ScrubberOut,
    errors: Vec<syn::Error>,
}

impl Scrubber {
    fn drain_split(&self, attrs: &mut Vec<Attribute>) -> KeepSplit {
        let mut keep = Vec::with_capacity(attrs.len());
        let mut removed = Vec::new();
        for a in attrs.drain(..) {
            if (self.is_helper)(&a) {
                removed.push(a);
            } else {
                keep.push(a);
            }
        }
        KeepSplit { keep, removed }
    }

    fn take_helper_attrs(&mut self, it: &mut Item) -> Vec<Attribute> {
        let mut attrs = match it.take_attrs() {
            Ok(attrs) => attrs,
            Err(err) => {
                self.errors.push(syn::Error::new(
                    it.span(),
                    format!("Failed to parse attrs: {err}"),
                ));
                return Vec::new();
            }
        };
        let KeepSplit { keep, removed } = self.drain_split(&mut attrs);
        let _ = it.put_attrs(keep.clone());
        removed
    }
}

impl VisitMut for Scrubber {
    fn visit_item_mut(&mut self, it: &mut Item) {
        let removed = self.take_helper_attrs(it);
        // errors handled in `take_helper_attrs`
        let item_attrs = it.clone_attrs().unwrap_or_default();
        self.out.observed.push(AttrSite::Item, item_attrs);
        if !removed.is_empty() {
            self.out.removed.push(AttrSite::Item, removed);
        }
        syn::visit_mut::visit_item_mut(self, it);
    }

    fn visit_item_struct_mut(&mut self, it: &mut syn::ItemStruct) {
        match &mut it.fields {
            syn::Fields::Named(fields_named) => {
                for field in &mut fields_named.named {
                    let field_ident = field.ident.clone().expect("named struct field");
                    let KeepSplit { keep, removed } = self.drain_split(&mut field.attrs);
                    field.attrs = keep.clone();
                    self.out.observed.push(
                        AttrSite::StructFieldNamed {
                            field: field_ident.clone(),
                        },
                        keep,
                    );
                    if !removed.is_empty() {
                        self.out
                            .removed
                            .push(AttrSite::StructFieldNamed { field: field_ident }, removed);
                    }
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                for (index, field) in fields_unnamed.unnamed.iter_mut().enumerate() {
                    let KeepSplit { keep, removed } = self.drain_split(&mut field.attrs);
                    field.attrs = keep.clone();
                    self.out
                        .observed
                        .push(AttrSite::StructFieldUnnamed { index }, keep);
                    if !removed.is_empty() {
                        self.out
                            .removed
                            .push(AttrSite::StructFieldUnnamed { index }, removed);
                    }
                }
            }
            syn::Fields::Unit => {}
        }
        syn::visit_mut::visit_item_struct_mut(self, it);
    }

    fn visit_item_enum_mut(&mut self, it: &mut syn::ItemEnum) {
        for variant in &mut it.variants {
            let variant_ident = variant.ident.clone();

            // variant-level
            let KeepSplit { keep, removed } = self.drain_split(&mut variant.attrs);
            variant.attrs = keep.clone();
            self.out.observed.push(
                AttrSite::Variant {
                    variant: variant_ident.clone(),
                },
                keep,
            );
            if !removed.is_empty() {
                self.out.removed.push(
                    AttrSite::Variant {
                        variant: variant_ident.clone(),
                    },
                    removed,
                );
            }

            // fields
            match &mut variant.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &mut fields_named.named {
                        let field_ident = field.ident.clone().expect("named variant field");
                        let KeepSplit { keep, removed } = self.drain_split(&mut field.attrs);
                        field.attrs = keep.clone();
                        self.out.observed.push(
                            AttrSite::VariantFieldNamed {
                                variant: variant_ident.clone(),
                                field: field_ident.clone(),
                            },
                            keep,
                        );
                        if !removed.is_empty() {
                            self.out.removed.push(
                                AttrSite::VariantFieldNamed {
                                    variant: variant_ident.clone(),
                                    field: field_ident,
                                },
                                removed,
                            );
                        }
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    for (index, field) in fields_unnamed.unnamed.iter_mut().enumerate() {
                        let KeepSplit { keep, removed } = self.drain_split(&mut field.attrs);
                        field.attrs = keep.clone();
                        self.out.observed.push(
                            AttrSite::VariantFieldUnnamed {
                                variant: variant_ident.clone(),
                                index,
                            },
                            keep,
                        );
                        if !removed.is_empty() {
                            self.out.removed.push(
                                AttrSite::VariantFieldUnnamed {
                                    variant: variant_ident.clone(),
                                    index,
                                },
                                removed,
                            );
                        }
                    }
                }
                syn::Fields::Unit => {}
            }
        }
        syn::visit_mut::visit_item_enum_mut(self, it);
    }
}

pub fn scrub_helpers_and_ident(
    input: proc_macro2::TokenStream,
    is_helper: fn(&Attribute) -> bool,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
) -> syn::Result<ScrubOutcome> {
    scrub_helpers_and_ident_with_filter(input, |_, _| true, is_helper, resolve_ident)
}

pub fn scrub_helpers_and_ident_with_filter(
    input: proc_macro2::TokenStream,
    is_site_allowed: fn(&AttrSite, &Attribute) -> bool,
    is_helper: fn(&Attribute) -> bool,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
) -> syn::Result<ScrubOutcome> {
    let mut item: Item = syn::parse2(input)?;
    let ident = resolve_ident(&item)?.clone();

    let mut scrubber = Scrubber {
        is_helper,
        out: ScrubberOut::default(),
        errors: vec![],
    };
    scrubber.visit_item_mut(&mut item);

    // validate “removed” helpers against the site filter
    for group in &scrubber.out.removed.0 {
        for attr in &group.attrs {
            if !is_site_allowed(&group.site, attr) {
                let path = attr.path().to_token_stream().to_string().replace(' ', "");
                let args = match attr.meta {
                    Meta::Path(_) => "",
                    Meta::List(_) => "(..)",
                    Meta::NameValue(_) => " = ...",
                };
                let msg = format!(
                    "Helper #[{path}{args}] not allowed on this site: {:?} @ {ident}",
                    group.site
                );
                scrubber.errors.push(syn::Error::new(attr.span(), msg));
            }
        }
    }

    Ok(ScrubOutcome {
        item,
        ident,
        observed: scrubber.out.observed,
        removed: scrubber.out.removed,
        errors: scrubber.errors,
    })
}

// Parse removed helpers into `T`
pub fn parse_removed_as<T: FromMeta>(
    removed: &SiteAttrsVec,
) -> darling::Result<Vec<(AttrSite, T)>> {
    removed.parse_as::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::analysis::item::resolve_ident_from_struct_or_enum;
    use crate::syntax::extensions::path::PathExt;
    use internal_test_proc_macro::xtest;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    impl SiteAttrsVec {
        pub fn from_vec(site_attrs: Vec<SiteAttrs>) -> Self {
            Self(site_attrs)
        }
        pub fn to_test_string(&self) -> String {
            const INDENT: &str = "\n    ";
            let inner = self
                .0
                .iter()
                .map(|h| h.to_test_string().replace("\n", INDENT))
                .collect::<Vec<_>>()
                .join(INDENT);
            format!("{{{INDENT}{inner}\n}}")
        }
    }

    impl SiteAttrs {
        pub fn to_test_string(&self) -> String {
            format!(
                "{}\n{}\n",
                self.attrs
                    .iter()
                    .map(|a| a.to_token_stream().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
                self.site.to_test_string(),
            )
        }
    }

    impl AttrSite {
        pub fn to_test_string(&self) -> String {
            match self {
                Self::Item => "Item".to_string(),
                Self::StructFieldNamed { field } => {
                    format!("StructFieldNamed {{ field: {} }}", field)
                }
                Self::StructFieldUnnamed { index } => {
                    format!("StructFieldUnnamed {{ index: {} }}", index)
                }
                Self::Variant { variant } => format!("Variant {{ variant: {} }}", variant),
                Self::VariantFieldNamed { variant, field } => format!(
                    "VariantFieldNamed {{ variant: {}, field: {} }}",
                    variant, field
                ),
                Self::VariantFieldUnnamed { variant, index } => format!(
                    "VariantFieldUnnamed {{ variant: {}, index: {} }}",
                    variant, index
                ),
            }
        }
    }

    mod scrub_helpers_and_ident {
        use super::*;
        #[inline]
        fn assert_no_errors(scrub_outcome: &ScrubOutcome) {
            assert_eq!(
                scrub_outcome
                    .errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
                Vec::<String>::new()
            );
        }

        #[inline]
        fn assert_ident(scrub_outcome: &ScrubOutcome, expected: &str) {
            assert_eq!(scrub_outcome.ident.to_token_stream().to_string(), expected);
        }

        #[inline]
        fn assert_no_helpers_remain_on_item(scrub_outcome: &ScrubOutcome) {
            let ts = scrub_outcome.item.to_token_stream().to_string();
            assert!(
                !ts.contains("::helper"),
                "item still has helper attributes: {}",
                ts
            );
        }

        fn is_helper(attr: &Attribute) -> bool {
            attr.path().is_similar_path_or_ident(&parse_quote!(helper))
        }

        fn resolve_ident(item: &Item) -> syn::Result<&Ident> {
            resolve_ident_from_struct_or_enum(item).map_err(|err| {
                syn::Error::new(item.span(), format!("failed to resolve ident: {err}"))
            })
        }

        #[xtest]
        fn test_scrub_helpers_and_ident_struct_field() -> syn::Result<()> {
            let input = quote! {
                #[item::helper]
                struct Foo {
                    #[field::helper]
                    x: i32,
                    y: i32,
                    #[field::_1::helper]
                    #[field::_2::helper]
                    z: i32,
                }
            };
            let scrub_outcome = scrub_helpers_and_ident(input, is_helper, resolve_ident)?;
            assert_no_errors(&scrub_outcome);
            assert_ident(&scrub_outcome, "Foo");
            assert_no_helpers_remain_on_item(&scrub_outcome);
            let got = scrub_outcome.removed;
            let expected = SiteAttrsVec::from_vec(vec![
                SiteAttrs {
                    site: AttrSite::Item,
                    attrs: vec![parse_quote!(#[item::helper])],
                },
                SiteAttrs {
                    site: AttrSite::StructFieldNamed {
                        field: parse_quote!(x),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                SiteAttrs {
                    site: AttrSite::StructFieldNamed {
                        field: parse_quote!(z),
                    },
                    attrs: vec![
                        parse_quote!(#[field::_1::helper]),
                        parse_quote!(#[field::_2::helper]),
                    ],
                },
            ]);
            assert_eq!(
                got.to_test_string(),
                expected.to_test_string(),
                "helpers got:\n{}\n\nexpected:\n{}",
                got.to_test_string(),
                expected.to_test_string(),
            );
            Ok(())
        }

        #[xtest]
        fn test_scrub_helpers_and_ident_variant() -> syn::Result<()> {
            let input = quote! {
                #[item::helper]
                enum Foo {
                    #[field::helper]
                    A,
                    B,
                    #[field::_1::helper]
                    #[field::_2::helper]
                    C,
                }
            };
            let scrub_outcome = scrub_helpers_and_ident(input, is_helper, resolve_ident)?;
            assert_no_errors(&scrub_outcome);
            assert_ident(&scrub_outcome, "Foo");
            assert_no_helpers_remain_on_item(&scrub_outcome);
            let got = scrub_outcome.removed;
            let expected = SiteAttrsVec::from_vec(vec![
                SiteAttrs {
                    site: AttrSite::Item,
                    attrs: vec![parse_quote!(#[item::helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(A),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(C),
                    },
                    attrs: vec![
                        parse_quote!(#[field::_1::helper]),
                        parse_quote!(#[field::_2::helper]),
                    ],
                },
            ]);
            assert_eq!(
                got.to_test_string(),
                expected.to_test_string(),
                "helpers got:\n{}\n\nexpected:\n{}",
                got.to_test_string(),
                expected.to_test_string(),
            );
            Ok(())
        }

        #[xtest]
        fn test_scrub_helpers_and_ident_preserve_non_helpers() -> syn::Result<()> {
            let input = quote! {
                #[item::helper]
                #[non_helper]
                enum Foo {
                    #[field::helper]
                    #[non_helper]
                    A,
                    #[non_helper]
                    B,
                    #[field::_1::helper]
                    #[field::_2::helper]
                    #[non_helper]
                    C,
                    #[field::helper]
                    X,
                    Y,
                }
            };
            let scrub_outcome = scrub_helpers_and_ident(input, is_helper, resolve_ident)?;
            assert_no_errors(&scrub_outcome);
            assert_ident(&scrub_outcome, "Foo");
            assert_no_helpers_remain_on_item(&scrub_outcome);
            let item = scrub_outcome.item;
            assert_eq!(
                item.to_token_stream().to_string(),
                quote! {
                    #[non_helper]
                    enum Foo {
                        #[non_helper]
                        A,
                        #[non_helper]
                        B,
                        #[non_helper]
                        C,
                        X,
                        Y,
                    }
                }
                .to_string(),
            );
            Ok(())
        }

        #[xtest]
        fn test_scrub_helpers_and_ident_all_sites() -> syn::Result<()> {
            let input = quote! {
                #[item::helper]
                #[non_helper]
                enum Foo {
                    #[field::helper]
                    #[non_helper]
                    A,
                    #[non_helper]
                    #[non_helper]
                    B,
                    #[field::_1::helper]
                    #[field::_2::helper]
                    C,
                }
            };
            let scrub_outcome = scrub_helpers_and_ident(input, is_helper, resolve_ident)?;
            assert_no_errors(&scrub_outcome);
            assert_ident(&scrub_outcome, "Foo");
            assert_no_helpers_remain_on_item(&scrub_outcome);
            let got = scrub_outcome.observed;
            let expected = SiteAttrsVec::from_vec(vec![
                SiteAttrs {
                    site: AttrSite::Item,
                    attrs: vec![parse_quote!(#[non_helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(A),
                    },
                    attrs: vec![parse_quote!(#[non_helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(B),
                    },
                    attrs: vec![parse_quote!(#[non_helper]), parse_quote!(#[non_helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(C),
                    },
                    attrs: vec![],
                },
            ]);
            assert_eq!(
                got.to_test_string(),
                expected.to_test_string(),
                "helpers got:\n{}\n\nexpected:\n{}",
                got.to_test_string(),
                expected.to_test_string(),
            );
            Ok(())
        }

        #[xtest]
        fn test_scrub_helpers_and_ident_all_with_removed_attrs() -> syn::Result<()> {
            let input = quote! {
                #[item::helper]
                #[non_helper]
                enum Foo {
                    #[field::helper]
                    #[non_helper]
                    A,
                    #[non_helper]
                    B,
                    #[field::_1::helper]
                    #[field::_2::helper]
                    #[non_helper]
                    C,
                    #[field::helper]
                    X,
                    Y,
                }
            };

            let scrub_outcome = scrub_helpers_and_ident(input, is_helper, resolve_ident)?;

            let got = SiteAttrsVec::from_vec(scrub_outcome.all_with_removed_attrs());
            let expected = SiteAttrsVec::from_vec(vec![
                SiteAttrs {
                    site: AttrSite::Item,
                    attrs: vec![parse_quote!(#[item::helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(A),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(B),
                    },
                    attrs: vec![],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(C),
                    },
                    attrs: vec![
                        parse_quote!(#[field::_1::helper]),
                        parse_quote!(#[field::_2::helper]),
                    ],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(X),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                SiteAttrs {
                    site: AttrSite::Variant {
                        variant: parse_quote!(Y),
                    },
                    attrs: vec![],
                },
            ]);

            assert_eq!(
                got.to_test_string(),
                expected.to_test_string(),
                "helpers got:\n{}\n\nexpected:\n{}",
                got.to_test_string(),
                expected.to_test_string(),
            );

            Ok(())
        }
    }
}
