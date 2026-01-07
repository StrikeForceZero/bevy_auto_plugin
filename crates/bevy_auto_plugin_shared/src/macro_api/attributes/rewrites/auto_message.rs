use crate::{
    codegen::{
        ExpandAttrs,
        tokens,
    },
    macro_api::prelude::*,
    syntax::{
        ast::flag_or_list::FlagOrList,
        validated::non_empty_path::NonEmptyPath,
    },
    util::macros::impl_from_default,
};
use darling::FromMeta;
use proc_macro2::Ident;

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
    fn from(_: &'a MessageArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a MessageArgs> for AddMessageArgs {
    fn from(_: &'a MessageArgs) -> Self {
        Self {}
    }
}

pub type IaMessage =
    ItemAttribute<Composed<MessageArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type MessageAttrExpandEmitter = AttrExpansionEmitter<IaMessage>;

impl AttrExpansionEmitterToExpandAttr for MessageAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs.attrs.push(tokens::derive_message(&self.args.args.base.derive.items));
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            expand_attrs.append(tokens::reflect(&self.args.args.base.reflect.items))
        }
        if self.args.args.base.register {
            expand_attrs.attrs.push(tokens::auto_register_type(self.into()));
        }

        // TODO: should this be gated behind a flag?
        expand_attrs.attrs.push(tokens::auto_add_message(self.into()));
    }
}

impl_from_default!(MessageArgs => (RegisterTypeArgs, AddMessageArgs));
