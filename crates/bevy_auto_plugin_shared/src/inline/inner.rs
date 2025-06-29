use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use syn::{Error, Item, Path};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::{generate_add_events, generate_auto_names, generate_init_resources, generate_init_states, generate_register_state_types, generate_register_types};
use crate::inline::file_state::{update_file_state, update_state};
use crate::util::{resolve_path_from_item_or_args, Target};

pub fn auto_plugin_inner(file_path: String, app_param_name: &Ident) -> syn::Result<MacroStream> {
    update_file_state(file_path, |file_state| {
        if file_state.plugin_registered {
            return Err(Error::new(
                Span::call_site(),
                "plugin already registered or duplicate attribute",
            ));
        }
        file_state.plugin_registered = true;
        let register_types = generate_register_types(
            app_param_name,
            file_state.context.register_types.clone().drain(),
        )?;
        let register_state_types = generate_register_state_types(
            app_param_name,
            file_state.context.register_state_types.drain(),
        )?;
        let add_events =
            generate_add_events(app_param_name, file_state.context.add_events.drain())?;
        let init_resources =
            generate_init_resources(app_param_name, file_state.context.init_resources.drain())?;
        let init_states =
            generate_init_states(app_param_name, file_state.context.init_states.drain())?;
        let auto_names =
            generate_auto_names(app_param_name, file_state.context.auto_names.drain())?;
        Ok(quote! {
            #register_types
            #register_state_types
            #add_events
            #init_resources
            #init_states
            #auto_names
        })
    })
}

pub fn handle_attribute_inner(
    file_path: String,
    item: Item,
    attr_span: Span,
    target: Target,
    args: Option<Punctuated<Path, Comma>>,
) -> syn::Result<()> {
    let path = resolve_path_from_item_or_args(&item, args)?;

    update_state(file_path, path, target).map_err(|err| Error::new(attr_span, err))?;

    Ok(())
}