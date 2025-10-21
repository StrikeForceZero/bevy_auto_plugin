use crate::__private::attribute::RewriteAttribute;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{AttributeIdent, ItemAttribute};
use crate::macro_api::prelude::*;
use crate::macro_api::schedule_config::ScheduleWithScheduleConfigArgs;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct SystemArgs {
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl AttributeIdent for SystemArgs {
    const IDENT: &'static str = "auto_system";
}

impl<'a> From<&'a SystemArgs> for RegisterTypeArgs {
    fn from(value: &'a SystemArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a SystemArgs> for AddSystemArgs {
    fn from(value: &'a SystemArgs) -> Self {
        AddSystemArgs {
            schedule_config: value.schedule_config.clone(),
        }
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

impl ToTokens
    for QQ<'_, ItemAttribute<Composed<SystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}
