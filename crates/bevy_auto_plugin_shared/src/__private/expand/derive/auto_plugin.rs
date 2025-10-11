use crate::util::macros::parse_macro_input2;
use proc_macro2::TokenStream as MacroStream;

pub fn expand_derive_auto_plugin(input: MacroStream) -> MacroStream {
    use crate::macro_api::derives::auto_plugin::AutoPluginDeriveArgs;
    use darling::FromDeriveInput;
    use quote::ToTokens;
    use quote::quote;
    use syn::DeriveInput;

    let derive_input = parse_macro_input2!(input as DeriveInput);
    let params = match AutoPluginDeriveArgs::from_derive_input(&derive_input) {
        Ok(params) => params,
        Err(err) => return err.write_errors(),
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
            let path_with_generics = match syn::parse_str::<syn::Path>(&full_name) {
                Ok(p) => p,
                Err(err) => return err.into_compile_error(),
            };

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
