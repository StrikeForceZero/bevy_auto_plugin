use crate::attribute_args::GlobalMacroArgs;
use crate::global::__internal::_plugin_entry_block;
use crate::{default_app_ident, ok_or_return_compiler_error};
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use syn::Item;

fn global_attribute_inner<A, F>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    require: fn(&Item) -> syn::Result<&Ident>,
    parse_attr: fn(MacroStream) -> syn::Result<A>,
    body: F,
) -> MacroStream
where
    A: GlobalMacroArgs,
    F: FnOnce(&Ident, A, &Item) -> syn::Result<MacroStream>,
{
    let attr = attr.into();
    let input = input.into();

    let item: Item = ok_or_return_compiler_error!(syn::parse2(input.clone()));

    let ident = ok_or_return_compiler_error!(require(&item));

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote!( #item #output )
}

pub fn global_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    prefix: &'static str,
    require: fn(&Item) -> syn::Result<&Ident>,
    generate_fn: impl FnOnce(&Ident, <T as GlobalMacroArgs>::Input) -> syn::Result<MacroStream>,
) -> MacroStream
where
    T: GlobalMacroArgs,
{
    global_attribute_inner(
        attr,
        input,
        require,
        syn::parse2::<T>,
        |ident, params, _item| {
            let unique_ident = params.get_unique_ident(prefix, ident);
            let plugin = params.plugin().clone();
            let input = params.to_input(ident);
            let app_ident = default_app_ident();
            let register = generate_fn(&app_ident, input)?;
            let expr: syn::ExprClosure = syn::parse_quote!(|#app_ident| { #register });
            Ok(_plugin_entry_block(&unique_ident, &plugin, &expr))
        },
    )
}
