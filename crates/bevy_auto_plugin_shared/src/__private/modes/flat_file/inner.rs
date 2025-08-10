use crate::__private::attribute_args::{
    AddObserverArgs, AddSystemArgs, AddSystemSerializedArgs, AddSystemWithTargetArgs,
    InsertResourceArgsWithPath, InsertResourceSerializedArgsWithPath,
};
use crate::__private::attribute_args::{InsertResourceArgs, StructOrEnumAttributeArgs};
use crate::__private::bevy_app_code_gen::{InputSets, expand_input_sets};
use crate::__private::modes::flat_file::attribute::FlatFileArgs;
use crate::__private::modes::flat_file::file_state::{update_file_state, update_state};
use crate::__private::target::{TargetData, TargetRequirePath};
use crate::__private::util::concrete_path::resolve_paths_from_item_or_args;
use crate::__private::util::local_file::{LocalFile, resolve_local_file};
use crate::__private::util::meta::fn_meta::{FnMeta, require_fn_param_mutable_reference};
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::__private::util::tokens::to_compile_error;
use crate::{ok_or_return_compiler_error, parse_macro_input2};
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Error, Item, ItemFn, Path, parse_str, parse2};

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

    require_fn_param_mutable_reference(&input, &app_param_name, "auto_plugin")?;

    let injected_code = auto_plugin_inner_to_stream(file_path, &app_param_name)?;

    #[cfg(feature = "flat_file_missing_auto_plugin_check")]
    let injected_code = {
        use crate::__private::modes::flat_file::file_state::files_missing_plugin_ts;
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
        fn map_to_path(input: impl IntoIterator<Item = String>) -> syn::Result<Vec<Path>> {
            input
                .into_iter()
                .map(|str| parse_str::<Path>(&str))
                .collect::<syn::Result<Vec<_>>>()
        }
        fn map_to_insert_resource(
            input: impl IntoIterator<Item = InsertResourceSerializedArgsWithPath>,
        ) -> syn::Result<Vec<InsertResourceArgsWithPath>> {
            input
                .into_iter()
                .map(InsertResourceArgsWithPath::try_from)
                .collect::<syn::Result<Vec<_>>>()
        }
        fn map_to_add_systems(
            input: impl IntoIterator<Item = AddSystemSerializedArgs>,
        ) -> syn::Result<Vec<AddSystemWithTargetArgs>> {
            input
                .into_iter()
                .map(AddSystemWithTargetArgs::try_from)
                .collect::<syn::Result<Vec<_>>>()
        }
        file_state.plugin_registered = true;

        let register_types = map_to_path(file_state.context.register_types.drain())?;
        let register_state_types = map_to_path(file_state.context.register_state_types.drain())?;
        let auto_names = map_to_path(file_state.context.auto_names.drain())?;
        let add_events = map_to_path(file_state.context.add_events.drain())?;
        let add_systems = map_to_add_systems(file_state.context.add_systems.drain())?;
        let add_observers = map_to_path(file_state.context.add_observers.drain())?;
        let insert_resources = map_to_insert_resource(file_state.context.insert_resources.drain())?;
        let init_states = map_to_path(file_state.context.init_states.drain())?;
        let init_resources = map_to_path(file_state.context.init_resources.drain())?;

        let pipeline_tokens = expand_input_sets(
            app_param_name,
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
        Ok(pipeline_tokens)
    })
}

macro_rules! extract_or_noop {
    ($out:ident, $item:expr, $ok:expr) => {
        #[allow(clippy::infallible_destructuring_match)]
        let $out = match $item {
            ValueOrNoop::Value(item) => item,
            #[cfg(feature = "flat_file_lang_server_noop")]
            ValueOrNoop::Noop => $ok,
        };
    };
}

enum ValueOrNoop<T> {
    Value(T),
    // drops unreachable branches during compilation
    #[cfg(feature = "flat_file_lang_server_noop")]
    Noop,
}

fn resolve_local_file_spanned(span: Span) -> syn::Result<ValueOrNoop<String>> {
    Ok(match resolve_local_file() {
        LocalFile::File(path) => ValueOrNoop::Value(path),
        #[cfg(feature = "flat_file_lang_server_noop")]
        LocalFile::Noop => ValueOrNoop::Noop,
        LocalFile::Error(err) => return Err(Error::new(span, err)),
    })
}

pub fn handle_insert_resource_outer(
    item: Item,
    attr_span: Span,
    resource_args: InsertResourceArgs,
) -> syn::Result<()> {
    extract_or_noop!(
        file_path,
        resolve_local_file_spanned(attr_span)?,
        return Ok(())
    );
    handle_insert_resource_inner(item, file_path, attr_span, resource_args)
}

pub fn handle_insert_resource_inner(
    item: Item,
    file_path: String,
    attr_span: Span,
    resource_args: InsertResourceArgs,
) -> syn::Result<()> {
    let paths = resolve_paths_from_item_or_args::<StructOrEnumMeta>(
        &item,
        StructOrEnumAttributeArgs {
            generics: resource_args
                .generics
                .as_ref()
                .map(|generics| vec![generics.clone()])
                .unwrap_or_default(),
        },
    )?;
    let mut paths = paths.into_iter();
    let path = paths
        .next()
        .ok_or_else(|| Error::new(attr_span, "failed to resolve any path"))?;
    if paths.next().is_some() {
        return Err(Error::new(attr_span, "failed to resolve single path"));
    }
    update_state(
        file_path,
        TargetData::InsertResource(InsertResourceArgsWithPath {
            path,
            resource_args,
        }),
    )
    .map_err(|err| Error::new(attr_span, err))?;
    Ok(())
}

