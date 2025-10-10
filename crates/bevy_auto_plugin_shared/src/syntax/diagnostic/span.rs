use proc_macro2::Span;

pub fn get_or_create_span_from_result<T>(e: &syn::Result<T>) -> Span {
    match e {
        Ok(_) => Span::call_site(),
        Err(e) => e.span(),
    }
}
