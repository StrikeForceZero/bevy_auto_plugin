use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::{
    GenericsArgs, ItemAttributeArgs, ToTokensWithConcreteTargetPath,
};
use crate::__private::expr_value::ExprValue;
use crate::__private::item_with_attr_match::{
    ItemWithAttributeMatch, struct_or_enum_items_with_attribute_macro,
};
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::ConcreteTargetPath;
use crate::__private::util::item::require_struct_or_enum;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Expr, Item};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct InsertResourceAttributeArgs {
    #[darling(default)]
    pub generics: Option<TypeList>,
    pub resource: ExprValue,
}

impl InsertResourceAttributeArgs {
    pub fn validate_resource(&self) -> syn::Result<()> {
        if !matches!(
            self.resource.0,
            Expr::Call(_) // Foo(_)  or Foo::Bar(_)
            | Expr::Path(_) // Foo or Foo::Bar
            | Expr::Struct(_) // Foo { .. } or Foo::Bar { .. }
        ) {
            return Err(syn::Error::new(
                self.resource.span(),
                "Expected a struct or enum value",
            ));
        }
        Ok(())
    }
}

impl ItemAttributeArgs for InsertResourceAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_global_auto_plugin_insert_resource_"
    }
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::InsertResource
    }
    fn resolve_item_ident(item: &Item) -> syn::Result<&Ident> {
        require_struct_or_enum(item)
    }
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<Self>>> {
        struct_or_enum_items_with_attribute_macro::<InsertResourceAttributeArgs>(items)
    }
}

impl GenericsArgs for InsertResourceAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        self.generics.as_slice()
    }
}

impl ToTokensWithConcreteTargetPath for InsertResourceAttributeArgs {
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
    use crate::__private::attribute_args::WithTargetPath;
    use syn::{Path, parse_quote, parse2};

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens_no_generics() -> syn::Result<()> {
        let args = parse2::<InsertResourceAttributeArgs>(quote!(resource(FooTarget)))?;
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
        let args = parse2::<InsertResourceAttributeArgs>(quote!(
            generics(u8, bool),
            resource(FooTarget(1, true))
        ))?;
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
        parse2::<InsertResourceAttributeArgs>(quote!(
            generics(u8, bool),
            generics(bool, bool),
            resource(FooTarget(1, true))
        ))
        .unwrap();
    }
}
