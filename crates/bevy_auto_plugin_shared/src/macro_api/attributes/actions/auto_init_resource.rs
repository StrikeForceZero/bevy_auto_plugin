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
pub struct InitResourceArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AttributeIdent for InitResourceArgs {
    const IDENT: &'static str = "auto_init_resource";
}

impl ItemAttributeArgs for InitResourceArgs {
    #[cfg(feature = "allow_on_use_statements")]
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        crate::syntax::analysis::item::resolve_ident_from_struct_or_enum_or_use_item(item)
    }
    #[cfg(not(feature = "allow_on_use_statements"))]
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        crate::syntax::analysis::item::resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for InitResourceArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for InitResourceArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .init_resource::< #target >()
        })
    }
}

impl ArgsBackToTokens for InitResourceArgs {
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
        let args = parse2::<InitResourceArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_resource :: < FooTarget > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<InitResourceArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_resource :: < FooTarget<u8, bool> > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args = parse2::<InitResourceArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_resource :: < FooTarget<u8, bool> > ()
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_resource :: < FooTarget<bool, bool> > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
