use crate::type_list::TypeList;
use crate::util::{PathExt, path_to_string, path_to_string_with_spaces};
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::{Ident, Span, TokenStream as MacroStream, TokenStream};
use quote::quote;
use std::collections::HashSet;
use std::hash::Hash;
use syn::{Attribute, Generics, Path, Type, Visibility, parse2};

pub mod flat_file;
pub mod global;
pub mod module;
mod type_list;
pub mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginAttribute {
    RegisterType,
    AddEvent,
    InitResource,
    InitState,
    Name,
    RegisterStateType,
    AddSystem,
}

impl AutoPluginAttribute {
    pub const fn ident_str(self) -> &'static str {
        match self {
            Self::RegisterType => "auto_register_type",
            Self::AddEvent => "auto_add_event",
            Self::InitResource => "auto_init_resource",
            Self::InitState => "auto_init_state",
            Self::Name => "auto_name",
            Self::RegisterStateType => "auto_register_state_type",
            Self::AddSystem => "auto_add_system",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromField)]
pub struct GlobalAutoPluginField {
    ident: Option<Ident>,
    ty: Type,
}

#[allow(dead_code)]
#[derive(Debug, FromVariant)]
pub struct GlobalAutoPluginVariant {
    ident: Ident,
    fields: darling::ast::Fields<GlobalAutoPluginField>,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(auto_plugin), forward_attrs, supports(struct_any, enum_any))]
