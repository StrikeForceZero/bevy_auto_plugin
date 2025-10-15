use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_struct_or_enum};
use crate::syntax::ast::any_expr::AnyExprCallClosureMacroPath;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddPluginArgs {
    #[darling(default)]
    pub generics: Option<TypeList>,
    #[darling(default)]
    pub value: Option<AnyExprCallClosureMacroPath>,
}

impl AttributeIdent for AddPluginArgs {
    const IDENT: &'static str = "auto_add_plugin";
}

impl ItemAttributeArgs for AddPluginArgs {
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for AddPluginArgs {
    fn type_lists(&self) -> &[TypeList] {
        self.generics.as_slice()
    }
}

impl ToTokensWithConcreteTargetPath for AddPluginArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        if let Some(value) = &self.value {
            tokens.extend(quote! {
                .add_plugins(#value)
            })
        } else {
            tokens.extend(quote! {
                .add_plugins(#target ::default())
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use internal_test_proc_macro::xtest;
    use syn::{Path, parse_quote, parse2};

    #[xtest]
    fn test_to_tokens_no_generics_no_plugin() -> syn::Result<()> {
        let args = parse2::<AddPluginArgs>(quote!())?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_plugins(FooTarget :: default ())
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<AddPluginArgs>(quote!(value(FooTarget)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_plugins(FooTarget)
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args = parse2::<AddPluginArgs>(quote!(generics(u8, bool), value(FooTarget(1, true))))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .add_plugins(FooTarget(1, true))
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[xtest]
    #[should_panic(expected = "Duplicate field `generics`")]
    fn test_to_tokens_multiple() {
        parse2::<AddPluginArgs>(quote!(
            generics(u8, bool),
            generics(bool, bool),
            plugin(FooTarget(1, true))
        ))
        .unwrap();
    }
}
