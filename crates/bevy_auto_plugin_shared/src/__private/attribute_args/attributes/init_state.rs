use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::{
    GenericsArgs, ItemAttributeArgs, ToTokensWithConcreteTargetPath,
};
use crate::__private::item_with_attr_match::{
    ItemWithAttributeMatch, struct_or_enum_items_with_attribute_macro,
};
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::ConcreteTargetPath;
use crate::__private::util::item::require_struct_or_enum;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitStateAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl ItemAttributeArgs for InitStateAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_global_auto_plugin_init_state_"
    }
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::InitState
    }
    fn resolve_item_ident(item: &Item) -> syn::Result<&Ident> {
        require_struct_or_enum(item)
    }
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<Self>>> {
        struct_or_enum_items_with_attribute_macro::<InitStateAttributeArgs>(items)
    }
}

impl GenericsArgs for InitStateAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for InitStateAttributeArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .init_state::< #target >()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::WithTargetPath;
    use syn::{Path, parse_quote, parse2};

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<InitStateAttributeArgs>(quote!())?;
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

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<InitStateAttributeArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_state :: < FooTarget<u8, bool> > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args =
            parse2::<InitStateAttributeArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_state :: < FooTarget<u8, bool> > ()
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .init_state :: < FooTarget<bool, bool> > ()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