pub struct GlobalAutoPluginDeriveParams {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: darling::ast::Data<GlobalAutoPluginVariant, GlobalAutoPluginField>,
    pub attrs: Vec<Attribute>,
    #[darling(flatten)]
    pub auto_plugin: GlobalAutoPluginStructOrEnumAttributeParams,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginStructOrEnumAttributeParams {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub impl_plugin_trait: bool,
    pub impl_generic_auto_plugin_trait: bool,
    pub impl_generic_plugin_trait: bool,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginFnAttributeParams {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

fn hash_global_struct_or_enum_attribute_params(
    ident: &Ident,
    params: &GlobalStructOrEnumAttributeParams,
) -> String {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    ident.hash(&mut hasher);
    params.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn get_unique_ident_string_for_global_struct_or_enum_attribute(
    prefix: &'static str,
    ident: &Ident,
    params: &GlobalStructOrEnumAttributeParams,
) -> String {
    let hash = hash_global_struct_or_enum_attribute_params(ident, params);
    format!("{prefix}_{hash}")
}

pub fn get_unique_ident_for_global_struct_or_enum_attribute(
    prefix: &'static str,
    ident: &Ident,
    params: &GlobalStructOrEnumAttributeParams,
) -> Ident {
    let ident_string =
        get_unique_ident_string_for_global_struct_or_enum_attribute(prefix, ident, params);
    Ident::new(&ident_string, ident.span())
}

#[derive(FromMeta, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalStructOrEnumAttributeParams {
    pub plugin: Path,
    #[darling(flatten, default)]
    pub inner: StructOrEnumAttributeParams,
}

impl GlobalStructOrEnumAttributeParams {
    pub fn has_generics(&self) -> bool {
        self.inner.has_generics()
    }
}

#[derive(FromMeta, Debug, Default, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StructOrEnumAttributeParams {
    // TODO: #[darling(multiple)]
    //     pub generics: Vec<TypeList>,
    pub generics: Option<TypeList>,
}

impl StructOrEnumAttributeParams {
    pub fn has_generics(&self) -> bool {
        self.generics
            .as_ref()
            .map(|types| !types.0.is_empty())
            .unwrap_or(false)
    }
}

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct ScheduleConfigParams {
    pub schedule: Path,
    pub in_set: Option<Path>,
    pub before: Option<Path>,
    pub after: Option<Path>,
    pub run_if: Option<Path>,
    pub distributive_run_if: Option<Path>,
    pub ambiguous_with: Option<Path>,
    pub ambiguous_with_all: Option<bool>,
    pub after_ignore_deferred: Option<Path>,
    pub before_ignore_deferred: Option<Path>,
}

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct AddSystemParams {
    pub generics: Option<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleConfigParams,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ScheduleConfigSerializedParams {
    pub schedule_path_string: String,
    pub scheduled_item_path_string: String,
    pub in_set_path_string: Option<String>,
    pub before_path_string: Option<String>,
    pub after_path_string: Option<String>,
    pub run_if_path_string: Option<String>,
    pub distributive_run_if_path_string: Option<String>,
    pub ambiguous_with_path_string: Option<String>,
    pub ambiguous_with_all_flag: Option<bool>,
    pub after_ignore_deferred_path_string: Option<String>,
    pub before_ignore_deferred_path_string: Option<String>,
}

impl ScheduleConfigSerializedParams {
    pub fn from_macro_attr(system: &Path, attr: &ScheduleConfigParams) -> Self {
        Self {
            schedule_path_string: path_to_string_with_spaces(&attr.schedule),
            scheduled_item_path_string: path_to_string_with_spaces(system),
            in_set_path_string: attr.in_set.as_ref().map(path_to_string_with_spaces),
            before_path_string: attr.before.as_ref().map(path_to_string_with_spaces),
            after_path_string: attr.after.as_ref().map(path_to_string_with_spaces),
            run_if_path_string: attr.run_if.as_ref().map(path_to_string_with_spaces),
            distributive_run_if_path_string: attr
                .distributive_run_if
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_path_string: attr
                .ambiguous_with
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_all_flag: attr.ambiguous_with_all,
            before_ignore_deferred_path_string: attr
                .before_ignore_deferred
                .as_ref()
                .map(path_to_string_with_spaces),
            after_ignore_deferred_path_string: attr
                .after_ignore_deferred
                .as_ref()
                .map(path_to_string_with_spaces),
        }
    }
    pub fn to_tokens(&self) -> syn::Result<MacroStream> {
        let mut output = quote! {};
        if let Some(in_set) = &self.in_set_path_string {
            let in_set = syn::parse_str::<Path>(in_set)?;
            output = quote! {
                #output
                    .in_set(#in_set)
            };
        }
        if let Some(before) = &self.before_path_string {
            let before = syn::parse_str::<Path>(before)?;
            output = quote! {
                #output
                    .before(#before)
            };
        }
        if let Some(after) = &self.after_path_string {
            let after = syn::parse_str::<Path>(after)?;
            output = quote! {
                #output
                    .after(#after)
            }
        }
        if let Some(run_if) = &self.run_if_path_string {
            let run_if = syn::parse_str::<Path>(run_if)?;
            output = quote! {
                #output
                    .run_if(#run_if)
            }
        }
        if let Some(distributive_run_if) = &self.distributive_run_if_path_string {
            let distributive_run_if = syn::parse_str::<Path>(distributive_run_if)?;
            output = quote! {
                #output
                    .distributive_run_if(#distributive_run_if)
            }
        }
        if let Some(ambiguous_with) = &self.ambiguous_with_path_string {
            let ambiguous_with = syn::parse_str::<Path>(ambiguous_with)?;
            output = quote! {
                #output
                    .ambiguous_with(#ambiguous_with)
            }
        }
        if let Some(true) = self.ambiguous_with_all_flag {
            output = quote! {
                #output
                    .ambiguous_with_all()
            }
        }
        if let Some(before_ignore_deferred) = &self.before_ignore_deferred_path_string {
            let before_ignore_deferred = syn::parse_str::<Path>(before_ignore_deferred)?;
            output = quote! {
                #output
                    .before_ignore_deferred(#before_ignore_deferred)
            }
        }
        if let Some(after_ignore_deferred) = &self.after_ignore_deferred_path_string {
            let after_ignore_deferred = syn::parse_str::<Path>(after_ignore_deferred)?;
            output = quote! {
                #output
                    .after_ignore_deferred(#after_ignore_deferred)
            }
        }
        Ok(output)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AddSystemSerializedParams {
    pub schedule_config: ScheduleConfigSerializedParams,
}

impl AddSystemSerializedParams {
    pub fn from_macro_attr(system: &Path, attr: &AddSystemParams) -> Self {
        let system_path_tokens = if system.has_generics() {
            quote! { #system }
        } else {
            let generics = attr
                .generics
                .as_ref()
                .map(TokenStream::from)
                .unwrap_or_default();
            quote! { #system::<#generics> }
        };
        // TODO: return result
        let system = parse2::<Path>(system_path_tokens).expect("failed to parse system path");

        Self {
            schedule_config: ScheduleConfigSerializedParams::from_macro_attr(
                &system,
                &attr.schedule_config,
            ),
        }
    }
    pub fn to_tokens(&self, app_ident: &Ident) -> syn::Result<MacroStream> {
        let config_tokens = self.schedule_config.to_tokens()?;
        let schedule = syn::parse_str::<Path>(&self.schedule_config.schedule_path_string)?;
        let system = syn::parse_str::<Path>(&self.schedule_config.scheduled_item_path_string)?;
        Ok(quote! {
            #app_ident.add_systems(#schedule, #system #config_tokens);
        })
    }
}

#[derive(Default)]
pub struct AutoPluginContext {
    pub register_types: HashSet<String>,
    pub register_state_types: HashSet<String>,
    pub add_events: HashSet<String>,
    pub init_resources: HashSet<String>,
    pub init_states: HashSet<String>,
    pub auto_names: HashSet<String>,
    pub add_systems: HashSet<AddSystemSerializedParams>,
}

pub fn generate_register_type(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    Ok(quote! {
        #app_ident.register_type::<#item>();
    })
}

pub fn generate_register_types(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let register_types = items
        .map(|item| generate_register_type(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // register_types
            #(#register_types)*
        }
    })
}

pub fn generate_add_event(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    Ok(quote! {
        #app_ident.add_event::<#item>();
    })
}

pub fn generate_add_events(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let add_events = items
        .map(|item| generate_add_event(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // add_events
            #(#add_events)*
        }
    })
}

pub fn generate_init_resource(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    Ok(quote! {
        #app_ident.init_resource::<#item>();
    })
}

pub fn generate_init_resources(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let init_resources = items
        .map(|item| generate_init_resource(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // init_resources
            #(#init_resources)*
        }
    })
}

pub fn generate_auto_name(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    let name = path_to_string(&item, true).replace("::", "");
    Ok(quote! {
        #app_ident.register_required_components_with::<#item, Name>(|| Name::new(#name));
    })
}

pub fn generate_auto_names(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let auto_names = items
        .map(|item| generate_auto_name(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // auto_names
            #(#auto_names)*
        }
    })
}

pub fn generate_register_state_type(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    Ok(quote! {
        #app_ident.register_type::<State<#item>>();
        #app_ident.register_type::<NextState<#item>>();
    })
}

pub fn generate_register_state_types(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let register_state_types = items
        .map(|item| generate_register_state_type(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // register_state_types
            #(#register_state_types)*
        }
    })
}

pub fn generate_init_state(app_ident: &Ident, item: String) -> syn::Result<MacroStream> {
    let item = syn::parse_str::<Path>(&item)?;
    Ok(quote! {
        #app_ident.init_state::<#item>();
    })
}

pub fn generate_init_states(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let init_states = items
        .map(|item| generate_init_state(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // init_states
            #(#init_states)*
        }
    })
}

pub fn generate_add_system(
    app_ident: &Ident,
    item: AddSystemSerializedParams,
) -> syn::Result<MacroStream> {
    item.to_tokens(app_ident)
}

pub fn generate_add_systems(
    app_ident: &Ident,
    items: impl Iterator<Item = AddSystemSerializedParams>,
) -> syn::Result<MacroStream> {
    let output = items
        .map(|item| generate_add_system(app_ident, item))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // add_systems
            #(#output)*
        }
    })
}

pub fn default_app_ident() -> Ident {
    Ident::new("app", Span::call_site())
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn __wasm_call_ctors();
}

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}
