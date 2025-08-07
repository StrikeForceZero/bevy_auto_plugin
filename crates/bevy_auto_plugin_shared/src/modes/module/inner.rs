use crate::attribute::AutoPluginAttribute;
use crate::attribute_args::{AddSystemArgs, AddSystemWithTargetArgs, InsertResourceArgsWithPath};
use crate::bevy_app_code_gen::{
    generate_add_events, generate_add_systems, generate_auto_names, generate_init_resources,
    generate_init_states, generate_insert_resources, generate_register_state_types,
    generate_register_types,
};
use crate::modes::module::attribute::ModuleArgs;
use crate::util::{
    FnRef, ItemWithAttributeMatch, inject_module, items_with_attribute_macro,
    struct_or_enum_items_with_attribute_macro, to_compile_error,
};
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Item, ItemMod, parse2};

pub fn auto_plugin_inner(mut module: ItemMod, init_name: &Ident) -> syn::Result<MacroStream> {
    let app_param_ident = Ident::new("app", Span::call_site());
    // Extract the content inside the module
    if let Some((_, items)) = &module.content {
        fn map_to_path(
            iter: impl IntoIterator<Item = ItemWithAttributeMatch>,
        ) -> impl Iterator<Item = syn::Path> {
            iter.into_iter().map(ItemWithAttributeMatch::path_owned)
        }

        fn map_to_insert_resource(
            iter: impl IntoIterator<Item = ItemWithAttributeMatch>,
        ) -> impl Iterator<Item = syn::Result<InsertResourceArgsWithPath>> {
            iter.into_iter().map(InsertResourceArgsWithPath::try_from)
        }

        fn map_to_add_system_args(
            iter: impl IntoIterator<Item = ItemWithAttributeMatch>,
        ) -> darling::Result<Vec<AddSystemWithTargetArgs>> {
            iter.into_iter().try_fold(Vec::new(), |mut acc, item| {
                let args = AddSystemArgs::from_meta(&item.matched_attribute.meta)?;
                let it = AddSystemWithTargetArgs::try_from_macro_attr(item.path, args)
                    .map_err(darling::Error::from)?;
                acc.extend(it);
                Ok(acc)
            })
        }

        // Find all items with the provided [`attribute_name`] #[...] attribute
        let auto_register_types =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::RegisterType)?;
        let auto_register_types = map_to_path(auto_register_types);

        let auto_add_events =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::AddEvent)?;
        let auto_add_events = map_to_path(auto_add_events);

        let auto_init_resources =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InitResource)?;
        let auto_init_resources = map_to_path(auto_init_resources);

        let auto_insert_resources =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InsertResource)?;
        let auto_insert_resources = map_to_insert_resource(auto_insert_resources);
        let auto_insert_resources =
            auto_insert_resources
                .into_iter()
                .try_fold(vec![], |mut res, next| {
                    res.push(next?);
                    syn::Result::Ok(res)
                })?;

        let auto_names =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::Name)?;
        let auto_names = map_to_path(auto_names);

        let auto_register_state_types = struct_or_enum_items_with_attribute_macro(
            items,
            AutoPluginAttribute::RegisterStateType,
        )?;
        let auto_register_state_types = map_to_path(auto_register_state_types);

        let auto_init_states =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InitState)?;
        let auto_init_states = map_to_path(auto_init_states);

        let auto_add_system =
            items_with_attribute_macro::<FnRef>(items, AutoPluginAttribute::AddSystem)?;
        let auto_add_system = map_to_add_system_args(auto_add_system)?;

        inject_module(&mut module, move || {
            let auto_register_types =
                generate_register_types(&app_param_ident, auto_register_types)?;
            let auto_add_events = generate_add_events(&app_param_ident, auto_add_events)?;
            let auto_init_resources =
                generate_init_resources(&app_param_ident, auto_init_resources)?;
            let auto_insert_resources =
                generate_insert_resources(&app_param_ident, auto_insert_resources)?;
            let auto_names = generate_auto_names(&app_param_ident, auto_names)?;
            let auto_register_state_types =
                generate_register_state_types(&app_param_ident, auto_register_state_types)?;
            let auto_init_states = generate_init_states(&app_param_ident, auto_init_states)?;
            let auto_add_systems = generate_add_systems(&app_param_ident, auto_add_system)?;

            let func_body = quote! {
                #auto_register_types
                #auto_register_state_types
                #auto_add_events
                #auto_init_resources
                #auto_insert_resources
                #auto_init_states
                #auto_names
                #auto_add_systems
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

pub fn expand_module(attr: MacroStream, item: MacroStream) -> MacroStream {
    expand_module_inner(attr, item).unwrap_or_else(to_compile_error)
}

pub fn expand_module_inner(attr: MacroStream, item: MacroStream) -> syn::Result<MacroStream> {
    let attr_args: Vec<NestedMeta> = NestedMeta::parse_meta_list(attr)?;
    let args = ModuleArgs::from_list(&attr_args)?;
    let item_mod: ItemMod = parse2(item)?;
    auto_plugin_inner(item_mod, &args.init_name)
}
