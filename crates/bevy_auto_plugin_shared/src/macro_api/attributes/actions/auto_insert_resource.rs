use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::macro_api::global_args::GenericsArgs;
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_struct_or_enum};
use crate::syntax::ast::any_expr::AnyExprCallClosureMacroPath;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct InsertResourceArgs {
    #[darling(default)]
    pub generics: Option<TypeList>,
    pub resource: AnyExprCallClosureMacroPath,
}

impl AttributeIdent for InsertResourceArgs {
    const IDENT: &'static str = "auto_insert_resource";
}

impl ItemAttributeArgs for InsertResourceArgs {
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for InsertResourceArgs {
    fn type_lists(&self) -> &[TypeList] {
        self.generics.as_slice()
    }
}

impl ToTokensWithConcreteTargetPath for InsertResourceArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        let resource = &self.resource;
        tokens.extend(quote! {
            .insert_resource::< #target >(#resource)
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
        let args = parse2::<InsertResourceArgs>(quote!(resource(FooTarget)))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .insert_resource :: < FooTarget > (FooTarget)
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_single() -> syn::Result<()> {
        let args =
            parse2::<InsertResourceArgs>(quote!(generics(u8, bool), resource(FooTarget(1, true))))?;
        let path: Path = parse_quote!(FooTarget);
        let args_with_target = WithTargetPath::try_from((path, args))?;
        let mut token_iter = args_with_target.to_tokens_iter();
        assert_eq!(
            token_iter.next().expect("token_iter").to_string(),
            quote! {
                .insert_resource :: < FooTarget<u8, bool> > (FooTarget(1, true))
            }
            .to_string()
        );
        assert!(token_iter.next().is_none());
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    #[should_panic(expected = "Duplicate field `generics`")]
    fn test_to_tokens_multiple() {
        parse2::<InsertResourceArgs>(quote!(
            generics(u8, bool),
            generics(bool, bool),
            resource(FooTarget(1, true))
        ))
        .unwrap();
    }
}
