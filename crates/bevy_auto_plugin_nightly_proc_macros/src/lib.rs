use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

#[cfg(feature = "missing_auto_plugin_check")]
use bevy_auto_plugin_nightly_shared::files_missing_plugin_ts;
use bevy_auto_plugin_nightly_shared::{FileState, UpdateStateError};
#[cfg(feature = "nightly_proc_macro_span")]
use bevy_auto_plugin_nightly_shared::{
    get_file_path as nightly_get_file_path, update_file_state as nightly_update_file_state,
    update_state as nightly_update_state,
};
use bevy_auto_plugin_shared::util::{
    FnParamMutabilityCheckErrMessages, Target, resolve_path_from_item_or_args,
};
use bevy_auto_plugin_shared::{
    generate_add_events, generate_auto_names, generate_init_resources, generate_init_states,
    generate_register_state_types, generate_register_types, util,
};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::meta::ParseNestedMeta;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, Item, ItemFn, Path, Result, Token, parse_macro_input};

fn update_file_state<R>(file_path: String, update_fn: impl FnOnce(&mut FileState) -> R) -> R {
    #[cfg(not(feature = "nightly_proc_macro_span"))]
    panic!("proc_macro_span feature is required for this crate");
    #[cfg(feature = "nightly_proc_macro_span")]
    nightly_update_file_state(file_path, update_fn)
}

fn update_state(
    file_path: String,
    path: Path,
    target: Target,
) -> std::result::Result<(), UpdateStateError> {
    #[cfg(not(feature = "nightly_proc_macro_span"))]
    panic!("proc_macro_span feature is required for this crate");
    #[cfg(feature = "nightly_proc_macro_span")]
    nightly_update_state(file_path, path, target)
}

fn get_file_path() -> String {
    #[cfg(not(feature = "nightly_proc_macro_span"))]
    panic!("proc_macro_span feature is required for this crate");
    #[cfg(feature = "nightly_proc_macro_span")]
    nightly_get_file_path()
}

#[derive(Default)]
struct AutoPluginAttributes {
    app_param_name: Option<Ident>,
}

impl AutoPluginAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("app") {
            self.app_param_name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    }
}

/// Attaches to a function accepting `&mut bevy::prelude::App`, automatically registering types, events, and resources in the `App`.
///
/// # Example
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// // Example attributes or declarations for components, events, or resources
/// // #[auto_register_type]
/// // #[derive(Component)]
/// // struct MyComponent;
///
/// // ^ auto macro attributes must be declared above #[auto_plugin]
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // Code generated by the macro is injected here.
///     // For example:
///     // app.register_type::<MyComponent>();
///
///     // Your custom logic comes here.
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let mut attrs = AutoPluginAttributes::default();
    let arg_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(attr with arg_parser);
    let Some(app_param_name) = attrs.app_param_name else {
        return Error::new(
            attrs.app_param_name.span(),
            "auto_plugin requires attribute specifying the name of the `&mut bevy::app::App` parameter. Example: #[auto_plugin(app=app)]",
        )
            .into_compile_error()
            .into();
    };

    // Parse the input function
    let input = parse_macro_input!(input as ItemFn);
    let _func_name = &input.sig.ident;
    let func_body = &input.block;
    let func_sig = &input.sig;
    let func_vis = &input.vis;
    let func_attrs = &input.attrs;

    // TODO: tuple struct with &'static string and app_param_name ?
    let app_param_mut_check_result = util::is_fn_param_mutable_reference(&input, &app_param_name, FnParamMutabilityCheckErrMessages {
        not_mutable_message: "auto_plugin attribute must be used on a function with a `&mut bevy::app::App` parameter".to_string(),
        not_found_message: format!("auto_plugin could not find the parameter named `{app_param_name}` in the function signature."),
    });
    if let Err(err) = app_param_mut_check_result {
        return err.into_compile_error().into();
    }

    let injected_code = match auto_plugin_inner(get_file_path(), &app_param_name) {
        Ok(code) => code,
        Err(err) => return err.to_compile_error().into(),
    };

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

    CompilerStream::from(expanded)
}

