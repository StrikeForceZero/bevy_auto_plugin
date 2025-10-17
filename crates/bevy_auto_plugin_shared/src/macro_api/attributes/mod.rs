use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::syntax::analysis::item::IdentFromItemResult;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::Ident;
use quote::format_ident;
use std::hash::Hash;
use syn::parse::Parse;
use syn::{Item, parse_quote};

mod actions;
mod auto_plugin;
mod rewrites;
mod traits;

pub mod prelude {
    pub use super::auto_plugin::{
        AutoPluginFnArgs, AutoPluginStructOrEnumArgs, resolve_app_param_name,
    };
    pub use crate::macro_api::attributes::actions::auto_add_message::AddMessageArgs;
    pub use crate::macro_api::attributes::actions::auto_add_observer::AddObserverArgs;
    pub use crate::macro_api::attributes::actions::auto_add_plugin::AddPluginArgs;
    pub use crate::macro_api::attributes::actions::auto_add_system::AddSystemArgs;
    pub use crate::macro_api::attributes::actions::auto_init_resource::InitResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_init_state::InitStateArgs;
    pub use crate::macro_api::attributes::actions::auto_init_sub_state::InitSubStateArgs;
    pub use crate::macro_api::attributes::actions::auto_insert_resource::InsertResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_name::NameArgs;
    pub use crate::macro_api::attributes::actions::auto_register_state_type::RegisterStateTypeArgs;
    pub use crate::macro_api::attributes::actions::auto_register_type::RegisterTypeArgs;
    pub use crate::macro_api::attributes::actions::auto_run_on_build::RunOnBuildArgs;
    pub use crate::macro_api::attributes::rewrites::auto_component::ComponentArgs;
    pub use crate::macro_api::attributes::rewrites::auto_event::EventArgs;
    pub use crate::macro_api::attributes::rewrites::auto_message::MessageArgs;
    pub use crate::macro_api::attributes::rewrites::auto_observer::ObserverArgs;
    pub use crate::macro_api::attributes::rewrites::auto_resource::ResourceArgs;
    pub use crate::macro_api::attributes::rewrites::auto_states::StatesArgs;
    pub use crate::macro_api::attributes::rewrites::auto_system::SystemArgs;
    pub use crate::macro_api::attributes::traits::prelude::*;
}

pub trait AttributeIdent {
    const IDENT: &'static str;
    fn full_attribute_path() -> NonEmptyPath {
        let ident = format_ident!("{}", Self::IDENT);
        parse_quote!( ::bevy_auto_plugin::prelude::#ident )
    }
}

pub trait ItemAttributeArgs:
    AttributeIdent + FromMeta + Parse + ToTokensWithConcreteTargetPath + Hash + Clone
{
    fn global_build_prefix() -> Ident {
        format_ident!("_auto_plugin_{}_", Self::IDENT)
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_>;
}
