use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::item_with_attr_match::{ItemWithAttributeMatch, items_with_attribute_match};
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::global_args::{AutoPluginAttributeKind, GenericsArgs, ItemAttributeArgs};
use crate::syntax::ast::type_list::TypeList;
use crate::util::concrete_path::ConcreteTargetPath;
use crate::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::util::resolve_ident_from_item::{
    IdentFromItemResult, resolve_ident_from_struct_or_enum,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterTypeAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AutoPluginAttributeKind for RegisterTypeAttributeArgs {
    type Attribute = AutoPluginItemAttribute;
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::RegisterType
    }
}

impl ItemAttributeArgs for RegisterTypeAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_auto_plugin_register_type_"
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<'_, Self>>> {
        items_with_attribute_match::<StructOrEnumMeta, RegisterTypeAttributeArgs>(items)
    }
}

impl GenericsArgs for RegisterTypeAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for RegisterTypeAttributeArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        tokens.extend(quote! {
            .register_type :: < #target >()
        })
    }
}

impl ArgsBackToTokens for RegisterTypeAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generics().to_attribute_arg_tokens());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use syn::{Path, parse_quote, parse2};

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<RegisterTypeAttributeArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < FooTarget >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<RegisterTypeAttributeArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < FooTarget<u8, bool> >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args =
            parse2::<RegisterTypeAttributeArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < FooTarget<u8, bool> >()
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < FooTarget<bool, bool> >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
