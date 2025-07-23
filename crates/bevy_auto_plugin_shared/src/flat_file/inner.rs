use crate::flat_file::attribute::FlatFileArgs;
use crate::flat_file::file_state::{update_file_state, update_state};
use crate::util::{
    FnParamMutabilityCheckErrMessages, LocalFile, StructOrEnumRef, TargetData, TargetRequirePath,
    is_fn_param_mutable_reference, resolve_local_file, resolve_path_from_item_or_args,
};
use crate::{
    AddSystemParams, StructOrEnumAttributeParams, generate_add_events, generate_add_systems,
    generate_auto_names, generate_init_resources, generate_init_states,
    generate_register_state_types, generate_register_types,
};
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Error, Item, ItemFn, Path, parse2};

pub fn auto_plugin_inner(
    file_path: String,
    input: ItemFn,
    app_param_name: Ident,
) -> syn::Result<MacroStream> {
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

    let injected_code = auto_plugin_inner_to_stream(file_path, &app_param_name)?;

    #[cfg(feature = "missing_auto_plugin_check")]
    let injected_code = {
        use crate::flat_file::file_state::files_missing_plugin_ts;
        let output = files_missing_plugin_ts();
        quote! {
            #output
            #injected_code
        }
    };

    let func_body = quote! {
        #injected_code
        #func_body
    };

    #[cfg(feature = "log_plugin_build")]
    let func_body = quote! {
        log::debug!("plugin START");
        #func_body
        log::debug!("plugin END");
    };

    let expanded = quote! {
        #(#func_attrs)*
        #func_vis #func_sig {
            #func_body
        }
    };

    Ok(expanded)
}

pub fn auto_plugin_inner_to_stream(
    file_path: String,
    app_param_name: &Ident,
) -> syn::Result<MacroStream> {
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
        let add_systems =
            generate_add_systems(app_param_name, file_state.context.add_systems.drain())?;
        Ok(quote! {
            #register_types
            #register_state_types
            #add_events
            #init_resources
            #init_states
            #auto_names
            #add_systems
        })
    })
}

macro_rules! extract_or_noop {
    ($out:ident, $item:expr, $ok:expr) => {
        #[allow(clippy::infallible_destructuring_match)]
        let $out = match $item {
            ValueOrNoop::Value(item) => item,
            #[cfg(feature = "lang_server_noop")]
            ValueOrNoop::Noop => $ok,
        };
    };
}

enum ValueOrNoop<T> {
    Value(T),
    // drops unreachable branches during compilation
    #[cfg(feature = "lang_server_noop")]
    Noop,
}

fn resolve_local_file_spanned(span: Span) -> syn::Result<ValueOrNoop<String>> {
    Ok(match resolve_local_file() {
        LocalFile::File(path) => ValueOrNoop::Value(path),
        #[cfg(feature = "lang_server_noop")]
        LocalFile::Noop => ValueOrNoop::Noop,
        LocalFile::Error(err) => return Err(Error::new(span, err)),
    })
}

pub fn handle_attribute_outer(
    item: Item,
    attr_span: Span,
    target: TargetRequirePath,
    args: StructOrEnumAttributeParams,
) -> syn::Result<()> {
    extract_or_noop!(
        file_path,
        resolve_local_file_spanned(attr_span)?,
        return Ok(())
    );
    handle_attribute_inner(file_path, item, attr_span, target, args)
}

pub fn handle_attribute_inner(
    file_path: String,
    item: Item,
    attr_span: Span,
    target: TargetRequirePath,
    args: StructOrEnumAttributeParams,
) -> syn::Result<()> {
    let path = resolve_path_from_item_or_args::<StructOrEnumRef>(&item, args)?;
    let target_data = TargetData::from_target_require_path(target, path);
    update_state(file_path, target_data).map_err(|err| Error::new(attr_span, err))?;
    Ok(())
}

pub fn handle_add_system_attribute_outer(
    item: ItemFn,
    args: AddSystemParams,
    attr_span: Span,
) -> syn::Result<()> {
    extract_or_noop!(
        file_path,
        resolve_local_file_spanned(attr_span)?,
        return Ok(())
    );
    handle_add_system_attribute_inner(file_path, item, args, attr_span)
}

pub fn handle_add_system_attribute_inner(
    file_path: String,
    item: ItemFn,
    args: AddSystemParams,
    attr_span: Span,
) -> syn::Result<()> {
    let ident = &item.sig.ident;
    let path = Path::from_string(&ident.to_string())?;
    let target_data = TargetData::AddSystem {
        system: path,
        params: args,
    };
    update_state(file_path, target_data).map_err(|err| Error::new(attr_span, err))?;
    Ok(())
}

pub fn expand_flat_file(attr: MacroStream, item: MacroStream) -> syn::Result<MacroStream> {
    let attr_args: Vec<NestedMeta> = NestedMeta::parse_meta_list(attr)?;
    let args = FlatFileArgs::from_list(&attr_args)?;
    let item_fn: ItemFn = parse2(item)?;
    let app_param = args.resolve_app_param_name(&item_fn)?;
    extract_or_noop!(file_path, resolve_local_file_spanned(Span::call_site())?, {
        use quote::ToTokens;
        return Ok(item_fn.to_token_stream());
    });
    auto_plugin_inner(file_path, item_fn, app_param)
}
