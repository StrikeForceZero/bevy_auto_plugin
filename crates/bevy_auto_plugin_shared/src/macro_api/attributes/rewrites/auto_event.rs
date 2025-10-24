use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use crate::util::macros::impl_from_default;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
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

pub type IaEvent =
    ItemAttribute<Composed<EventArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type RewriteQEvent = RewriteQ<IaEvent>;
impl RewriteQToExpandAttr for RewriteQEvent {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            if matches!(self.args.args.base.target, EventTarget::Global) {
                expand_attrs
                    .attrs
                    .push(tokens::derive_event(&self.args.args.base.derive.items));
            }
            if matches!(self.args.args.base.target, EventTarget::Entity) {
                expand_attrs.attrs.push(tokens::derive_entity_event(
                    &self.args.args.base.derive.items,
                ));
            }
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            expand_attrs.append(tokens::reflect(&self.args.args.base.reflect.items))
        }
        if self.args.args.base.register {
            expand_attrs
                .attrs
                .push(tokens::auto_register_type(self.into()));
        }
    }
}

impl_from_default!(EventArgs => (RegisterTypeArgs));
