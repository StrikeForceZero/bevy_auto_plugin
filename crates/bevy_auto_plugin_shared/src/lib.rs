use crate::type_list::TypeList;
use crate::util::{path_to_string, path_to_string_with_spaces};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::quote;
use std::collections::HashSet;
use syn::Path;

pub mod flat_file;
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

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct StructOrEnumAttributeParams {
    pub generics: Option<TypeList>,
}

impl StructOrEnumAttributeParams {
    pub fn has_generics(&self) -> bool {
        self.generics
            .as_ref()
            .map(|types| types.0.len() > 0)
            .unwrap_or(false)
    }
}

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct AddSystemParams {
    pub schedule: Path,
    pub generics: Option<TypeList>,
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AddSystemSerializedParams {
    pub schedule_path_string: String,
    pub system_ident_string: String,
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

impl AddSystemSerializedParams {
    pub fn from_macro_attr(system: &Path, attr: &AddSystemParams) -> Self {
        let generics = attr
            .generics
            .as_ref()
            .map(TokenStream::from)
            .unwrap_or_default();
        let system_ident_string = quote! { #system::<#generics> };

        Self {
            schedule_path_string: path_to_string_with_spaces(&attr.schedule),
            system_ident_string: system_ident_string.to_string(),
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
    pub fn to_tokens(&self, app_ident: &Ident) -> syn::Result<MacroStream> {
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
        let schedule = syn::parse_str::<Path>(&self.schedule_path_string)?;
        let system = syn::parse_str::<Path>(&self.system_ident_string)?;
        Ok(quote! {
            #app_ident.add_systems(#schedule, #system #output);
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

pub fn generate_register_types(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let register_types = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            Ok(quote! {
                #app_ident.register_type::<#item>();
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // register_types
            #(#register_types)*
        }
    })
}

pub fn generate_add_events(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let add_events = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            Ok(quote! {
                #app_ident.add_event::<#item>();
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // add_events
            #(#add_events)*
        }
    })
}

pub fn generate_init_resources(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let init_resources = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            Ok(quote! {
                #app_ident.init_resource::<#item>();
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // init_resources
            #(#init_resources)*
        }
    })
}

pub fn generate_auto_names(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let auto_names = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            let name = path_to_string(&item, true)
                // TODO: flag
                .replace("::", "");
            Ok(quote! {
                #app_ident.register_required_components_with::<#item, Name>(|| Name::new(#name));
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // auto_names
            #(#auto_names)*
        }
    })
}

pub fn generate_register_state_types(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let register_state_types = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            Ok(quote! {
                #app_ident.register_type::<State<#item>>();
                #app_ident.register_type::<NextState<#item>>();
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // register_state_types
            #(#register_state_types)*
        }
    })
}

pub fn generate_init_states(
    app_ident: &Ident,
    items: impl Iterator<Item = String>,
) -> syn::Result<MacroStream> {
    let init_states = items
        .map(|item| {
            let item = syn::parse_str::<Path>(&item)?;
            Ok(quote! {
                #app_ident.init_state::<#item>();
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // init_states
            #(#init_states)*
        }
    })
}

pub fn generate_add_systems(
    app_ident: &Ident,
    items: impl Iterator<Item = AddSystemSerializedParams>,
) -> syn::Result<MacroStream> {
    let output = items
        .map(|item| item.to_tokens(app_ident))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        {
            // register systems
            #(#output)*
        }
    })
}
