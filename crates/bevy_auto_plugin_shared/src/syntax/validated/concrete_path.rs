use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::generics::{Generics, GenericsCollection};
use crate::syntax::validated::path_without_generics::PathWithoutGenerics;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Path, PathArguments, parse2};

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
    use internal_test_proc_macro::xtest;

    #[xtest]
    fn test_generics_from_path() -> Result<(), syn::Error> {
        let item = parse2::<Path>(quote! {
            foo::bar::<u32, i32>
        })?;
        let generics = generics_from_path(&item).expect("no generics");
        let generics = quote! { #generics };
        assert_eq!("u32 , i32", generics.to_string());
        Ok(())
    }

    #[xtest]
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

    #[xtest]
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
