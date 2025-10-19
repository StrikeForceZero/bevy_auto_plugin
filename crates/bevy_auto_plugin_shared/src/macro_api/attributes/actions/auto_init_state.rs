use crate::codegen::tokens;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::syntax::analysis::item::IdentFromItemResult;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitStateArgs {}

impl AttributeIdent for InitStateArgs {
    const IDENT: &'static str = "auto_init_state";
}

impl ItemAttributeArgs for InitStateArgs {
    #[cfg(feature = "allow_on_use_statements")]
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        crate::syntax::analysis::item::resolve_ident_from_struct_or_enum_or_use_item(item)
    }
    #[cfg(not(feature = "allow_on_use_statements"))]
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        crate::syntax::analysis::item::resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for InitStateArgs {
    fn type_lists(&self) -> &[TypeList] {
        &[]
    }
}

impl ToTokensWithConcreteTargetPath for InitStateArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .init_state::< #target >()
        })
    }
    fn required_use_statements(&self) -> Vec<syn::ItemUse> {
        vec![tokens::use_bevy_state_app_ext_states()]
    }
}

impl ArgsBackToTokens for InitStateArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generics().to_attribute_arg_tokens());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use internal_test_proc_macro::xtest;
    use syn::{Path, parse_quote, parse2};

    #[xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<InitStateArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_state :: < FooTarget > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
