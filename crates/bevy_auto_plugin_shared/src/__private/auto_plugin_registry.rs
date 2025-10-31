// derived from Bevy Butler - MIT/Apache 2.0 https://github.com/TGRCdev/bevy-butler/blob/4eca26421d275134e0adc907e8c851bdcf10823a/bevy-butler/src/__private/plugin.rs

use proc_macro2::{
    Ident,
    TokenStream as MacroStream,
};
use quote::quote;
use std::{
    any::{
        TypeId,
        type_name,
    },
    collections::HashMap,
    sync::LazyLock,
};
use syn::{
    ExprClosure,
    Path,
};

pub use bevy_app;
#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
pub use inventory;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "inventory")))]
pub use linkme;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "inventory")))]
#[linkme::distributed_slice]
pub static AUTO_PLUGINS: [AutoPluginRegistryEntryFactory];

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
inventory::collect!(AutoPluginRegistryEntryFactory);

pub static AUTO_PLUGIN_REGISTRY: LazyLock<AutoPluginRegistry> = LazyLock::new(|| {
    #[cfg(target_arch = "wasm32")]
    crate::_initialize();

    #[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
    let iter = AUTO_PLUGINS.into_iter();
    #[cfg(any(target_arch = "wasm32", feature = "inventory"))]
    let iter = ::inventory::iter::<AutoPluginRegistryEntryFactory>.into_iter();

    #[allow(unused_variables)]
    let mut count = 0;
    let mut registry: HashMap<TypeId, Vec<BevyAppBuildFn>> = HashMap::new();

    for (ix, AutoPluginRegistryEntryFactory(type_factory, sys_factory)) in iter.enumerate() {
        registry.entry(type_factory()).or_default().push(*sys_factory);
        #[allow(unused_assignments)]
        {
            count = ix + 1;
        }
    }

    // Trim down
    registry.values_mut().for_each(|vec| vec.shrink_to_fit());
    registry.shrink_to_fit();

    #[cfg(feature = "debug_log_plugin_registry")]
    log::debug!("Building AutoPluginRegistry from {count} entries");

    AutoPluginRegistry(registry)
});

pub trait AutoPluginTypeId {
    fn type_id() -> TypeId;
}

impl<T: 'static> AutoPluginTypeId for T {
    #[inline]
    fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

pub trait AutoPlugin: AutoPluginTypeId {
    #[inline]
    fn name(&self) -> &'static str {
        Self::static_name()
    }
    #[inline]
    fn static_name() -> &'static str {
        type_name::<Self>()
    }
    #[inline]
    fn build(&self, app: &mut bevy_app::App) {
        Self::static_build(app);
    }
    fn static_build(app: &mut bevy_app::App) {
        let type_id = <Self as AutoPluginTypeId>::type_id();
        AUTO_PLUGIN_REGISTRY.get_entries(type_id).iter().for_each(|build_fn| {
            build_fn(app);
        });
    }
}

pub type TypeIdFn = fn() -> TypeId;
pub type BevyAppBuildFn = fn(&mut bevy_app::App);
pub struct AutoPluginRegistryEntryFactory(TypeIdFn, BevyAppBuildFn);

impl AutoPluginRegistryEntryFactory {
    pub const fn new(type_factory: fn() -> TypeId, sys_factory: fn(&mut bevy_app::App)) -> Self {
        Self(type_factory, sys_factory)
    }
}
pub struct AutoPluginRegistry(HashMap<TypeId, Vec<BevyAppBuildFn>>);

impl AutoPluginRegistry {
    pub(crate) fn get_entries(&'static self, marker: TypeId) -> &'static [BevyAppBuildFn] {
        self.0.get(&marker).map(|v| v.as_slice()).unwrap_or_default()
    }
}

pub fn _plugin_entry_block(static_ident: &Ident, plugin: &Path, expr: &ExprClosure) -> MacroStream {
    quote! {
        ::bevy_auto_plugin::__private::shared::_plugin_entry!(
            #static_ident,
            ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPluginRegistryEntryFactory::new(
                || <#plugin as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPluginTypeId>::type_id(),
                #expr
            )
        );
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
#[macro_export]
#[doc(hidden)]
macro_rules! _plugin_entry {
    ($static_ident:ident, $entry:expr) => {
        #[::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::linkme::distributed_slice(
            ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AUTO_PLUGINS
        )]
        #[linkme(crate = ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::linkme)]
        #[allow(non_upper_case_globals)]
        static $static_ident:
            ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPluginRegistryEntryFactory =
            $entry;
    };
}

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
#[macro_export]
#[doc(hidden)]
macro_rules! _plugin_entry {
    ($static_ident:ident, $entry:expr) => {
        ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::inventory::submit!(
            $entry
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(all(test, not(feature = "inventory"), not(target_arch = "wasm32")))]
    mod native_tests {
        use super::*;
        #[test]
        fn builds_registry() {
            let _ = &*AUTO_PLUGIN_REGISTRY;
        }
    }

    #[cfg(all(test, any(feature = "inventory", target_arch = "wasm32")))]
    mod inv_tests {
        use super::*;
        #[test]
        fn builds_registry() {
            let _ = &*AUTO_PLUGIN_REGISTRY;
        }
    }
}
