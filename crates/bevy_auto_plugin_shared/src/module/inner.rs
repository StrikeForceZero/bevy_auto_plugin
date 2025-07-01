use crate::util::{ItemWithAttributeMatch, inject_module, items_with_attribute_macro};
use crate::{
    AutoPluginAttribute, generate_add_events, generate_auto_names, generate_init_resources,
    generate_init_states, generate_register_state_types, generate_register_types,
};
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Item, ItemMod, parse2};

pub fn auto_plugin_inner(mut module: ItemMod, init_name: &Ident) -> syn::Result<MacroStream> {
    let app_param_ident = Ident::new("app", Span::call_site());
    // Extract the content inside the module
    if let Some((_, items)) = &module.content {
        fn map_to_string(
            iter: impl IntoIterator<Item = ItemWithAttributeMatch>,
        ) -> impl Iterator<Item = String> {
            iter.into_iter()
                .map(ItemWithAttributeMatch::into_path_string)
        }

        // Find all items with the provided [`attribute_name`] #[...] attribute
        let auto_register_types =
            items_with_attribute_macro(items, AutoPluginAttribute::RegisterType)?;
        let auto_register_types = map_to_string(auto_register_types);

        let auto_add_events = items_with_attribute_macro(items, AutoPluginAttribute::AddEvent)?;
        let auto_add_events = map_to_string(auto_add_events);

        let auto_init_resources =
            items_with_attribute_macro(items, AutoPluginAttribute::InitResource)?;
        let auto_init_resources = map_to_string(auto_init_resources);

        let auto_names = items_with_attribute_macro(items, AutoPluginAttribute::Name)?;
        let auto_names = map_to_string(auto_names);

        let auto_register_state_types =
            items_with_attribute_macro(items, AutoPluginAttribute::RegisterStateType)?;
        let auto_register_state_types = map_to_string(auto_register_state_types);

        let auto_init_states = items_with_attribute_macro(items, AutoPluginAttribute::InitState)?;
        let auto_init_states = map_to_string(auto_init_states);

        inject_module(&mut module, move || {
            let auto_register_types =
                generate_register_types(&app_param_ident, auto_register_types)?;
            let auto_add_events = generate_add_events(&app_param_ident, auto_add_events)?;
            let auto_init_resources =
                generate_init_resources(&app_param_ident, auto_init_resources)?;
            let auto_names = generate_auto_names(&app_param_ident, auto_names)?;
            let auto_register_state_types =
                generate_register_state_types(&app_param_ident, auto_register_state_types)?;
            let auto_init_states = generate_init_states(&app_param_ident, auto_init_states)?;
            
            let func_body = quote! {                
                #auto_register_types
                #auto_register_state_types
                #auto_add_events
                #auto_init_resources
                #auto_init_states
                #auto_names
            };

            #[cfg(feature = "log_plugin_build")]
            let func_body = quote! {
                log::debug!("plugin START");
                #func_body
                log::debug!("plugin END");
            };
            
            parse2::<Item>(quote! {
                pub(super) fn #init_name(app: &mut bevy_app::prelude::App) {
                    #func_body
                }
            })
        })?;
    }

    let output = quote! {
        #module
    };

    Ok(output)
}
