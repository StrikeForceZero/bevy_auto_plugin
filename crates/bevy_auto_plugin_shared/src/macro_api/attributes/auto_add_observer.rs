use crate::__private::attribute::AutoPluginItemAttribute;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::global_args::{AutoPluginAttributeKind, GenericsArgs, ItemAttributeArgs};
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use crate::util::resolve_ident_from_item::{IdentFromItemResult, resolve_ident_from_fn};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddObserverAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AutoPluginAttributeKind for AddObserverAttributeArgs {
    type Attribute = AutoPluginItemAttribute;
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::AddObserver
    }
}

impl ItemAttributeArgs for AddObserverAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_auto_plugin_add_observer_"
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_fn(item)
    }
}

impl GenericsArgs for AddObserverAttributeArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for AddObserverAttributeArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .add_observer(#target)
        })
    }
}

impl ArgsBackToTokens for AddObserverAttributeArgs {
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
        let args = parse2::<AddObserverAttributeArgs>(quote!())?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {

                .add_observer(foo_target)
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<AddObserverAttributeArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_observer(foo_target::<u8, bool>)
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args =
            parse2::<AddObserverAttributeArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(foo_target);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_observer(foo_target::<u8, bool>)
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_observer(foo_target::<bool, bool>)
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
