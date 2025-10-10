use crate::__private::attribute::RewriteAttribute;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::AttributeIdent;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::schedule_config::ScheduleWithScheduleConfigArgs;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::quote;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct SystemArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl GenericsArgs for SystemArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AttributeIdent for SystemArgs {
    const IDENT: &'static str = "auto_system";
}

impl<'a> From<&'a SystemArgs> for RegisterTypeArgs {
    fn from(value: &'a SystemArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a SystemArgs> for AddSystemArgs {
    fn from(value: &'a SystemArgs) -> Self {
        AddSystemArgs {
            generics: value.generics.clone(),
            schedule_config: value.schedule_config.clone(),
        }
    }
}

impl ArgsBackToTokens for SystemArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut items = vec![];
        items.extend(self.generics().to_attribute_arg_vec_tokens());
        items.extend(self.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! { #(#items),* });
    }
}

impl RewriteAttribute for SystemArgs {
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
            .push(tokens::auto_add_systems(plugin.clone(), self.into()));
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_api::with_plugin::WithPlugin;
    use crate::test_util::macros::*;
    use darling::ast::NestedMeta;
    use internal_test_proc_macro::xtest;
    use internal_test_util::vec_spread;
    use quote::ToTokens;
    use syn::parse_quote;

    #[xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        let args = vec![quote! { schedule = Update }];
        println!("checking args: {}", quote! { #(#args),*});
        assert_vec_args_expand!(plugin!(parse_quote!(Test)), SystemArgs, args);
        Ok(())
    }

    #[xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
            schedule = Update,
        )};
        let args = WithPlugin::<SystemArgs>::from_nested_meta(&args)?;
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
                attrs: vec_spread![tokens::auto_add_systems(
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
