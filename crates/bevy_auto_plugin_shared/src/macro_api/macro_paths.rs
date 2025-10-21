use crate::macro_api::attributes::prelude::*;
use crate::macro_api::context::Context;
use syn::parse_quote;

#[derive(Debug, Clone)]
pub struct MacroPaths {
    /// resolved absolute path to `auto_add_system`
    pub emit_add_system_macro: syn::Path,
    /// resolved absolute path to `auto_add_message`
    pub emit_add_message_macro: syn::Path,
    /// resolved absolute path to `auto_add_observer`
    pub emit_add_observer_macro: syn::Path,
    /// resolved absolute path to `auto_add_plugin`
    pub emit_add_plugin_macro: syn::Path,
    /// resolved absolute path to `auto_init_resource`
    pub emit_init_resource_macro: syn::Path,
    /// resolved absolute path to `auto_init_state`
    pub emit_init_state_macro: syn::Path,
    /// resolved absolute path to `auto_init_sub_state`
    pub emit_init_sub_state_macro: syn::Path,
    /// resolved absolute path to `auto_insert_resource`
    pub emit_insert_resource_macro: syn::Path,
    /// resolved absolute path to `auto_register_state_type`
    pub emit_register_state_type_macro: syn::Path,
    /// resolved absolute path to `auto_register_type`
    pub emit_register_type_macro: syn::Path,
    /// resolved absolute path to `auto_run_on_build`
    pub emit_run_on_build_macro: syn::Path,
}

impl Default for MacroPaths {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            emit_add_system_macro:          parse_quote!(  ::bevy_auto_plugin::prelude::auto_add_system            ),
            emit_add_message_macro:         parse_quote!(  ::bevy_auto_plugin::prelude::auto_add_message           ),
            emit_add_observer_macro:        parse_quote!(  ::bevy_auto_plugin::prelude::auto_add_observer          ),
            emit_add_plugin_macro:          parse_quote!(  ::bevy_auto_plugin::prelude::auto_add_plugin            ),
            emit_init_resource_macro:       parse_quote!(  ::bevy_auto_plugin::prelude::auto_init_resource         ),
            emit_init_state_macro:          parse_quote!(  ::bevy_auto_plugin::prelude::auto_init_state            ),
            emit_init_sub_state_macro:      parse_quote!(  ::bevy_auto_plugin::prelude::auto_init_sub_state        ),
            emit_insert_resource_macro:     parse_quote!(  ::bevy_auto_plugin::prelude::auto_insert_resource       ),
            emit_register_state_type_macro: parse_quote!(  ::bevy_auto_plugin::prelude::auto_register_state_type   ),
            emit_register_type_macro:       parse_quote!(  ::bevy_auto_plugin::prelude::auto_register_type         ),
            emit_run_on_build_macro:        parse_quote!(  ::bevy_auto_plugin::prelude::auto_run_on_build          ),
        }
    }
}

pub trait MacroPathProvider {
    fn macro_path(context: &Context) -> &syn::Path;
}

impl MacroPathProvider for AddSystemArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_add_system_macro
    }
}

impl MacroPathProvider for AddMessageArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_add_message_macro
    }
}

impl MacroPathProvider for AddObserverArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_add_observer_macro
    }
}

impl MacroPathProvider for AddPluginArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_add_plugin_macro
    }
}

impl MacroPathProvider for InitResourceArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_init_resource_macro
    }
}

impl MacroPathProvider for InitStateArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_init_state_macro
    }
}

impl MacroPathProvider for InitSubStateArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_init_sub_state_macro
    }
}

impl MacroPathProvider for InsertResourceArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_insert_resource_macro
    }
}

impl MacroPathProvider for RegisterStateTypeArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_register_state_type_macro
    }
}

impl MacroPathProvider for RegisterTypeArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_register_type_macro
    }
}

impl MacroPathProvider for RunOnBuildArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_run_on_build_macro
    }
}
