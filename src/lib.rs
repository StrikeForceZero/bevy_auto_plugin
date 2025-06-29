mod prelude;
pub use prelude::*;

#[deprecated(since = "0.3.0", note = "use bevy_auto_plugin::inline::prelude instead")]
#[doc(inline)]
pub use bevy_auto_plugin_nightly_proc_macros as auto_plugin;
#[deprecated(since = "0.3.0", note = "use bevy_auto_plugin::module::prelude instead")]
#[doc(inline)]
pub use bevy_auto_plugin_proc_macros as auto_plugin_module;
