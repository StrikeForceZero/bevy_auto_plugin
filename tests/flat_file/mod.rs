mod auto_add_event;
mod auto_add_event_generic;
mod auto_add_observer;
mod auto_add_systems;
mod auto_add_systems_complex_with_generics;
mod auto_add_systems_with_generics;
mod auto_add_systems_with_set;
mod auto_init_resource;
mod auto_init_resource_generic;
mod auto_init_state;
mod auto_insert_resource;
mod auto_insert_resource_with_generics;
mod auto_name;
mod auto_name_with_generics;
mod auto_plugin_default_param;
mod auto_plugin_default_param_method;
mod auto_plugin_multiple_param;
mod auto_plugin_param;
mod auto_register_state_type;
mod auto_register_type;
mod auto_register_type_generic;
#[cfg(feature = "legacy_path_param")]
mod auto_register_type_generic_legacy;
#[cfg(not(wasm))]
mod ui_tests;
