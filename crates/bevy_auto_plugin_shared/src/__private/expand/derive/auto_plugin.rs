use crate::util::macros::parse_macro_input2;
use proc_macro2::TokenStream as MacroStream;
use syn::spanned::Spanned;

pub fn expand_derive_auto_plugin(input: MacroStream) -> MacroStream {
    use crate::macro_api::prelude::*;
    use crate::syntax::extensions::generics;
    use darling::FromDeriveInput;
    use quote::quote;
    use syn::DeriveInput;

    let derive_input = parse_macro_input2!(input as DeriveInput);
    let params = {
        let mut params = match AutoPluginDeriveArgs::from_derive_input(&derive_input) {
            Ok(params) => params,
            Err(err) => return err.write_errors(),
        };
        generics::inject_send_sync_static(&mut params.generics);
        params
    };

    let mut compile_warnings = quote! {};

    #[allow(deprecated)]
    if params
        .auto_plugin
        .impl_generic_auto_plugin_trait
        .is_present()
    {
        compile_warnings.extend(
            syn::Error::new(
                params.auto_plugin.impl_generic_auto_plugin_trait.span(),
                "always implemented - remove `impl_generic_auto_plugin_trait`",
            )
            .to_compile_error(),
        )
    }

    #[allow(deprecated)]
    if params.auto_plugin.impl_generic_plugin_trait.is_present() {
        compile_warnings.extend(
            syn::Error::new(
                params.auto_plugin.impl_generic_plugin_trait.span(),
                "use `impl_plugin_trait` instead",
            )
            .to_compile_error(),
        )
    }

    #[allow(deprecated)]
    for generics in &params.auto_plugin.generics {
        compile_warnings.extend(
            syn::Error::new(generics.span(), "no longer needed - remove `generics(...)`")
                .to_compile_error(),
        )
    }

    let ident = &params.ident; // `Test`
    let generics = &params.generics; // `<T1, T2>`
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = quote! {
        impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin
            for #ident #ty_generics #where_clause
        {}
    };

    if params.auto_plugin.impl_plugin_trait.is_present() {
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

    quote! {
        #compile_warnings
        #output
    }
}
