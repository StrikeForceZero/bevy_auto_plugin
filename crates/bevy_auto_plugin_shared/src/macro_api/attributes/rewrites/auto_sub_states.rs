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
pub struct SubStatesArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl AttributeIdent for SubStatesArgs {
    const IDENT: &'static str = "auto_sub_states";
}

impl<'a> From<&'a SubStatesArgs> for RegisterTypeArgs {
    fn from(_value: &'a SubStatesArgs) -> Self {
        Self::default()
    }
}

impl<'a> From<&'a SubStatesArgs> for RegisterStateTypeArgs {
    fn from(_value: &'a SubStatesArgs) -> Self {
        Self::default()
    }
}

impl<'a> From<&'a SubStatesArgs> for InitSubStateArgs {
    fn from(_value: &'a SubStatesArgs) -> Self {
        Self::default()
    }
}

pub type IaSubState =
    ItemAttribute<Composed<SubStatesArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;

pub type SubStateAttrExpandEmitter = AttrExpansionEmitter<IaSubState>;

impl AttrExpansionEmitterToExpandAttr for SubStateAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs.append(tokens::derive_sub_states(&self.args.args.base.derive.items));
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            expand_attrs.append(tokens::reflect(&self.args.args.base.reflect.items))
        }
        if self.args.args.base.register {
            expand_attrs.attrs.push(tokens::auto_register_type(self.into()));
            expand_attrs.attrs.push(tokens::auto_register_state_type(self.into()));
        }
        if self.args.args.base.init {
            expand_attrs.attrs.push(tokens::auto_init_sub_states(self.into()));
        }
    }
}

impl_from_default!(SubStatesArgs => (RegisterTypeArgs, RegisterStateTypeArgs, InitSubStateArgs));