fn auto_plugin_inner(file_path: String, app_param_name: &Ident) -> Result<MacroStream> {
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

fn handle_attribute_inner(
    file_path: String,
    item: Item,
    attr_span: Span,
    target: Target,
    args: Option<Punctuated<Path, Comma>>,
) -> Result<()> {
    let path = resolve_path_from_item_or_args(&item, args)?;

    update_state(file_path, path, target).map_err(|err| Error::new(attr_span, err))?;

    Ok(())
}

fn handle_attribute(attr: CompilerStream, input: CompilerStream, target: Target) -> CompilerStream {
    let cloned_input = input.clone();
    let parsed_item = parse_macro_input!(input as Item);
    let args = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr with Punctuated::<Path, Token![,]>::parse_terminated))
    };

    handle_attribute_inner(
        get_file_path(),
        parsed_item,
        Span::call_site(),
        target,
        args,
    )
    .map(|_| cloned_input)
    .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Automatically registers a type with the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_register_type]
/// #[derive(Component, Reflect)]
/// #[reflect(Component)]
/// struct FooComponent;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.register_type::<FooComponent>();
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_register_type(FooComponentWithGeneric<bool>)]
/// #[auto_register_type(FooComponentWithGeneric<u32>)]
/// #[derive(Component, Reflect)]
/// #[reflect(Component)]
/// struct FooComponentWithGeneric<T>(T);
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.register_type::<FooComponentWithGeneric<bool>>();
///     app.register_type::<FooComponentWithGeneric<u32>>();
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::RegisterTypes)
}
/// Automatically adds an event type to the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_add_event]
/// #[derive(Event, Reflect)]
/// struct FooEvent;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     app.add_event::<FooEvent>();
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_add_event(FooEventWithGeneric<bool>)]
/// #[derive(Event, Reflect)]
/// struct FooEventWithGeneric<T>(T);
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     app.add_event::<FooEventWithGeneric<bool>>();
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::AddEvents)
}
/// Automatically initializes a resource in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_init_resource]
/// #[derive(Resource, Default, Reflect)]
/// #[reflect(Resource)]
/// struct FooResource;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     app.init_resource::<FooResource>();
/// }
/// ```
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_init_resource(FooResourceWithGeneric<bool>)]
/// #[derive(Resource, Default, Reflect)]
/// #[reflect(Resource)]
/// struct FooResourceWithGeneric<T>(T);
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     app.init_resource::<FooResourceWithGeneric<bool>>();
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::InitResources)
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_register_type]
/// #[derive(Component, Reflect)]
/// #[reflect(Component)]
/// #[auto_name]
/// struct FooComponent;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.register_type::<FooComponent>();
///     app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_register_type(FooComponentWithGeneric<bool>)]
/// #[auto_register_type(FooComponentWithGeneric<u32>)]
/// #[derive(Component, Reflect)]
/// #[reflect(Component)]
/// #[auto_name(FooComponentWithGeneric<bool>)]
/// struct FooComponentWithGeneric<T>(T);
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.register_type::<FooComponentWithGeneric<bool>>();
///     app.register_type::<FooComponentWithGeneric<u32>>();
///     app.register_required_components_with::<FooComponentWithGeneric<boo>, Name>(|| Name::new("FooComponentWithGeneric<boo>"));
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::RequiredComponentAutoName)
}

/// Automatically initializes a State in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_init_state]
/// #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
/// struct Foo;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.init_state::<Foo>();
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::InitStates)
}

/// Automatically registers a State type in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_register_state_type]
/// #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
/// struct Foo;
///
/// #[auto_plugin(app=app)]
/// fn plugin(app: &mut App) {
///     // generated code:
///     app.register_type::<State<Foo>>();
///     app.register_type::<NextState<Foo>>();
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(attr, input, Target::RegisterStateTypes)
}
