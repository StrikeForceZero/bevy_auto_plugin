use crate::__private::attribute_args::attributes::prelude::{
    AddObserverAttributeArgs, RegisterTypeAttributeArgs,
};
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::attributes::shorthand::{
    AutoPluginShortHandAttribute, ExpandAttrs, ShortHandAttribute, tokens,
};
use crate::__private::attribute_args::{AutoPluginAttributeKind, GenericsArgs};
use crate::__private::non_empty_path::NonEmptyPath;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ObserverAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl GenericsArgs for ObserverAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for ObserverAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::Observer
    }
}

impl<'a> From<&'a ObserverAttributeArgs> for RegisterTypeAttributeArgs {
    fn from(value: &'a ObserverAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a ObserverAttributeArgs> for AddObserverAttributeArgs {
    fn from(value: &'a ObserverAttributeArgs) -> Self {
        AddObserverAttributeArgs {
            generics: value.generics.clone(),
        }
    }
}

impl ArgsBackToTokens for ObserverAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        AddObserverAttributeArgs::from(self).back_to_inner_arg_tokens(tokens);
    }
}

impl ShortHandAttribute for ObserverAttributeArgs {
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
    use crate::__private::attribute_args::GlobalArgs;
    use crate::test_util::macros::*;
    use darling::ast::NestedMeta;
    use internal_test_util::vec_spread;
    use quote::ToTokens;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        let args = vec![quote! {}];
        println!("checking args: {}", quote! { #(#args),*});
        assert_vec_args_expand!(plugin!(parse_quote!(Test)), ObserverAttributeArgs, args);
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
        )};
        let args = GlobalArgs::<ObserverAttributeArgs>::from_nested_meta(&args)?;
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
