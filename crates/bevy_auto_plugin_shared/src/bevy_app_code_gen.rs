use crate::attribute_args::{AddSystemWithTargetArgs, InsertResourceArgsWithPath};
use crate::util::path_fmt::path_to_string;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::quote;
use syn::Path;

/// Generic code-gen for iterator of items passing each item to build param fn
#[inline]
fn gen_many<I, F, P>(app_ident: &Ident, items: I, doc: &str, build: F) -> syn::Result<MacroStream>
where
    I: IntoIterator<Item = P>,
    F: Fn(&Ident, P) -> syn::Result<MacroStream>,
{
    let tokens = items
        .into_iter()
        .map(|item| build(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        #[doc = #doc]
        { #(#tokens)* }
    })
}

/// Custom Single block app_ident method call impl
macro_rules! impl_custom {
    // with conversion
    (
        $single:ident,        // fn name for the “single” helper
        $plural:ident,        // fn name for the “plural” helper
        $doc:literal,         // doc-comment for the plural code-gen block
        $item_ty:ty,          // the type that callers pass in
        $conversion:expr,     // <-- user-supplied conversion
        $build:expr $(,)?     // |app_ident, item| -> TokenStream
    ) => {
        impl_custom!(@impl $single, $plural, $doc, $item_ty, $conversion, $build);
    };

    // without conversion
    (
        $single:ident,
        $plural:ident,
        $doc:literal,
        $item_ty:ty,
        $build:expr $(,)?
    ) => {
        // Identity conversion: just return the item unchanged
        impl_custom!(@impl
            $single, $plural, $doc, $item_ty,
            |_, item| syn::Result::Ok(item),   // default conversion
            $build
        );
    };

    // base
    (@impl
        $single:ident,
        $plural:ident,
        $doc:literal,
        $item_ty:ty,
        $conversion:expr,
        $build:expr $(,)?
    ) => {
        pub fn $single(app_ident: &Ident, item: $item_ty) -> syn::Result<MacroStream> {
            let item = $conversion(app_ident, item)?;
            $build(app_ident, item)
        }

        pub fn $plural<I>(app_ident: &Ident, items: I) -> syn::Result<MacroStream>
        where
            I: IntoIterator<Item = $item_ty>,
        {
            gen_many(app_ident, items, $doc, |app_ident, item| $single(app_ident, item))
        }
    };
}

/// Simple single block app_ident method call impl
///
/// Generated functions accept [`String`] and attempt to parse it into [`Path`] as a validity check before returning [`TokenStream`]
macro_rules! impl_simple {
    (
        $single:ident, // fn name for the single-item version
        $plural:ident, // fn name for the plural-item version
        $doc:literal, // doc comment for the plural version
        $build:expr, // |app_ident, path| { ... } → TokenStream
    ) => {
        impl_custom!($single, $plural, $doc, syn::Path, |app_ident, item| Ok(
            $build(app_ident, &item)
        ),);
    };
}

// ── generators ──────────────────────────────────────────────────────────────
impl_simple!(
    generate_register_type,
    generate_register_types,
    "generated register_types",
    |app_ident, path| quote! { #app_ident.register_type::<#path>(); },
);

impl_simple!(
    generate_add_event,
    generate_add_events,
    "generated add_events",
    |app_ident, path| quote! { #app_ident.add_event::<#path>(); },
);

impl_simple!(
    generate_init_resource,
    generate_init_resources,
    "generated init_resources",
    |app_ident, path| quote! { #app_ident.init_resource::<#path>(); },
);

impl_custom!(
    generate_insert_resource,
    generate_insert_resources,
    "generated insert_resources",
    InsertResourceArgsWithPath,
    |app_ident, item| InsertResourceArgsWithPath::to_tokens(item, app_ident),
);

impl_simple!(
    generate_register_state_type,
    generate_register_state_types,
    "generated register_state_types",
    |app_ident, path| quote! {
        #app_ident.register_type::<State<#path>>();
        #app_ident.register_type::<NextState<#path>>();
    },
);

impl_simple!(
    generate_init_state,
    generate_init_states,
    "generated init_states",
    |app_ident, path| quote! { #app_ident.init_state::<#path>(); },
);

impl_simple!(
    generate_auto_name,
    generate_auto_names,
    "generated auto_names",
    |app_ident, path| {
        let name = path_to_string(path, true).replace("::", "");
        quote! { #app_ident.register_required_components_with::<#path, Name>(|| Name::new(#name)); }
    },
);

impl_custom!(
    generate_add_system,
    generate_add_systems,
    "generated add_systems",
    AddSystemWithTargetArgs,
    |app_ident, item| AddSystemWithTargetArgs::to_tokens(item, app_ident),
);

/// code-gen input data
#[derive(Default)]
pub struct InputSets {
    pub register_types: Vec<Path>,
    pub register_state_types: Vec<Path>,
    pub auto_names: Vec<Path>,
    pub add_events: Vec<Path>,
    pub add_systems: Vec<AddSystemWithTargetArgs>,
    pub insert_resources: Vec<InsertResourceArgsWithPath>,
    pub init_states: Vec<Path>,
    pub init_resources: Vec<Path>,
}

/// Expands codegen from [`InputSets`]
pub fn expand_input_sets(app_ident: &Ident, sets: InputSets) -> syn::Result<TokenStream> {
    let register_types = generate_register_types(app_ident, sets.register_types)?;
    let register_state_types = generate_register_state_types(app_ident, sets.register_state_types)?;
    let auto_names = generate_auto_names(app_ident, sets.auto_names)?;
    let add_events = generate_add_events(app_ident, sets.add_events)?;
    let add_systems = generate_add_systems(app_ident, sets.add_systems)?;
    let insert_resources = generate_insert_resources(app_ident, sets.insert_resources)?;
    let init_states = generate_init_states(app_ident, sets.init_states)?;
    let init_resources = generate_init_resources(app_ident, sets.init_resources)?;

    Ok(quote! {
        #register_types
        #register_state_types
        #auto_names
        #add_events
        #add_systems
        #insert_resources
        #init_states
        #init_resources
    })
}
