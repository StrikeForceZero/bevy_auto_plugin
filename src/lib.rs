pub mod prelude;

#[deprecated(since = "0.3.0", note = "use bevy_auto_plugin::prelude::inline instead")]
#[doc(inline)]
pub use bevy_auto_plugin_nightly_proc_macros as auto_plugin;
#[deprecated(since = "0.3.0", note = "use bevy_auto_plugin::prelude::module instead")]
#[doc(inline)]
pub use bevy_auto_plugin_proc_macros as auto_plugin_module;
