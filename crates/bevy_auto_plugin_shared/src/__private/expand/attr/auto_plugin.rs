use crate::util::macros::{
    compile_error_with,
    ok_or_emit_with,
    parse_macro_input2_or_emit_with,
};
use proc_macro2::TokenStream as MacroStream;
use syn::ItemFn;

pub fn expand_auto_plugin(attr: MacroStream, input: MacroStream) -> MacroStream {
    use crate::{
        macro_api::prelude::*,
        syntax::analysis::fn_param::require_fn_param_mutable_reference,
    };
    use proc_macro2::Ident;
    use quote::quote;
    use syn::{
        FnArg,
        parse2,
        spanned::Spanned,
    };
    let og_input = input.clone();
    let item = parse_macro_input2_or_emit_with!(input as ItemFn, og_input);
    let params = ok_or_emit_with!(parse2::<AutoPluginFnArgs>(attr), og_input);
    let vis = &item.vis;
    let attrs = &item.attrs;
    let sig = &item.sig;
    let fn_ident = &sig.ident;
    let block = &item.block;
    let inputs = &sig.inputs;
    let self_args = inputs
        .into_iter()
        .flat_map(|input| match input {
            FnArg::Receiver(recv) => Some(Ident::new("self", recv.span())),
            FnArg::Typed(_) => None,
        })
        .take(1)
        .collect::<Vec<_>>();
    let self_arg = self_args.first();

    // TODO: use helper
    let app_param_ident =
        ok_or_emit_with!(resolve_app_param_name(&item, params.app_param.as_ref()), og_input);

    if let Err(err) = require_fn_param_mutable_reference(&item, app_param_ident, "bevy app") {
        return compile_error_with!(err, og_input);
    }

    let mut impl_plugin = quote! {};

    let auto_plugin_hook = if let Some(self_arg) = self_arg {
        if params.plugin.is_some() {
            return compile_error_with!(
                syn::Error::new(
                    params.plugin.span(),
                    "auto_plugin on trait impl can't specify plugin ident",
                ),
                og_input
            );
        };
        quote! {
            <Self as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin>::build(#self_arg, #app_param_ident);
        }
    } else {
        if sig.inputs.len() > 1 {
            return compile_error_with!(
                syn::Error::new(
                    sig.inputs.span(),
                    "auto_plugin on bare fn can only accept a single parameter with the type &mut bevy::prelude::App",
                ),
                og_input
            );
        }
        let Some(plugin_ident) = params.plugin else {
            return compile_error_with!(
                syn::Error::new(
                    params.plugin.span(),
                    "auto_plugin on bare fn requires the plugin ident to be specified",
                ),
                og_input
            );
        };
        impl_plugin.extend(quote! {
            impl ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::Plugin for #plugin_ident {
                fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::bevy_app::App) {
                    #fn_ident(app);
                }
            }
        });
        quote! {
            <#plugin_ident as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPlugin>::static_build(#app_param_ident);
        }
    };

    quote! {
        #(#attrs)*
        #vis #sig
        {
            #auto_plugin_hook
            #block
        }

        #impl_plugin
    }
}
