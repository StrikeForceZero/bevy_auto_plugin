use crate::__private::attribute_args::{GenericsArgs, ItemAttributeArgs, WithTargetPath};
use crate::__private::generics::{Generics, GenericsCollection};
use crate::__private::type_list::TypeList;
use crate::__private::util::extensions::from_meta::FromMetaExt;
use crate::__private::util::extensions::path::PathExt;
use crate::__private::util::generics::CountGenerics;
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::__private::util::path_fmt::{PathWithoutGenerics, TryFromPathWithoutGenericsError};
use darling::FromMeta;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Item, Path, PathArguments, parse2};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ConcreteTargetPath {
    pub target: PathWithoutGenerics,
    pub generics: Generics,
    pub turbofish: bool,
}

impl ToTokens for ConcreteTargetPath {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        let path = &self.target;
        let generics = &self.generics;
        tokens.extend(if generics.is_empty() {
            // TODO: generics already handles tokens properly when empty but we shoehorned the turbofish flag
            //  seemed more appropriate than forcing Generics to inherit the complexity and  become an enum for both variants
            //  but there's likely a better way?
            quote! { #path }
        } else if self.turbofish {
            quote! { #path :: #generics }
        } else {
            quote! { #path #generics }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ConcreteTargetPathWithGenericsCollection {
    pub target: PathWithoutGenerics,
    pub generics: GenericsCollection,
    pub turbofish: bool,
}

impl ConcreteTargetPathWithGenericsCollection {
    pub fn from_args<T: GenericsArgs>(path: PathWithoutGenerics, args: &T) -> Self {
        Self {
            target: path,
            generics: args.generics(),
            turbofish: T::TURBOFISH,
        }
    }
}

impl IntoIterator for ConcreteTargetPathWithGenericsCollection {
    type Item = ConcreteTargetPath;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // TODO: better way?
        self.generics
            .iter_with_default_generics_when_empty()
            .into_iter()
            .map(|generics| ConcreteTargetPath {
                target: self.target.clone(),
                generics,
                turbofish: self.turbofish,
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T: GenericsArgs> From<(PathWithoutGenerics, T)> for ConcreteTargetPathWithGenericsCollection {
    fn from(value: (PathWithoutGenerics, T)) -> Self {
        let (target, args) = value;
        Self::from_args(target, &args)
    }
}

impl<T: GenericsArgs> TryFrom<(Path, T)> for ConcreteTargetPathWithGenericsCollection {
    type Error = TryFromPathWithoutGenericsError;
    fn try_from(value: (Path, T)) -> Result<Self, Self::Error> {
        let (target, args) = value;
        Ok(Self::from((target.try_into()?, args)))
    }
}

impl<T: GenericsArgs> From<WithTargetPath<T>> for ConcreteTargetPathWithGenericsCollection {
    fn from(value: WithTargetPath<T>) -> Self {
        let (target, args) = value.into();
        Self::from((target, args))
    }
}

pub fn resolve_paths_from_item_or_args<'a, T, A>(
    item: &'a Item,
    args: &A,
) -> syn::Result<ConcreteTargetPathWithGenericsCollection>
where
    T: IdentGenericsAttrsMeta<'a>,
    A: ItemAttributeArgs + FromMeta + GenericsArgs,
{
    let item_meta = T::try_from(item)?;
    validate_generic_counts(item_meta.generics(), args)?;
    Ok(ConcreteTargetPathWithGenericsCollection::from_args(
        item_meta.ident().into(),
        args,
    ))
}

/// Build the concrete paths for an item from an attribute with generics
pub fn resolve_paths_from_attribute_and_item<'a, A, TIRM, R, M>(
    item_meta: TIRM,
    attr: &Attribute,
) -> syn::Result<ConcreteTargetPathWithGenericsCollection>
where
    A: ItemAttributeArgs + FromMeta + GenericsArgs,
    TIRM: TryInto<R, Error = syn::Error>,
    R: AsRef<M>,
    M: IdentGenericsAttrsMeta<'a>,
{
    let args = A::from_meta_ext(&attr.meta).map_err(syn::Error::from)?;
    let item_meta = item_meta.try_into()?;
    let item_meta = item_meta.as_ref();
    validate_generic_counts(item_meta.generics(), &args)?;
    Ok(ConcreteTargetPathWithGenericsCollection::from_args(
        item_meta.ident().into(),
        &args,
    ))
}

pub fn validate_generic_counts<T>(generics: &syn::Generics, cg: &T) -> syn::Result<()>
where
    T: CountGenerics,
{
    let expected_generics_count = generics.type_params().count();
    if expected_generics_count > 0 {
        let count = cg.count_generics();
        if count != expected_generics_count {
            return Err(syn::Error::new(
                cg.get_span(),
                format!("expected {expected_generics_count} generic parameters, found {count}"),
            ));
        }
    }
    Ok(())
}

pub fn generics_from_path(path: &Path) -> syn::Result<TypeList> {
    let mut generics = TypeList::empty();
    for segment in &path.segments {
        if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
            let type_list = parse2::<TypeList>(angle_bracketed.args.to_token_stream())?;
            generics.0.extend(type_list.0);
        }
    }
    Ok(generics)
}

pub fn legacy_generics_from_path(
    struct_or_enum_ref: &StructOrEnumMeta,
    attr: MacroStream,
) -> syn::Result<TypeList> {
    let path = parse2::<Path>(attr)?;
    let path_last_segment = path.segments.last();
    let path_maybe_ident = path_last_segment.map(|s| &s.ident);
    if Some(struct_or_enum_ref.ident) != path_maybe_ident {
        use syn::spanned::Spanned;
        return Err(syn::Error::new(
            path.span(),
            format!(
                "path ident {} does not match struct or enum ident {:?}",
                struct_or_enum_ref.ident, path_maybe_ident
            ),
        ));
    }
    validate_generic_counts(struct_or_enum_ref.generics, &path)?;
    PathExt::generics(&path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::generics::Generics;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_generics_from_path() -> Result<(), syn::Error> {
        let item = parse2::<Path>(quote! {
            foo::bar::<u32, i32>
        })?;
        let generics = generics_from_path(&item).expect("no generics");
        let generics = quote! { #generics };
        assert_eq!("u32 , i32", generics.to_string());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_legacy_generics_from_path() -> Result<(), syn::Error> {
        let item = parse_quote! {
            #[auto_register_types(Foo<T, U>)]
            struct Foo<T, U>(T, U);
        };
        let attribute = quote! {
            Foo<T, U>
        };
        let struct_or_enum_ref = StructOrEnumMeta::try_from(&item)?;
        let generics = legacy_generics_from_path(&struct_or_enum_ref, attribute)?;
        assert_eq!("T , U", generics.to_token_stream().to_string().trim());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_concrete_target_path_to_tokens() -> syn::Result<()> {
        assert_eq!(
            ConcreteTargetPath {
                target: parse2::<PathWithoutGenerics>(quote! { Foo })?,
                generics: Generics(parse2::<TypeList>(quote! {})?),
                turbofish: false,
            }
            .to_token_stream()
            .to_string(),
            quote! { Foo }.to_string()
        );
        assert_eq!(
            ConcreteTargetPath {
                target: parse2::<PathWithoutGenerics>(quote! { Foo })?,
                generics: Generics(parse2::<TypeList>(quote! { u8 })?),
                turbofish: false,
            }
            .to_token_stream()
            .to_string(),
            quote! { Foo<u8> }.to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_concrete_target_path_to_tokens_turbofish() -> syn::Result<()> {
        assert_eq!(
            ConcreteTargetPath {
                target: parse2::<PathWithoutGenerics>(quote! { Foo })?,
                generics: Generics(parse2::<TypeList>(quote! {})?),
                turbofish: true,
            }
            .to_token_stream()
            .to_string(),
            quote! { Foo }.to_string()
        );
        assert_eq!(
            ConcreteTargetPath {
                target: parse2::<PathWithoutGenerics>(quote! { Foo })?,
                generics: Generics(parse2::<TypeList>(quote! { u8 })?),
                turbofish: true,
            }
            .to_token_stream()
            .to_string(),
            quote! { Foo::<u8> }.to_string()
        );
        Ok(())
    }
}
