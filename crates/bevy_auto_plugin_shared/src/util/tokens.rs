use proc_macro2::TokenStream as MacroStream;

pub fn to_compile_error(err: syn::Error) -> MacroStream {
    err.to_compile_error()
}
