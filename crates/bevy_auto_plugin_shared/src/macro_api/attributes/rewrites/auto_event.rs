use crate::__private::attribute::RewriteAttribute;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::AttributeIdent;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Default, Debug, Copy, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub enum EventTarget {
    #[default]
    #[darling(rename = "global")]
    Global,
    #[darling(rename = "entity")]
    Entity,
}

impl ToTokens for EventTarget {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            EventTarget::Global => {
                tokens.extend(quote!(global));
            }
            EventTarget::Entity => {
                tokens.extend(quote!(entity));
            }
        }
    }
}

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct EventArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub target: EventTarget,
}

impl AttributeIdent for EventArgs {
    const IDENT: &'static str = "auto_event";
}

impl<'a> From<&'a EventArgs> for RegisterTypeArgs {
    fn from(value: &'a EventArgs) -> Self {
        Self {}
    }
}

impl RewriteAttribute for EventArgs {
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
            if matches!(self.target, EventTarget::Global) {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_event(&self.derive.items));
            }
            if matches!(self.target, EventTarget::Entity) {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_entity_event(&self.derive.items));
            }
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
        expanded_attrs
    }
}
