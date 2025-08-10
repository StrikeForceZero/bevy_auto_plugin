use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::{
    GenericsArgs, ItemAttributeArgs, ToTokensWithConcreteTargetPath,
};
use crate::__private::item_with_attr_match::{ItemWithAttributeMatch, items_with_attribute_macro};
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::ConcreteTargetPath;
use crate::__private::util::item::require_fn;
use crate::__private::util::meta::fn_meta::FnMeta;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddObserverAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl ItemAttributeArgs for AddObserverAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_global_auto_plugin_add_observer_"
    }
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::AddObserver
    }
    fn resolve_item_ident(item: &Item) -> syn::Result<&Ident> {
        require_fn(item)
    }

    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<Self>>> {
        items_with_attribute_macro::<FnMeta, AddObserverAttributeArgs>(items)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::WithTargetPath;
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
