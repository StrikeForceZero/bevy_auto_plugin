mod prelude;
pub use prelude::*;

#[cfg(feature = "mode_flat_file")]
#[deprecated(
    since = "0.3.0",
    note = "use bevy_auto_plugin::flat_file::prelude instead"
)]
#[doc(inline)]
pub use crate::flat_file::prelude as auto_plugin;

#[cfg(feature = "mode_module")]
#[deprecated(
    since = "0.3.0",
    note = "use bevy_auto_plugin::module::prelude instead"
)]
#[doc(inline)]
pub use crate::module::prelude as auto_plugin_module;
