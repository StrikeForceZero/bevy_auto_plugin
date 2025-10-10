use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::global_args::{AttributeIdent, GenericsArgs, ItemAttributeArgs};
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_struct_or_enum};
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct NameArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AttributeIdent for NameArgs {
    const IDENT: &'static str = "auto_name";
}

impl ItemAttributeArgs for NameArgs {
    fn global_build_prefix() -> &'static str {
        "_auto_plugin_auto_name__"
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for NameArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for NameArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        // TODO: move to util fn
        let name = quote!(#target)
            .to_string()
            .replace(" < ", "<")
            .replace(" >", ">")
            .replace(" ,", ",");
        // TODO: offer option to only remove all spaces?
        //  .replace(" ", "")
        let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
        tokens.extend(quote! {
            .register_required_components_with::<#target, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new(#name))
        })
    }
}

impl ArgsBackToTokens for NameArgs {
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
        let args = parse2::<NameArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_required_components_with::<FooTarget, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new("FooTarget"))
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<NameArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_required_components_with::<FooTarget<u8, bool>, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new("FooTarget<u8, bool>"))
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args = parse2::<NameArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_required_components_with::<FooTarget<u8, bool>, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new("FooTarget<u8, bool>"))
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_required_components_with::<FooTarget<bool, bool>, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new("FooTarget<bool, bool>"))
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
