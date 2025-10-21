use crate::macro_api::attributes::AttributeIdent;
use crate::macro_api::schedule_config::ScheduleWithScheduleConfigArgs;
use darling::FromMeta;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AddSystemArgs {
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl AttributeIdent for AddSystemArgs {
    const IDENT: &'static str = "auto_add_system";
}
