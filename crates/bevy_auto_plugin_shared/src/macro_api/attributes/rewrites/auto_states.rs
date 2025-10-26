use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use crate::util::macros::impl_from_default;
use darling::FromMeta;
use proc_macro2::Ident;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StatesArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl AttributeIdent for StatesArgs {
    const IDENT: &'static str = "auto_states";
}

impl<'a> From<&'a StatesArgs> for RegisterTypeArgs {
    fn from(_value: &'a StatesArgs) -> Self {
        Self::default()
    }
}

impl<'a> From<&'a StatesArgs> for InitStateArgs {
    fn from(_value: &'a StatesArgs) -> Self {
        Self::default()
    }
}
pub type IaState =
    ItemAttribute<Composed<StatesArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type RewriteQState = AttrExpansionEmitter<IaState>;
impl AttrExpansionEmitterToExpandAttr for RewriteQState {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs.append(tokens::derive_states(&self.args.args.base.derive.items));
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
        if self.args.args.base.init {
            expand_attrs
                .attrs
                .push(tokens::auto_init_states(self.into()));
        }
    }
}

impl_from_default!(StatesArgs => (RegisterTypeArgs, InitStateArgs));
