use crate::__private::attribute::{AutoPluginShortHandAttribute, ShortHandAttribute};
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::global_args::AutoPluginAttributeKind;
use crate::macro_api::global_args::GenericsArgs;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ObserverArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl GenericsArgs for ObserverArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for ObserverArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::Observer
    }
}

impl<'a> From<&'a ObserverArgs> for RegisterTypeArgs {
    fn from(value: &'a ObserverArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a ObserverArgs> for AddObserverArgs {
    fn from(value: &'a ObserverArgs) -> Self {
        AddObserverArgs {
            generics: value.generics.clone(),
        }
    }
}

impl ArgsBackToTokens for ObserverArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        AddObserverArgs::from(self).back_to_inner_arg_tokens(tokens);
    }
}

impl ShortHandAttribute for ObserverArgs {
    fn expand_args(&self, plugin: &NonEmptyPath) -> MacroStream {
        let mut args = Vec::new();
        args.push(quote! { plugin = #plugin });
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        quote! { #(#args),* }
    }

    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();
        expanded_attrs
            .attrs
            .push(tokens::auto_add_observer(plugin.clone(), self.into()));
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_api::global_args::GlobalArgs;
    use crate::test_util::macros::*;
    use darling::ast::NestedMeta;
    use internal_test_util::vec_spread;
    use quote::ToTokens;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        let args = vec![quote! {}];
        println!("checking args: {}", quote! { #(#args),*});
        assert_vec_args_expand!(plugin!(parse_quote!(Test)), ObserverArgs, args);
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
        )};
        let args = GlobalArgs::<ObserverArgs>::from_nested_meta(&args)?;
        println!(
            "{}",
            args.inner.expand_attrs(&args.plugin()).to_token_stream()
        );
        assert_eq!(
            args.inner
                .expand_attrs(&args.plugin())
                .to_token_stream()
                .to_string(),
            ExpandAttrs {
                use_items: vec![],
                attrs: vec_spread![tokens::auto_add_observer(
                    args.plugin(),
                    (&args.inner).into()
                ),]
            }
            .to_token_stream()
            .to_string()
        );
        Ok(())
    }
}
