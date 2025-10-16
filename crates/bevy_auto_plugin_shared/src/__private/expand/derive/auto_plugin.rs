use crate::util::macros::{ok_or_emit, parse_macro_input2};
use proc_macro2::TokenStream as MacroStream;

// TODO: move
pub fn inject_send_sync_static(generics: &mut syn::Generics) {
    use syn::{Lifetime, Path, TraitBound, TraitBoundModifier, TypeParamBound, parse_quote};
    fn path_is_ident(path: &Path, name: &str) -> bool {
        path.segments.len() == 1 && path.segments[0].ident == name
    }
    for tp in generics.type_params_mut() {
        // Scan existing bounds so we don't duplicate them.
        let mut has_send = false;
        let mut has_sync = false;
        let mut has_static = false;

        for b in &tp.bounds {
            match b {
                TypeParamBound::Trait(TraitBound {
                    modifier: TraitBoundModifier::None,
                    path,
                    ..
                }) => {
                    if path_is_ident(path, "Send") {
                        has_send = true;
                    }
                    if path_is_ident(path, "Sync") {
                        has_sync = true;
                    }
                }
                TypeParamBound::Trait(TraitBound {
                    modifier: TraitBoundModifier::Maybe(_),
                    ..
                }) => {
                    // e.g. ?Sized — ignore
                }
                TypeParamBound::Lifetime(lt) => {
                    if lt == &Lifetime::new("'static", lt.apostrophe) {
                        has_static = true;
                    }
                }
                _ => {}
            }
        }

        if !has_send {
            tp.bounds.push(parse_quote!(::core::marker::Send));
        }
        if !has_sync {
            tp.bounds.push(parse_quote!(::core::marker::Sync));
        }
        if !has_static {
            tp.bounds.push(parse_quote!('static));
        }
    }
}

pub fn expand_derive_auto_plugin(input: MacroStream) -> MacroStream {
    use crate::macro_api::derives::auto_plugin::AutoPluginDeriveArgs;
    use darling::FromDeriveInput;
    use quote::ToTokens;
    use quote::quote;
    use syn::DeriveInput;

    let derive_input = parse_macro_input2!(input as DeriveInput);
    let params = {
        let mut params = match AutoPluginDeriveArgs::from_derive_input(&derive_input) {
            Ok(params) => params,
            Err(err) => return err.write_errors(),
        };
        inject_send_sync_static(&mut params.generics);
        params
    };

    let ident = &params.ident; // `Test`
    let generics = &params.generics; // `<T1, T2>`
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = MacroStream::new();

    let mut auto_plugin_implemented = false;

    if params.auto_plugin.impl_plugin_trait {
        let full_names = if params.auto_plugin.generics.is_empty() {
            vec![ident.to_string()]
        } else {
            params
                .auto_plugin
                .generics
                .iter()
                .map(|tl| format!("{}::<{}>", ident, tl.to_token_stream()))
                .collect()
        };
        for full_name in full_names {
            let path_with_generics = ok_or_emit!(syn::parse_str::<syn::Path>(&full_name));

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::Plugin for #path_with_generics {
                    fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::App) {
                        <Self as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin>::build(self, app);
                    }
                }

                impl ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if params.auto_plugin.impl_generic_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::Plugin
                for #ident #ty_generics #where_clause
            {
                fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::App) {
                    <Self as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin>::build(self, app);
                }
            }
        });
    }

    // TODO: maybe default to this behavior
    if params.auto_plugin.impl_generic_auto_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin
                for #ident #ty_generics #where_clause
            {}
        });
    } else if !auto_plugin_implemented {
        auto_plugin_implemented = true;

        let full_names = if params.auto_plugin.generics.is_empty() {
            vec![ident.to_string()]
        } else {
            params
                .auto_plugin
                .generics
                .iter()
                .map(|tl| format!("{}::<{}>", ident, tl.to_token_stream()))
                .collect()
        };
        for full_name in full_names {
            let path_with_generics = match syn::parse_str::<syn::Path>(&full_name) {
                Ok(p) => p,
                Err(err) => return err.into_compile_error(),
            };

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if auto_plugin_implemented {
        // satisfy linter #[warn(unused_assignments)]
    }

    output
}
