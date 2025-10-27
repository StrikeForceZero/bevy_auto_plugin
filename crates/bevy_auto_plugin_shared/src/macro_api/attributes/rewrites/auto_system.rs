use crate::{
    codegen::{
        ExpandAttrs,
        tokens,
    },
    macro_api::{
        prelude::*,
        schedule_config::ScheduleWithScheduleConfigArgs,
    },
};
use darling::FromMeta;

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
    fn from(_: &'a SystemArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a SystemArgs> for AddSystemArgs {
    fn from(value: &'a SystemArgs) -> Self {
        AddSystemArgs { schedule_config: value.schedule_config.clone() }
    }
}

pub type IaSystem =
    ItemAttribute<Composed<SystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type SystemAttrExpandEmitter = AttrExpansionEmitter<IaSystem>;

impl AttrExpansionEmitterToExpandAttr for SystemAttrExpandEmitter {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        expand_attrs.attrs.push(tokens::auto_add_systems(self.into()));
    }
}

impl From<SystemArgs> for AddSystemArgs {
    fn from(value: SystemArgs) -> Self {
        Self { schedule_config: value.schedule_config }
    }
}
