use crate::macro_api::prelude::*;
use proc_macro2::TokenStream as MacroStream;

pub mod action;
pub mod auto_bind_plugin;
pub mod auto_plugin;
pub mod rewrite;

macro_rules! gen_action_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
         $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                action::proc_attribute_outer::<$args>(attr, input)
            }
         )+
    };
}

macro_rules! gen_rewrite_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                rewrite::proc_attribute_rewrite_outer::<$args>(attr, input)
            }
        )+
    };
}

gen_action_outers! {
    auto_run_on_build          => IaRunOnBuild,
    auto_register_type         => IaRegisterType,
    auto_add_message           => IaAddMessage,
    auto_init_resource         => IaInitResource,
    auto_insert_resource       => IaInsertResource,
    auto_init_state            => IaInitState,
    auto_init_sub_state        => IaInitSubState,
    auto_name                  => IaName,
    auto_register_state_type   => IaRegisterStateType,
    auto_add_system            => IaAddSystem,
    auto_add_observer          => IaAddObserver,
    auto_add_plugin            => IaAddPlugin,
    auto_configure_system_set  => IaConfigureSystemSet,
    auto_plugin_custom         => IaAutoPluginCustom,
}

gen_rewrite_outers! {
    auto_component => IaComponent,
    auto_resource  => IaResource,
    auto_system    => IaSystem,
    auto_event     => IaEvent,
    auto_message   => IaMessage,
    auto_observer  => IaObserver,
    auto_states    => IaState,
    auto_sub_states => IaSubState,
}