pub fn handle_attribute_outer(
    item: Item,
    attr_span: Span,
    target: TargetRequirePath,
    args: StructOrEnumAttributeArgs,
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
    args: StructOrEnumAttributeArgs,
) -> syn::Result<()> {
    let paths = resolve_paths_from_item_or_args::<StructOrEnumMeta>(&item, args)?;
    for path in paths {
        let target_data = TargetData::from_target_require_path(target, path);
        // TODO: cloning here feels dumb
        update_state(file_path.clone(), target_data).map_err(|err| Error::new(attr_span, err))?;
    }
    Ok(())
}

pub fn handle_add_observer_attribute(attr: MacroStream, input: MacroStream) -> MacroStream {
    let cloned_input = input.clone();
    let item = parse_macro_input2!(input as ItemFn);
    let add_observer_args = parse_macro_input2!(attr as AddObserverArgs);
    handle_add_observer_attribute_outer(item, Span::call_site(), add_observer_args)
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}

pub fn handle_add_observer_attribute_outer(
    item: ItemFn,
    attr_span: Span,
    args: AddObserverArgs,
) -> syn::Result<()> {
    extract_or_noop!(
        file_path,
        resolve_local_file_spanned(attr_span)?,
        return Ok(())
    );
    handle_add_observer_attribute_inner(file_path, item, attr_span, args)
}

pub fn handle_add_observer_attribute_inner(
    file_path: String,
    item: ItemFn,
    attr_span: Span,
    args: AddObserverArgs,
) -> syn::Result<()> {
    let item = Item::Fn(item);
    let paths = resolve_paths_from_item_or_args::<FnMeta>(&item, args)?;
    for path in paths {
        let target_data =
            TargetData::from_target_require_path(TargetRequirePath::AddObserver, path);
        // TODO: cloning here feels dumb
        update_state(file_path.clone(), target_data).map_err(|err| Error::new(attr_span, err))?;
    }
    Ok(())
}

pub fn handle_insert_resource_attribute(attr: MacroStream, input: MacroStream) -> MacroStream {
    let cloned_input = input.clone();
    let item = parse_macro_input2!(input as Item);
    // TODO: compiler error if multiple auto_insert_resource attributes found for same type
    let insert_resource_args = parse_macro_input2!(attr as InsertResourceArgs);
    if let Err(err) = insert_resource_args.validate_resource() {
        return err.to_compile_error();
    }
    handle_insert_resource_outer(item, Span::call_site(), insert_resource_args)
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}

pub fn handle_add_system_attribute(attr: MacroStream, input: MacroStream) -> MacroStream {
    let cloned_input = input.clone();
    let item = parse_macro_input2!(input as ItemFn);
    let args = parse_macro_input2!(attr as AddSystemArgs);
    handle_add_system_attribute_outer(item, args, Span::call_site())
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}

pub fn handle_add_system_attribute_outer(
    item: ItemFn,
    args: AddSystemArgs,
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
    args: AddSystemArgs,
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

pub fn expand_flat_file(attr: MacroStream, item: MacroStream) -> MacroStream {
    expand_flat_file_inner(attr, item).unwrap_or_else(to_compile_error)
}

pub fn expand_flat_file_inner(attr: MacroStream, item: MacroStream) -> syn::Result<MacroStream> {
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

/// Handle a flat-file attribute (e.g. `auto_register_type`) that targets a
/// single [`TargetRequirePath`].
pub fn flat_file_handle_attribute(
    attr: MacroStream,
    input: MacroStream,
    target: TargetRequirePath,
) -> MacroStream {
    let cloned_input = input.clone();
    let parsed_item: Item = match parse2(input) {
        Ok(it) => it,
        Err(err) => return err.to_compile_error(),
    };

    // LEGACY PATH PARAM SUPPORT (unchanged)
    #[cfg(feature = "legacy_path_param")]
    let attr_cloned = attr.clone();

    let args: syn::Result<StructOrEnumAttributeArgs> = match syn::parse2(attr) {
        Ok(a) => Ok(a),
        Err(err) => {
            #[cfg(not(feature = "legacy_path_param"))]
            {
                return err.to_compile_error();
            }
            #[cfg(feature = "legacy_path_param")]
            {
                use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
                StructOrEnumMeta::try_from(&parsed_item)
                    .and_then(|se_ref| {
                        crate::__private::util::concrete_path::legacy_generics_from_path(
                            &se_ref,
                            attr_cloned,
                        )
                    })
                    .map(StructOrEnumAttributeArgs::from)
                    .map_err(|legacy_err| {
                        syn::Error::new(err.span(), format!("\nnew: {err}\nlegacy: {legacy_err}"))
                    })
            }
        }
    };
    let args = ok_or_return_compiler_error!(args);

    handle_attribute_outer(parsed_item, Span::call_site(), target, args)
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}
