use crate::{
    codegen::{
        ExpandAttrs,
        tokens,
    },
    macro_api::prelude::*,
    syntax::{
        ast::{
            flag_or_list::FlagOrList,
            flag_or_lit::FlagOrLit,
        },
        validated::non_empty_path::NonEmptyPath,
    },
    util::macros::impl_from_default,
};
use darling::FromMeta;
use proc_macro2::Ident;
use syn::parse_quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct SceneComponentArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub auto_name: FlagOrLit,
}

impl AttributeIdent for SceneComponentArgs {
    const IDENT: &'static str = "auto_scene_component";
}

impl From<&SceneComponentArgs> for RegisterTypeArgs {
    fn from(_: &SceneComponentArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a SceneComponentArgs> for NameArgs {
    fn from(value: &'a SceneComponentArgs) -> Self {
        Self { name: value.auto_name.lit.clone() }
    }
}

pub type IaSceneComponent = ItemAttribute<
    Composed<SceneComponentArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type SceneComponentAttrExpandEmitter = AttrExpansionEmitter<IaSceneComponent>;

impl AttrExpansionEmitterToExpandAttr for SceneComponentAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs
                .attrs
                .push(tokens::derive_scene_component(&self.args.args.base.derive.items));
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Component);
            let items =
                std::iter::once(&component_ident).chain(self.args.args.base.reflect.items.iter());
            expand_attrs.append(tokens::reflect(items))
        }
        if self.args.args.base.register {
            expand_attrs.attrs.push(tokens::auto_register_type(self.into()));
        }
        if self.args.args.base.auto_name.present {
            expand_attrs.attrs.push(tokens::auto_name(self.into()));
        }
    }
}

impl_from_default!(SceneComponentArgs => (RegisterTypeArgs, NameArgs));
