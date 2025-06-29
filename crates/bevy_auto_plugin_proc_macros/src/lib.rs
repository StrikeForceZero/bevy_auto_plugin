use proc_macro::{TokenStream as CompilerStream};
use syn::{parse_macro_input, ItemMod};
use bevy_auto_plugin_shared::module;

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
///
/// # Example
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin_module::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_type]
///     #[derive(Component)]
///     pub struct MyComponent;
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.register_type::<MyComponent>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let mut attrs = bevy_auto_plugin_shared::module::attribute::AutoPluginAttributes::default();
    let arg_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(attr with arg_parser);

    // Parse the input module
    let module = parse_macro_input!(input as ItemMod);

    let injected_module = match module::inner::auto_plugin_inner(module, &attrs.init_name()) {
        Ok(code) => code,
        Err(err) => return err.to_compile_error().into(),
    };

    CompilerStream::from(injected_module)
}

/// Automatically registers a type with the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin_module::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_type]
///     #[derive(Component, Reflect)]
///     #[reflect(Component)]
///     struct FooComponent;
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.register_type::<FooComponent>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_type(FooComponentWithGeneric<bool>)]
///     #[auto_register_type(FooComponentWithGeneric<u32>)]
///     #[derive(Component, Reflect)]
///     #[reflect(Component)]
///     struct FooComponentWithGeneric<T>(T);
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.register_type::<FooComponentWithGeneric<bool>>();
///         app.register_type::<FooComponentWithGeneric<u32>>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_register_type(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}
/// Automatically adds an event type to the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_add_event]
///     #[derive(Event, Reflect)]
///     struct FooEvent;
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.add_event::<FooEvent>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_add_event(FooEventWithGeneric<bool>)]
///     #[derive(Event, Reflect)]
///     struct FooEventWithGeneric<T>(T);
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {
///         app.add_event::<FooEventWithGeneric<bool>>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_add_event(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}
/// Automatically initializes a resource in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_init_resource]
///     #[derive(Resource, Default, Reflect)]
///     #[reflect(Resource)]
///     struct FooResource;
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.init_resource::<FooResource>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_init_resource(FooResourceWithGeneric<bool>)]
///     #[derive(Resource, Default, Reflect)]
///     #[reflect(Resource)]
///     struct FooResourceWithGeneric<T>(T);
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.init_resource::<FooResourceWithGeneric<bool>>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_init_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_type]
///     #[derive(Component, Reflect)]
///     #[reflect(Component)]
///     #[auto_name]
///     struct FooComponent;
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {
///         app.register_type::<FooComponent>();
///         app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
///
/// # Example (with generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_type(FooComponentWithGeneric<bool>)]
///     #[auto_register_type(FooComponentWithGeneric<u32>)]
///     #[derive(Component, Reflect)]
///     #[reflect(Component)]
///     #[auto_name(FooComponentWithGeneric<bool>)]
///     struct FooComponentWithGeneric<T>(T);
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.register_type::<FooComponentWithGeneric<bool>>();
///         app.register_type::<FooComponentWithGeneric<u32>>();
///         app.register_required_components_with::<FooComponentWithGeneric<boo>, Name>(|| Name::new("FooComponentWithGeneric<boo>"));
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_name(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a State in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_init_state]
///     #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
///     enum Foo {
///         #[default]
///         A,
///     }
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.init_state::<FooResource>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_init_state(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically registers a State<T> and NextState<T> in the Bevy `App`.
///
/// # Example (without generics)
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_auto_plugin::auto_plugin::*;
///
/// #[auto_plugin(init_name=init)]
/// pub mod my_plugin {
///     use super::*;
///
///     #[auto_register_state_type]
///     #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
///     enum Foo {
///         #[default]
///         A,
///     }
///
///     // code gen:
///     pub(super) fn init(app: &mut App) {  
///         app.register_type::<State<Foo>>();
///         app.register_type::<NextState<Foo>>();
///     }
/// }
///
/// fn plugin(app: &mut App) {
///     app.add_plugin(my_plugin::init)
/// }
/// ```
#[proc_macro_attribute]
pub fn module_auto_register_state_type(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/* INLINE */

#[cfg(feature = "missing_auto_plugin_check")]
use bevy_auto_plugin_shared::inline::file_state::files_missing_plugin_ts;
use bevy_auto_plugin_shared::util::{
    FnParamMutabilityCheckErrMessages, Target,
};
use bevy_auto_plugin_shared::{inline, util};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Item, ItemFn, Path, Token};
use quote::quote;
use proc_macro2::Span;
use bevy_auto_plugin_shared::inline::file_state::files_missing_plugin_ts;
use bevy_auto_plugin_shared::inline::inner::auto_plugin_inner;

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
pub fn inline_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let mut attrs = inline::attribute::AutoPluginAttributes::default();
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

    CompilerStream::from(auto_plugin_inner(input, app_param_name).unwrap_or_else(|err| err.to_compile_error()))
}

fn inline_handle_attribute(attr: CompilerStream, input: CompilerStream, target: Target) -> CompilerStream {
    let cloned_input = input.clone();
    let parsed_item = parse_macro_input!(input as Item);
    let args = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr with Punctuated::<Path, Token![,]>::parse_terminated))
    };

    inline::inner::handle_attribute_inner(
        inline::file_state::get_file_path(),
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
pub fn inline_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::RegisterTypes)
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
pub fn inline_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::AddEvents)
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
pub fn inline_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::InitResources)
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
pub fn inline_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::RequiredComponentAutoName)
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
pub fn inline_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::InitStates)
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
pub fn inline_auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    inline_handle_attribute(attr, input, Target::RegisterStateTypes)
}
