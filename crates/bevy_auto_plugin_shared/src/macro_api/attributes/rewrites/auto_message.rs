use crate::__private::attribute::RewriteAttribute;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::AttributeIdent;
use crate::macro_api::attributes::actions::auto_add_message::AddMessageArgs;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct MessageArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
}

impl AttributeIdent for MessageArgs {
    const IDENT: &'static str = "auto_message";
}

impl<'a> From<&'a MessageArgs> for RegisterTypeArgs {
    fn from(value: &'a MessageArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a MessageArgs> for AddMessageArgs {
    fn from(value: &'a MessageArgs) -> Self {
        Self {}
    }
}

impl RewriteAttribute for MessageArgs {
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

        if self.derive.present {
            expanded_attrs
                .attrs
                .push(tokens::derive_message(&self.derive.items));
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            expanded_attrs.append(tokens::reflect(&self.reflect.items))
        }
        if self.register {
            expanded_attrs
                .attrs
                .push(tokens::auto_register_type(plugin.clone(), self.into()));
        }

        // TODO: should this be gated behind a flag?
        expanded_attrs
            .attrs
            .push(tokens::auto_add_message(plugin.clone(), self.into()));

        expanded_attrs
    }
}
