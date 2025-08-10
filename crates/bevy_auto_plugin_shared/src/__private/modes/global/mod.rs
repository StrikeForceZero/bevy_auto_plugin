// derived from Bevy Butler - MIT/Apache 2.0 https://github.com/TGRCdev/bevy-butler/blob/4eca26421d275134e0adc907e8c851bdcf10823a/bevy-butler/src/__private/plugin.rs

pub mod inner;

use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::sync::LazyLock;
use syn::{ExprClosure, Path};

pub use bevy_app;
pub use bevy_log;
#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
pub use inventory;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "inventory")))]
pub use linkme;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "inventory")))]
#[linkme::distributed_slice]
pub static GLOBAL_AUTO_PLUGINS: [GlobalAutoPluginRegistryEntryFactory];

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
inventory::collect!(GlobalAutoPluginRegistryEntryFactory);

pub static GLOBAL_AUTO_PLUGIN_REGISTRY: LazyLock<GlobalAutoPluginRegistry> = LazyLock::new(|| {
    #[cfg(target_arch = "wasm32")]
    crate::_initialize();

    #[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
    let iter = GLOBAL_AUTO_PLUGINS.into_iter();
    #[cfg(any(target_arch = "wasm32", feature = "inventory"))]
    let iter = ::inventory::iter::<GlobalAutoPluginRegistryEntryFactory>.into_iter();

    let mut count = 0;
    let mut registry: HashMap<TypeId, Vec<BevyAppBuildFn>> = HashMap::new();
    iter.for_each(
        |GlobalAutoPluginRegistryEntryFactory(type_factory, sys_factory)| {
            registry
                .entry(type_factory())
                .or_default()
                .push(*sys_factory);
            count += 1;
        },
    );

    // Trim down
    registry.values_mut().for_each(|vec| vec.shrink_to_fit());
    registry.shrink_to_fit();

    bevy_log::debug!("Building GlobalAutoPluginRegistry from {count} entries");

    GlobalAutoPluginRegistry(registry)
});

pub trait AutoPluginTypeId {
    fn type_id() -> TypeId;
}

pub trait AutoPlugin: bevy_app::Plugin + AutoPluginTypeId {
    fn name(&self) -> &'static str {
        Self::static_name()
    }
    fn static_name() -> &'static str {
        type_name::<Self>()
    }
    fn build(&self, app: &mut bevy_app::App) {
        Self::static_build(app);
    }
    fn static_build(app: &mut bevy_app::App) {
        let type_id = <Self as AutoPluginTypeId>::type_id();
        GLOBAL_AUTO_PLUGIN_REGISTRY
            .get_entries(type_id)
            .iter()
            .for_each(|build_fn| {
                build_fn(app);
            });
    }
}

pub type TypeIdFn = fn() -> TypeId;
pub type BevyAppBuildFn = fn(&mut bevy_app::App);
pub struct GlobalAutoPluginRegistryEntryFactory(TypeIdFn, BevyAppBuildFn);

impl GlobalAutoPluginRegistryEntryFactory {
    pub const fn new(type_factory: fn() -> TypeId, sys_factory: fn(&mut bevy_app::App)) -> Self {
        Self(type_factory, sys_factory)
    }
}
pub struct GlobalAutoPluginRegistry(HashMap<TypeId, Vec<BevyAppBuildFn>>);

impl GlobalAutoPluginRegistry {
    pub(crate) fn get_entries(&'static self, marker: TypeId) -> &'static [BevyAppBuildFn] {
        self.0
            .get(&marker)
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }
}

pub fn _plugin_entry_block(static_ident: &Ident, plugin: &Path, expr: &ExprClosure) -> MacroStream {
    quote! {
        ::bevy_auto_plugin::__private::shared::_plugin_entry!(
            #static_ident,
            ::bevy_auto_plugin::__private::shared::__private::modes::global::GlobalAutoPluginRegistryEntryFactory::new(
                || <#plugin as ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPluginTypeId>::type_id(),
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
            #[::bevy_auto_plugin::__private::shared::__private::modes::global::linkme::distributed_slice(::bevy_auto_plugin::__private::shared::__private::modes::global::GLOBAL_AUTO_PLUGINS)]
            #[linkme(crate = ::bevy_auto_plugin::__private::shared::__private::modes::global::linkme)]
            #[allow(non_upper_case_globals)]
            static $static_ident:
                ::bevy_auto_plugin::__private::shared::__private::modes::global::GlobalAutoPluginRegistryEntryFactory =
                $entry;
        };
    }

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
#[macro_export]
#[doc(hidden)]
macro_rules! _plugin_entry {
    ($static_ident:ident, $entry:expr) => {
        ::bevy_auto_plugin::__private::shared::__private::modes::global::inventory::submit!($entry);
    };
}
