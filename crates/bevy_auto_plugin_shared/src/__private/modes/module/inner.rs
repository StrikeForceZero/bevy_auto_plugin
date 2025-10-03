use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::attribute_args::attributes::modes::module::auto_plugin::AutoPluginArgs;
use crate::__private::attribute_args::attributes::prelude::{
    AddMessageAttributeArgs, AddObserverAttributeArgs, AddSystemAttributeArgs,
    AutoNameAttributeArgs, InitResourceAttributeArgs, InitStateAttributeArgs,
    InsertResourceAttributeArgs, RegisterStateTypeAttributeArgs, RegisterTypeAttributeArgs,
};
use crate::__private::context::{AutoPluginContext, ToTokenStringValue};
use crate::__private::util::concrete_path::ConcreteTargetPathWithGenericsCollection;
use crate::__private::util::module::inject_module;
use crate::__private::util::tokens::to_compile_error;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Item, ItemMod, parse2};

pub fn auto_plugin_inner(mut module: ItemMod, init_name: &Ident) -> syn::Result<MacroStream> {
    let app_param_ident = Ident::new("app", Span::call_site());
    // Extract the content inside the module
    if let Some((_, items)) = &module.content {
        // Find all items with the provided [`attribute_name`] #[...] attribute
        let register_types = RegisterTypeAttributeArgs::match_items(items)?;
        let add_events = AddMessageAttributeArgs::match_items(items)?;
        let init_resources = InitResourceAttributeArgs::match_items(items)?;
        let insert_resources = InsertResourceAttributeArgs::match_items(items)?;
        let auto_names = AutoNameAttributeArgs::match_items(items)?;
        let register_state_types = RegisterStateTypeAttributeArgs::match_items(items)?;
        let init_states = InitStateAttributeArgs::match_items(items)?;
        let add_systems = AddSystemAttributeArgs::match_items(items)?;
        let add_observers = AddObserverAttributeArgs::match_items(items)?;

        let mut context = AutoPluginContext::default();

        macro_rules! insert {
            ($var:ident, $ident:ident) => {
                $var.into_iter().for_each(|item| {
                    // TODO: this turned out ugly
                    ConcreteTargetPathWithGenericsCollection::from(&item)
                        .into_iter()
                        .for_each(|path| {
                            context
                                .$var
                                .insert(ToTokenStringValue::<$ident>::from((path, &item.args)));
                        });
                });
            };
        }

        insert!(register_types, RegisterTypeAttributeArgs);
        insert!(add_events, AddMessageAttributeArgs);
        insert!(init_resources, InitResourceAttributeArgs);
        insert!(insert_resources, InsertResourceAttributeArgs);
        insert!(auto_names, AutoNameAttributeArgs);
        insert!(register_state_types, RegisterStateTypeAttributeArgs);
        insert!(init_states, InitStateAttributeArgs);
        insert!(add_systems, AddSystemAttributeArgs);
        insert!(add_observers, AddObserverAttributeArgs);

        inject_module(&mut module, move || {
            let func_body = context.expand_build(&app_param_ident);

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
    let args = AutoPluginArgs::from_list(&attr_args)?;
    let item_mod: ItemMod = parse2(item)?;
    auto_plugin_inner(item_mod, &args.init_name)
}
