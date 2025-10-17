use crate::syntax::extensions::item::ItemAttrsExt;
use darling::FromMeta;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{Attribute, Ident, Item, Meta};

/// Where a helper attribute was attached.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HelperSite {
    /// struct/enum (item-level)
    Item,
    /// struct Foo { #[helper] x: T }
    StructFieldNamed { field: Ident },
    /// struct Foo( #[helper] T, ... )
    StructFieldUnnamed { index: usize },
    /// enum Foo { #[helper] A, ... }
    Variant { variant: Ident },
    /// enum Foo { #[helper] x: T, ... }
    VariantFieldNamed { variant: Ident, field: Ident },
    /// enum Foo( #[helper] T, ... )
    VariantFieldUnnamed { variant: Ident, index: usize },
}

impl HelperSite {
    #[rustfmt::skip]
    pub fn is_allowed(&self, allowed: &AllowedSite) -> bool {
        matches!(
            (self, allowed),
            (HelperSite::Item, AllowedSite::Item)
                | (HelperSite::StructFieldNamed { .. }, AllowedSite::StructFieldNamed)
                | (HelperSite::StructFieldUnnamed { .. }, AllowedSite::StructFieldUnnamed)
                | (HelperSite::Variant { .. }, AllowedSite::Variant)
                | (HelperSite::VariantFieldNamed { .. }, AllowedSite::VariantFieldNamed)
                | (HelperSite::VariantFieldUnnamed { .. }, AllowedSite::VariantFieldUnnamed)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowedSite {
    Item,
    StructFieldNamed,
    StructFieldUnnamed,
    Variant,
    VariantFieldNamed,
    VariantFieldUnnamed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AllowedSiteConfig {
    #[default]
    Any,
    AllowedSites(Vec<AllowedSite>),
}

impl AllowedSiteConfig {
    pub fn is_allowed(&self, helper_site: &HelperSite) -> bool {
        let allowed_sites = match self {
            Self::Any => return true,
            Self::AllowedSites(sites) => sites,
        };
        allowed_sites
            .iter()
            .any(|allowed| helper_site.is_allowed(allowed))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AllowedSites(pub AllowedSiteConfig);

/// A single set of helpers scrubbed from one attachment site.
#[derive(Debug, Clone, PartialEq)]
pub struct StrippedHelper {
    pub site: HelperSite,
    pub attrs: Vec<Attribute>, // keep raw; parse later with FromMeta
}

/// All scrubbed helpers from the item.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StrippedHelpers {
    pub helpers: Vec<StrippedHelper>,
}

pub struct ScrubOutcome {
    /// Scrubbed item (no helper attributes remain)
    pub item: Item,
    /// Resolved ident for struct/enum
    pub ident: Ident,
    /// All sites - regardless of whether they were stripped or not
    pub all_sites: Vec<StrippedHelper>,
    /// All stripped helpers with their sites
    pub stripped_helpers: StrippedHelpers,
    /// Non-fatal errors encountered while scrubbing (we still emit the scrubbed item)
    pub errors: Vec<syn::Error>,
}

/// Scrub helpers from the whole item tree and resolve its ident.
/// Always strips helpers regardless of later success, so they never re-trigger compile errors.
pub fn scrub_helpers_and_ident(
    input: proc_macro2::TokenStream,
    is_helper: fn(&Attribute) -> bool,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
) -> syn::Result<ScrubOutcome> {
    scrub_helpers_and_ident_with_allowed_sites(input, |_, _| true, is_helper, resolve_ident)
}

/// Scrub helpers from the whole item tree and resolve its ident.
/// Returns Errors for any helpers that are not allowed on the given sites.
/// Always strips helpers regardless of later success, so they never re-trigger compile errors.
pub fn scrub_helpers_and_ident_with_allowed_sites(
    input: proc_macro2::TokenStream,
    is_helper_site_allowed: fn(&HelperSite, &Attribute) -> bool,
    is_helper: fn(&Attribute) -> bool,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
) -> syn::Result<ScrubOutcome> {
    let mut item: Item = syn::parse2(input)?;
    let ident = resolve_ident(&item)?.clone();

    #[derive(Debug, Default)]
    struct KeepHelpers {
        keep: Vec<Attribute>,
        helpers: Vec<Attribute>,
    }

    #[derive(Debug, Default)]
    #[repr(transparent)]
    struct Helpers(Vec<Attribute>);

    #[derive(Debug, Default)]
    struct ScrubberOut {
        all: Vec<StrippedHelper>,
        helpers: StrippedHelpers,
    }

    struct Scrubber {
        is_helper: fn(&Attribute) -> bool,
        out: ScrubberOut,
        errors: Vec<syn::Error>,
    }

    impl Scrubber {
        fn drain_split(&self, attrs: &mut Vec<Attribute>) -> KeepHelpers {
            let mut keep = Vec::with_capacity(attrs.len());
            let mut helpers = Vec::new();
            for a in attrs.drain(..) {
                if (self.is_helper)(&a) {
                    helpers.push(a);
                } else {
                    keep.push(a);
                }
            }
            KeepHelpers { keep, helpers }
        }

        fn take_helper_attrs(&mut self, it: &mut Item) -> Helpers {
            let mut attrs = match it.take_attrs() {
                Ok(attrs) => attrs,
                Err(err) => {
                    self.errors.push(syn::Error::new(
                        it.span(),
                        format!("Failed to parse attrs: {err}"),
                    ));
                    return Helpers::default();
                }
            };
            let KeepHelpers { keep, helpers } = self.drain_split(&mut attrs);
            // Put back what we kept; infallible
            it.put_attrs(keep).unwrap();
            Helpers(helpers)
        }
    }

    impl VisitMut for Scrubber {
        fn visit_item_mut(&mut self, it: &mut Item) {
            // Scrub item-level helpers (for both struct and enum)
            let Helpers(helpers) = self.take_helper_attrs(it);
            // failure is handled in `self.take_helper_attrs`
            let item_attrs = it.clone_attrs().unwrap_or_default();
            self.out.all.push(StrippedHelper {
                site: HelperSite::Item,
                attrs: item_attrs,
            });
            if !helpers.is_empty() {
                self.out.helpers.helpers.push(StrippedHelper {
                    site: HelperSite::Item,
                    attrs: helpers,
                });
            }
            syn::visit_mut::visit_item_mut(self, it);
        }

        fn visit_item_struct_mut(&mut self, it: &mut syn::ItemStruct) {
            // Fields of struct (named & unnamed)
            match &mut it.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &mut fields_named.named {
                        let field_ident = field.ident.clone().expect("named struct field");
                        let KeepHelpers { keep, helpers } = self.drain_split(&mut field.attrs);
                        field.attrs = keep.clone();
                        self.out.all.push(StrippedHelper {
                            site: HelperSite::StructFieldNamed {
                                field: field_ident.clone(),
                            },
                            attrs: keep,
                        });
                        if !helpers.is_empty() {
                            self.out.helpers.helpers.push(StrippedHelper {
                                site: HelperSite::StructFieldNamed { field: field_ident },
                                attrs: helpers,
                            });
                        }
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    for (ix, field) in fields_unnamed.unnamed.iter_mut().enumerate() {
                        let KeepHelpers { keep, helpers } = self.drain_split(&mut field.attrs);
                        field.attrs = keep.clone();
                        self.out.all.push(StrippedHelper {
                            site: HelperSite::StructFieldUnnamed { index: ix },
                            attrs: keep,
                        });
                        if !helpers.is_empty() {
                            self.out.helpers.helpers.push(StrippedHelper {
                                site: HelperSite::StructFieldUnnamed { index: ix },
                                attrs: helpers,
                            });
                        }
                    }
                }
                syn::Fields::Unit => {}
            }

            syn::visit_mut::visit_item_struct_mut(self, it);
        }

        fn visit_item_enum_mut(&mut self, it: &mut syn::ItemEnum) {
            // Variants + their fields
            for variant in &mut it.variants {
                let variant_ident = variant.ident.clone();

                // Variant-level helpers
                {
                    let KeepHelpers { keep, helpers } = self.drain_split(&mut variant.attrs);
                    variant.attrs = keep.clone();
                    self.out.all.push(StrippedHelper {
                        site: HelperSite::Variant {
                            variant: variant_ident.clone(),
                        },
                        attrs: keep,
                    });
                    if !helpers.is_empty() {
                        self.out.helpers.helpers.push(StrippedHelper {
                            site: HelperSite::Variant {
                                variant: variant_ident.clone(),
                            },
                            attrs: helpers,
                        });
                    }
                }

                // Variant fields
                match &mut variant.fields {
                    syn::Fields::Named(fields_named) => {
                        for field in &mut fields_named.named {
                            let field_ident = field.ident.clone().expect("named variant field");
                            let KeepHelpers { keep, helpers } = self.drain_split(&mut field.attrs);
                            field.attrs = keep.clone();
                            self.out.all.push(StrippedHelper {
                                site: HelperSite::VariantFieldNamed {
                                    variant: variant_ident.clone(),
                                    field: field_ident.clone(),
                                },
                                attrs: keep,
                            });
                            if !helpers.is_empty() {
                                self.out.helpers.helpers.push(StrippedHelper {
                                    site: HelperSite::VariantFieldNamed {
                                        variant: variant_ident.clone(),
                                        field: field_ident,
                                    },
                                    attrs: helpers,
                                });
                            }
                        }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        for (ix, field) in fields_unnamed.unnamed.iter_mut().enumerate() {
                            let KeepHelpers { keep, helpers } = self.drain_split(&mut field.attrs);
                            field.attrs = keep.clone();
                            self.out.all.push(StrippedHelper {
                                site: HelperSite::VariantFieldUnnamed {
                                    variant: variant_ident.clone(),
                                    index: ix,
                                },
                                attrs: keep.clone(),
                            });
                            if !helpers.is_empty() {
                                self.out.helpers.helpers.push(StrippedHelper {
                                    site: HelperSite::VariantFieldUnnamed {
                                        variant: variant_ident.clone(),
                                        index: ix,
                                    },
                                    attrs: helpers,
                                });
                            }
                        }
                    }
                    syn::Fields::Unit => {}
                }
            }

            syn::visit_mut::visit_item_enum_mut(self, it);
        }
    }

    let mut scrubber = Scrubber {
        is_helper,
        out: ScrubberOut::default(),
        errors: vec![],
    };
    scrubber.visit_item_mut(&mut item);

    for helper in &scrubber.out.helpers.helpers {
        for attr in &helper.attrs {
            if !is_helper_site_allowed(&helper.site, attr) {
                let attr_path = attr.path().to_token_stream().to_string().replace(" ", "");
                let attr_args = match attr.meta {
                    Meta::Path(_) => "",
                    Meta::List(_) => "(..)",
                    Meta::NameValue(_) => " = ...",
                };
                let helper_site = &helper.site;
                let message = format!(
                    "Helper #[{attr_path}{attr_args}] not allowed on this site: {helper_site:?} @ {ident}",
                );
                scrubber.errors.push(syn::Error::new(attr.span(), message));
            }
        }
    }

    Ok(ScrubOutcome {
        item,
        ident,
        all_sites: scrubber.out.all,
        stripped_helpers: scrubber.out.helpers,
        errors: scrubber.errors,
    })
}

/// Convenience: parse all stripped helpers into a concrete `THelperArgs: FromMeta`,
/// returning `(site, parsed)` pairs
pub fn parse_stripped_helpers_as<T: FromMeta>(
    stripped: &StrippedHelpers,
) -> darling::Result<Vec<(HelperSite, T)>> {
    let mut out = Vec::new();
    for helper in &stripped.helpers {
        for attr in &helper.attrs {
            let parsed = T::from_meta(&attr.meta)?;
            out.push((helper.site.clone(), parsed));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::analysis::item::resolve_ident_from_struct_or_enum;
    use crate::syntax::extensions::path::PathExt;
    use internal_test_proc_macro::xtest;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    impl StrippedHelpers {
        pub fn from_vec(helpers: Vec<StrippedHelper>) -> Self {
            Self { helpers }
        }
        pub fn to_test_string(&self) -> String {
            const INDENT: &str = "\n    ";
            let inner = self
                .helpers
                .iter()
                .map(|h| h.to_test_string().replace("\n", INDENT))
                .collect::<Vec<_>>()
                .join(INDENT);
            format!("{{{INDENT}{inner}\n}}")
        }
    }

    impl StrippedHelper {
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

    impl HelperSite {
        pub fn to_test_string(&self) -> String {
            match self {
                HelperSite::Item => "Item".to_string(),
                HelperSite::StructFieldNamed { field } => {
                    format!("StructFieldNamed {{ field: {} }}", field)
                }
                HelperSite::StructFieldUnnamed { index } => {
                    format!("StructFieldUnnamed {{ index: {} }}", index)
                }
                HelperSite::Variant { variant } => format!("Variant {{ variant: {} }}", variant),
                HelperSite::VariantFieldNamed { variant, field } => format!(
                    "VariantFieldNamed {{ variant: {}, field: {} }}",
                    variant, field
                ),
                HelperSite::VariantFieldUnnamed { variant, index } => format!(
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
            let helpers = scrub_outcome.stripped_helpers.helpers;
            let expected = vec![
                StrippedHelper {
                    site: HelperSite::Item,
                    attrs: vec![parse_quote!(#[item::helper])],
                },
                StrippedHelper {
                    site: HelperSite::StructFieldNamed {
                        field: parse_quote!(x),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                StrippedHelper {
                    site: HelperSite::StructFieldNamed {
                        field: parse_quote!(z),
                    },
                    attrs: vec![
                        parse_quote!(#[field::_1::helper]),
                        parse_quote!(#[field::_2::helper]),
                    ],
                },
            ];
            let got = StrippedHelpers::from_vec(helpers);
            let expected = StrippedHelpers::from_vec(expected);
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
            let helpers = scrub_outcome.stripped_helpers.helpers;
            let expected = vec![
                StrippedHelper {
                    site: HelperSite::Item,
                    attrs: vec![parse_quote!(#[item::helper])],
                },
                StrippedHelper {
                    site: HelperSite::Variant {
                        variant: parse_quote!(A),
                    },
                    attrs: vec![parse_quote!(#[field::helper])],
                },
                StrippedHelper {
                    site: HelperSite::Variant {
                        variant: parse_quote!(C),
                    },
                    attrs: vec![
                        parse_quote!(#[field::_1::helper]),
                        parse_quote!(#[field::_2::helper]),
                    ],
                },
            ];
            let got = StrippedHelpers::from_vec(helpers);
            let expected = StrippedHelpers::from_vec(expected);
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
            let helpers = scrub_outcome.all_sites;
            let expected = vec![
                StrippedHelper {
                    site: HelperSite::Item,
                    attrs: vec![parse_quote!(#[non_helper])],
                },
                StrippedHelper {
                    site: HelperSite::Variant {
                        variant: parse_quote!(A),
                    },
                    attrs: vec![parse_quote!(#[non_helper])],
                },
                StrippedHelper {
                    site: HelperSite::Variant {
                        variant: parse_quote!(B),
                    },
                    attrs: vec![parse_quote!(#[non_helper]), parse_quote!(#[non_helper])],
                },
                StrippedHelper {
                    site: HelperSite::Variant {
                        variant: parse_quote!(C),
                    },
                    attrs: vec![],
                },
            ];
            let got = StrippedHelpers::from_vec(helpers);
            let expected = StrippedHelpers::from_vec(expected);
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
