#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub mod flat_file;
#[cfg(feature = "mode_global")]
pub mod global;
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub mod module;
