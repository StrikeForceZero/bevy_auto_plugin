use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::attribute_args::attributes::add_observer::AddObserverAttributeArgs;
use crate::__private::attribute_args::attributes::modes::flat_file::auto_plugin::AutoPluginArgs;
use crate::__private::attribute_args::attributes::modes::resolve_app_param_name;
use crate::__private::attribute_args::attributes::prelude::{
    AddSystemAttributeArgs, InsertResourceAttributeArgs,
};
use crate::__private::context::{
    AutoPluginContextInsert, SupportsAutoPluginContextInsert, ToTokenStringValue,
};
use crate::__private::modes::flat_file::file_state::{update_file_state, update_state};
use crate::__private::util::concrete_path::{
    ConcreteTargetPathWithGenericsCollection, resolve_paths_from_item_or_args,
};
use crate::__private::util::local_file::{LocalFile, resolve_local_file};
use crate::__private::util::meta::fn_meta::{FnMeta, require_fn_param_mutable_reference};
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::__private::util::path_fmt::PathWithoutGenerics;
use crate::__private::util::tokens::to_compile_error;
use crate::{ok_or_return_compiler_error, parse_macro_input2};
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Error, Item, ItemFn, parse2};

pub fn auto_plugin_inner(
    file_path: String,
    input: &ItemFn,
    app_param_name: &Ident,
) -> syn::Result<MacroStream> {
    let _func_name = &input.sig.ident;
    let func_body = &input.block;
    let func_sig = &input.sig;
    let func_vis = &input.vis;
    let func_attrs = &input.attrs;

    require_fn_param_mutable_reference(input, app_param_name, "auto_plugin")?;

    let injected_code = auto_plugin_inner_to_stream(file_path, app_param_name)?;

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
        file_state.plugin_registered = true;
        let tokens = file_state.context.expand_build(app_param_name);
        Ok(tokens)
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
    resource_args: InsertResourceAttributeArgs,
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
    resource_args: InsertResourceAttributeArgs,
) -> syn::Result<()> {
    let paths = resolve_paths_from_item_or_args::<StructOrEnumMeta, _>(&item, &resource_args)?;
    let mut paths = paths.into_iter();
    let path = paths
        .next()
        .ok_or_else(|| Error::new(attr_span, "failed to resolve any path"))?;
    if paths.next().is_some() {
        return Err(Error::new(attr_span, "failed to resolve single path"));
    }
    let to_token_string_value = ToTokenStringValue::from((path, resource_args));
    update_state(file_path, to_token_string_value).map_err(|err| Error::new(attr_span, err))?;
    Ok(())
}

pub fn handle_attribute_outer<T>(item: Item, attr_span: Span, args: T) -> syn::Result<()>
where
    T: ItemAttributeArgs + SupportsAutoPluginContextInsert,
    ToTokenStringValue<T>: AutoPluginContextInsert,
{
    extract_or_noop!(
        file_path,
        resolve_local_file_spanned(attr_span)?,
        return Ok(())
    );
    handle_attribute_inner(file_path, item, attr_span, args)
}

pub fn handle_attribute_inner<T>(
    file_path: String,
    item: Item,
    attr_span: Span,
    args: T,
) -> syn::Result<()>
where
    T: ItemAttributeArgs + SupportsAutoPluginContextInsert,
    ToTokenStringValue<T>: AutoPluginContextInsert,
{
    let paths = resolve_paths_from_item_or_args::<StructOrEnumMeta, _>(&item, &args)?;
    for path in paths {
        let ttsv = ToTokenStringValue::from((path, &args));
        // TODO: cloning here feels dumb
        update_state(file_path.clone(), ttsv).map_err(|err| Error::new(attr_span, err))?;
    }
    Ok(())
}

pub fn handle_add_observer_attribute(attr: MacroStream, input: MacroStream) -> MacroStream {
    let cloned_input = input.clone();
    let item = parse_macro_input2!(input as ItemFn);
    let add_observer_args = parse_macro_input2!(attr as AddObserverAttributeArgs);
    handle_add_observer_attribute_outer(item, Span::call_site(), add_observer_args)
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}

pub fn handle_add_observer_attribute_outer(
    item: ItemFn,
    attr_span: Span,
    args: AddObserverAttributeArgs,
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
    args: AddObserverAttributeArgs,
) -> syn::Result<()> {
    let item = Item::Fn(item);
    let paths = resolve_paths_from_item_or_args::<FnMeta, _>(&item, &args)?;
    for path in paths {
        let ttsv = ToTokenStringValue::from((path, &args));
        // TODO: cloning here feels dumb
        update_state(file_path.clone(), ttsv).map_err(|err| Error::new(attr_span, err))?;
    }
    Ok(())
}

pub fn handle_insert_resource_attribute(attr: MacroStream, input: MacroStream) -> MacroStream {
    let cloned_input = input.clone();
    let item = parse_macro_input2!(input as Item);
    // TODO: compiler error if multiple auto_insert_resource attributes found for same type
    let insert_resource_args = parse_macro_input2!(attr as InsertResourceAttributeArgs);
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
    let args = parse_macro_input2!(attr as AddSystemAttributeArgs);
    handle_add_system_attribute_outer(item, args, Span::call_site())
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}

pub fn handle_add_system_attribute_outer(
    item: ItemFn,
    args: AddSystemAttributeArgs,
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
    args: AddSystemAttributeArgs,
    attr_span: Span,
) -> syn::Result<()> {
    let path_without_generics = PathWithoutGenerics::from(item.sig.ident);
    let concrete_target_paths =
        ConcreteTargetPathWithGenericsCollection::from_args(path_without_generics, &args);
    for path in concrete_target_paths {
        let ttsv = ToTokenStringValue::from((path, &args));
        // TODO: cloning here feels dumb
        update_state(file_path.clone(), ttsv).map_err(|err| Error::new(attr_span, err))?;
    }
    Ok(())
}

pub fn expand_flat_file(attr: MacroStream, item: MacroStream) -> MacroStream {
    expand_flat_file_inner(attr, item).unwrap_or_else(to_compile_error)
}

pub fn expand_flat_file_inner(attr: MacroStream, item: MacroStream) -> syn::Result<MacroStream> {
    let attr_args: Vec<NestedMeta> = NestedMeta::parse_meta_list(attr)?;
    let args = AutoPluginArgs::from_list(&attr_args)?;
    let item_fn: ItemFn = parse2(item)?;
    let app_param = resolve_app_param_name(&item_fn, args.app_param.as_ref())?;
    extract_or_noop!(file_path, resolve_local_file_spanned(Span::call_site())?, {
        use quote::ToTokens;
        return Ok(item_fn.to_token_stream());
    });
    auto_plugin_inner(file_path, &item_fn, app_param)
}

/// Handle a flat-file attribute
pub fn flat_file_handle_attribute<T>(attr: MacroStream, input: MacroStream) -> MacroStream
where
    T: ItemAttributeArgs + SupportsAutoPluginContextInsert,
    ToTokenStringValue<T>: AutoPluginContextInsert,
{
    let cloned_input = input.clone();
    let parsed_item: Item = match parse2(input) {
        Ok(it) => it,
        Err(err) => return err.to_compile_error(),
    };

    let args = ok_or_return_compiler_error!(parse2::<T>(attr));

    handle_attribute_outer(parsed_item, Span::call_site(), args)
        .map(|_| cloned_input)
        .unwrap_or_else(to_compile_error)
}
