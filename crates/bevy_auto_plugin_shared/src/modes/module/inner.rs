use crate::attribute::AutoPluginAttribute;
use crate::attribute_args::{AddSystemArgs, AddSystemWithTargetArgs, InsertResourceArgsWithPath};
use crate::bevy_app_code_gen::{InputSets, expand_input_sets};
use crate::item_with_attr_match::struct_or_enum_items_with_attribute_macro;
use crate::item_with_attr_match::{ItemWithAttributeMatch, items_with_attribute_macro};
use crate::modes::module::attribute::ModuleArgs;
use crate::util::meta::fn_meta::FnMeta;
use crate::util::module::inject_module;
use crate::util::tokens::to_compile_error;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Item, ItemMod, parse2};

pub fn auto_plugin_inner(mut module: ItemMod, init_name: &Ident) -> syn::Result<MacroStream> {
    let app_param_ident = Ident::new("app", Span::call_site());
    // Extract the content inside the module
    if let Some((_, items)) = &module.content {
        fn map_to_path(iter: impl IntoIterator<Item = ItemWithAttributeMatch>) -> Vec<syn::Path> {
            iter.into_iter()
                .map(ItemWithAttributeMatch::path_owned)
                .collect()
        }

        fn map_to_insert_resource(
            iter: impl IntoIterator<Item = ItemWithAttributeMatch>,
        ) -> Vec<syn::Result<InsertResourceArgsWithPath>> {
            iter.into_iter()
                .map(InsertResourceArgsWithPath::try_from)
                .collect()
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
        let register_types =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::RegisterType)?;
        let register_types = map_to_path(register_types);

        let add_events =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::AddEvent)?;
        let add_events = map_to_path(add_events);

        let init_resources =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InitResource)?;
        let init_resources = map_to_path(init_resources);

        let insert_resources =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InsertResource)?;
        let insert_resources = map_to_insert_resource(insert_resources);
        let insert_resources = insert_resources
            .into_iter()
            .try_fold(vec![], |mut res, next| {
                res.push(next?);
                syn::Result::Ok(res)
            })?;

        let auto_names =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::Name)?;
        let auto_names = map_to_path(auto_names);

        let register_state_types = struct_or_enum_items_with_attribute_macro(
            items,
            AutoPluginAttribute::RegisterStateType,
        )?;
        let register_state_types = map_to_path(register_state_types);

        let init_states =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::InitState)?;
        let init_states = map_to_path(init_states);

        let add_systems =
            items_with_attribute_macro::<FnMeta>(items, AutoPluginAttribute::AddSystem)?;
        let add_systems = map_to_add_system_args(add_systems)?;

        let add_observers =
            struct_or_enum_items_with_attribute_macro(items, AutoPluginAttribute::AddObserver)?;
        let add_observers = map_to_path(add_observers);

        inject_module(&mut module, move || {
            let func_body = expand_input_sets(
                &app_param_ident,
                InputSets {
                    register_types,
                    register_state_types,
                    auto_names,
                    add_events,
                    add_systems,
                    add_observers,
                    insert_resources,
                    init_states,
                    init_resources,
                },
            )?;

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
