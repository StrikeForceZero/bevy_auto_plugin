use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use syn::{Error, Item, ItemFn, Path};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::{generate_add_events, generate_auto_names, generate_init_resources, generate_init_states, generate_register_state_types, generate_register_types};
use crate::inline::file_state;
use crate::inline::file_state::{update_file_state, update_state};
use crate::util::{is_fn_param_mutable_reference, resolve_path_from_item_or_args, FnParamMutabilityCheckErrMessages, Target};

pub fn auto_plugin_inner(input: ItemFn, app_param_name: Ident) -> syn::Result<MacroStream> {
    let _func_name = &input.sig.ident;
    let func_body = &input.block;
    let func_sig = &input.sig;
    let func_vis = &input.vis;
    let func_attrs = &input.attrs;

    // TODO: tuple struct with &'static string and app_param_name ?
    let app_param_mut_check_result = is_fn_param_mutable_reference(&input, &app_param_name, FnParamMutabilityCheckErrMessages {
        not_mutable_message: "auto_plugin attribute must be used on a function with a `&mut bevy::app::App` parameter".to_string(),
        not_found_message: format!("auto_plugin could not find the parameter named `{app_param_name}` in the function signature."),
    });
    app_param_mut_check_result?;

    let injected_code = auto_plugin_inner_to_stream(file_state::get_file_path(), &app_param_name)?;

    #[cfg(feature = "missing_auto_plugin_check")]
    let injected_code = {
        let output = files_missing_plugin_ts();
        quote! {
            #output
            #injected_code
        }
    };

    #[cfg(feature = "log_plugin_build")]
    let injected_code = quote! {
        log::debug!("plugin START");
        #injected_code
    };

    #[cfg(feature = "log_plugin_build")]
    let func_body = quote! {
        #func_body
        log::debug!("plugin END");
    };

    let expanded = quote! {
        #(#func_attrs)*
        #func_vis #func_sig {
            #injected_code
            #func_body
        }
    };
    
    Ok(expanded)
}

pub fn auto_plugin_inner_to_stream(file_path: String, app_param_name: &Ident) -> syn::Result<MacroStream> {
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