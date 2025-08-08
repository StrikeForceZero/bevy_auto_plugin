/// Modes
pub mod modes;

#[doc(hidden)]
pub mod __private {
    pub use bevy_auto_plugin_shared as shared;
}

#[deprecated(
    since = "0.3.0",
    note = "use `bevy_auto_plugin::modes::flat_file` instead"
)]
#[cfg(feature = "mode_flat_file")]
/// Pre v0.3 exports
pub mod auto_plugin {
    pub use crate::modes::flat_file::*;
}

#[deprecated(
    since = "0.3.0",
    note = "use `bevy_auto_plugin::modes::module` instead"
)]
#[cfg(feature = "mode_module")]
/// Pre v0.3 exports
pub mod auto_plugin_module {
    pub use crate::modes::module::*;
}

#[deprecated(
    since = "0.4.0",
    note = "use `bevy_auto_plugin::modes::flat_file` instead"
)]
#[cfg(feature = "mode_flat_file")]
/// Pre v0.4 exports
pub mod flat_file {
    pub use crate::modes::flat_file::*;
}
#[deprecated(
    since = "0.4.0",
    note = "use `bevy_auto_plugin::modes::global` instead"
)]
#[cfg(feature = "mode_global")]
/// Pre v0.4 exports
pub mod global {
    pub use crate::modes::global::*;
}
#[deprecated(
    since = "0.4.0",
    note = "use `bevy_auto_plugin::modes::module` instead"
)]
#[cfg(feature = "mode_module")]
/// Pre v0.4 exports
pub mod module {
    pub use crate::modes::module::*;
}
