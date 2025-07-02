mod prelude;
pub use prelude::*;

#[deprecated(
    since = "0.3.0",
    note = "use bevy_auto_plugin::flat_file::prelude instead"
)]
#[doc(inline)]
pub use crate::flat_file::prelude as auto_plugin;
#[deprecated(
    since = "0.3.0",
    note = "use bevy_auto_plugin::module::prelude instead"
)]
#[doc(inline)]
pub use crate::module::prelude as auto_plugin_module;
