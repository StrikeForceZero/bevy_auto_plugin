use crate::{
    codegen::{
        ExpandAttrs,
        tokens,
    },
    macro_api::prelude::*,
    syntax::{
        ast::{
            any_expr::AnyExprCallClosureMacroPath,
            any_expr_list::AnyExprList,
            flag_or_list::FlagOrList,
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
pub struct ResourceArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
    pub insert: Option<AnyExprList<AnyExprCallClosureMacroPath>>,
}

impl AttributeIdent for ResourceArgs {
    const IDENT: &'static str = "auto_resource";
}

impl<'a> From<&'a ResourceArgs> for RegisterTypeArgs {
    fn from(_: &'a ResourceArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a ResourceArgs> for InitResourceArgs {
    fn from(_: &'a ResourceArgs) -> Self {
        Self::default()
    }
}

pub type IaResource =
    ItemAttribute<Composed<ResourceArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type ResourceAttrExpandEmitter = AttrExpansionEmitter<IaResource>;
impl AttrExpansionEmitterToExpandAttr for ResourceAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs.attrs.push(tokens::derive_resource(&self.args.args.base.derive.items));
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Resource);
            let items =
                std::iter::once(&component_ident).chain(self.args.args.base.reflect.items.iter());
            expand_attrs.append(tokens::reflect(items))
        }
        if self.args.args.base.register {
            expand_attrs.attrs.push(tokens::auto_register_type(self.into()));
        }
        if self.args.args.base.init {
            expand_attrs.attrs.push(tokens::auto_init_resource(self.into()));
        }
        if let Some(insert) = &self.args.args.base.insert {
            for item in insert.iter() {
                let ia_insert_resource: IaInsertResource =
                    self.args.clone().into_zero_or_one_generic_target::<InsertResourceArgs>(
                        |_| InsertResourceArgs::from_insert(item.clone()),
                        |_| {
                            // TODO: if we wanted to actually use the generics passed
                            //  we could store an iterator before the for loop and call .next()
                            Default::default()
                        },
                    );
                let ia_insert_resource_emitter = AttrEmitter::from_args(ia_insert_resource);
                expand_attrs.attrs.push(tokens::auto_insert_resource(ia_insert_resource_emitter));
            }
        }
    }
}

impl_from_default!(ResourceArgs => (RegisterTypeArgs, InitResourceArgs, InsertResourceArgs));
