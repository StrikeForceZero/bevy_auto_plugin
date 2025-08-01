use crate::AddSystemSerializedParams;
use crate::util::path_to_string;
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use syn::Path;

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
        /// generated register_types
        {
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
        /// generated add_events
        {
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
        /// generated init_resources
        {
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
        /// generated auto_names
        {
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
        /// generated register_state_types
        {
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
        /// generated init_states
        {
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
        /// generated add_systems
        {
            #(#output)*
        }
    })
}
