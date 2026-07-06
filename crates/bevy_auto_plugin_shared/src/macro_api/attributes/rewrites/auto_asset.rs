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
pub struct AssetArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl AttributeIdent for AssetArgs {
    const IDENT: &'static str = "auto_asset";
}

impl<'a> From<&'a AssetArgs> for InitAssetArgs {
    fn from(_: &'a AssetArgs) -> Self {
        Self::default()
    }
}

impl<'a> From<&'a AssetArgs> for RegisterAssetReflectArgs {
    fn from(_: &'a AssetArgs) -> Self {
        Self::default()
    }
}

pub type IaAsset =
    ItemAttribute<Composed<AssetArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type AssetAttrExpandEmitter = AttrExpansionEmitter<IaAsset>;

impl AttrExpansionEmitterToExpandAttr for AssetAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            let derive_tokens = if self.args.args.base.reflect.present {
                tokens::derive_asset_without_type_path(&self.args.args.base.derive.items)
            } else {
                tokens::derive_asset(&self.args.args.base.derive.items)
            };
            expand_attrs.attrs.push(derive_tokens);
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            expand_attrs.append(tokens::reflect(&self.args.args.base.reflect.items))
        }
        if self.args.args.base.register {
            expand_attrs.attrs.push(tokens::auto_register_asset_reflect(self.into()));
        }
        if self.args.args.base.init {
            expand_attrs.attrs.push(tokens::auto_init_asset(self.into()));
        }
    }
}

impl_from_default!(AssetArgs => (InitAssetArgs, RegisterAssetReflectArgs));
