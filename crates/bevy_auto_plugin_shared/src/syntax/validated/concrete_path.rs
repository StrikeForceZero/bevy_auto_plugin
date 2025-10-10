use crate::codegen::with_target_path::WithTargetPath;
use crate::macro_api::global_args::GenericsArgs;
use crate::macro_api::global_args::ItemAttributeArgs;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::traits::generics::CountGenerics;
use crate::syntax::validated::generics::{Generics, GenericsCollection};
use crate::syntax::validated::path_without_generics::{
    PathWithoutGenerics, TryFromPathWithoutGenericsError,
};
use crate::util::extensions::from_meta::FromMetaExt;
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

pub fn validate_generic_counts<T>(generics: &syn::Generics, cg: &T) -> syn::Result<()>
where
    T: CountGenerics,
{
    let expected_generics_count = generics.type_params().count();
    if expected_generics_count > 0 {
        let count = cg.count_generics()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::validated::generics::Generics;

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
