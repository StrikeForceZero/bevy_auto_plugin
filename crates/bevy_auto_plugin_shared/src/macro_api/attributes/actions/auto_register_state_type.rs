use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::macro_api::with_plugin::GenericsArgs;
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_struct_or_enum};
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterStateTypeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl AttributeIdent for RegisterStateTypeArgs {
    const IDENT: &'static str = "auto_register_state_type";
}

impl ItemAttributeArgs for RegisterStateTypeArgs {
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for RegisterStateTypeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for RegisterStateTypeArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        let bevy_state = crate::__private::paths::state::root_path();
        tokens.extend(quote! {
            .register_type :: < #bevy_state::prelude::State< #target > >()
            .register_type :: < #bevy_state::prelude::NextState< #target > >()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use syn::{Path, parse_quote, parse2};

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<RegisterStateTypeArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_state = crate::__private::paths::state::root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < #bevy_state::prelude::State< FooTarget > >()
                .register_type :: < #bevy_state::prelude::NextState< FooTarget > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<RegisterStateTypeArgs>(quote!(generics(u8, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_state = crate::__private::paths::state::root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < #bevy_state::prelude::State< FooTarget<u8, bool> > >()
                .register_type :: < #bevy_state::prelude::NextState< FooTarget<u8, bool> > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_multiple() -> syn::Result<()> {
        let args =
            parse2::<RegisterStateTypeArgs>(quote!(generics(u8, bool), generics(bool, bool)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let bevy_state = crate::__private::paths::state::root_path();
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < #bevy_state::prelude::State< FooTarget<u8, bool> > >()
                .register_type :: < #bevy_state::prelude::NextState< FooTarget<u8, bool> > >()
            }
            .to_string()
        );
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .register_type :: < #bevy_state::prelude::State< FooTarget<bool, bool> > >()
                .register_type :: < #bevy_state::prelude::NextState< FooTarget<bool, bool> > >()
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }
}
