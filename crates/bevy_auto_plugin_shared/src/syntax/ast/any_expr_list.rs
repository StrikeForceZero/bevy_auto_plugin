#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnyExprList<T>(pub Vec<T>);

impl<T> AnyExprList<T> {
    pub fn iter(&self) -> ::std::slice::Iter<'_, T> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> ::quote::ToTokens for AnyExprList<T>
where
    T: ::quote::ToTokens,
{
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let items = &self.0;
        tokens.extend(::quote::quote! { #(#items),* });
    }
}

impl<T> From<&AnyExprList<T>> for ::proc_macro2::TokenStream
where
    T: ::quote::ToTokens,
{
    fn from(list: &AnyExprList<T>) -> Self {
        use ::quote::ToTokens;
        let mut ts = ::proc_macro2::TokenStream::new();
        list.to_tokens(&mut ts);
        ts
    }
}

impl<T> From<AnyExprList<T>> for ::proc_macro2::TokenStream
where
    T: ::quote::ToTokens,
{
    fn from(list: AnyExprList<T>) -> Self {
        (&list).into()
    }
}
