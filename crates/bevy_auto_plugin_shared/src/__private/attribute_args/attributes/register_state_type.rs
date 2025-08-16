use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::{
    GenericsArgs, ItemAttributeArgs, ToTokensWithConcreteTargetPath,
};
use crate::__private::item_with_attr_match::{ItemWithAttributeMatch, items_with_attribute_match};
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::ConcreteTargetPath;
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::__private::util::resolve_ident_from_item::{
    IdentFromItemResult, resolve_ident_from_struct_or_enum,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterStateTypeAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl ItemAttributeArgs for RegisterStateTypeAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_global_auto_plugin_register_state_type_"
    }
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::RegisterStateType
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<'_, Self>>> {
        items_with_attribute_match::<StructOrEnumMeta, RegisterStateTypeAttributeArgs>(items)
    }
}

impl GenericsArgs for RegisterStateTypeAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for RegisterStateTypeAttributeArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::State< #target > >()
            .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::NextState< #target > >()
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
        let args = parse2::<RegisterStateTypeAttributeArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::State< FooTarget > >()
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::NextState< FooTarget > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<RegisterStateTypeAttributeArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::State< FooTarget<u8, bool> > >()
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::NextState< FooTarget<u8, bool> > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args = parse2::<RegisterStateTypeAttributeArgs>(quote!(
            generics(u8, bool),
            generics(bool, bool)
        ))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::State< FooTarget<u8, bool> > >()
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::NextState< FooTarget<u8, bool> > >()
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::State< FooTarget<bool, bool> > >()
                .register_type :: < ::bevy_auto_plugin::__private::shared::__private::bevy_state::prelude::NextState< FooTarget<bool, bool> > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
