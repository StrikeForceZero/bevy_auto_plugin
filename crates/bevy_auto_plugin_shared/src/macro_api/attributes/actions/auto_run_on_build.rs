use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_fn};
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct RunOnBuildArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AttributeIdent for RunOnBuildArgs {
    const IDENT: &'static str = "auto_run_on_build";
}

impl ItemAttributeArgs for RunOnBuildArgs {
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_fn(item)
    }
}

impl GenericsArgs for RunOnBuildArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for RunOnBuildArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            #target
        })
    }
}

impl ArgsBackToTokens for RunOnBuildArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut args = vec![];
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        tokens.extend(quote! { #(#args),* });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use syn::{Path, parse_quote, parse2};

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<RunOnBuildArgs>(quote!())?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {

                foo_target
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<RunOnBuildArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                foo_target::<u8, bool>
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args = parse2::<RunOnBuildArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                foo_target::<u8, bool>
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                foo_target::<bool, bool>
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
