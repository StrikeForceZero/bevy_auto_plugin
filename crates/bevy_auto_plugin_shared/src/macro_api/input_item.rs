use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};
use syn::{
    parse2,
    spanned::Spanned,
};

#[derive(Debug, Clone)]
pub enum InputItem {
    Tokens(TokenStream),
    Item(Box<syn::Item>),
}

impl PartialEq for InputItem {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = self.to_token_stream();
        let other_tokens = other.to_token_stream();
        self_tokens.to_string() == other_tokens.to_string()
    }
}

impl InputItem {
    fn _upgrade(&mut self) -> syn::Result<()> {
        if let Self::Tokens(tokens) = self {
            *self = Self::Item(parse2(tokens.clone())?);
        }
        Ok(())
    }
    pub fn ensure_ast(&mut self) -> syn::Result<&syn::Item> {
        self._upgrade()?;
        Ok(match &*self {
            Self::Item(item) => item.as_ref(),
            _ => unreachable!(),
        })
    }
    pub fn ensure_ast_mut(&mut self) -> syn::Result<&mut syn::Item> {
        self._upgrade()?;
        Ok(match &mut *self {
            Self::Item(item) => item.as_mut(),
            _ => unreachable!(),
        })
    }
    // TODO: use instead of analysis/item helpers
    pub fn get_ident(&mut self) -> syn::Result<Option<&syn::Ident>> {
        let item = self.ensure_ast()?;
        Ok(item.get_ident())
    }
    pub fn ident(&mut self) -> syn::Result<&syn::Ident> {
        self.ensure_ast().and_then(|item| {
            item.get_ident()
                .ok_or_else(|| syn::Error::new(item.span(), "expected item to have an ident"))
        })
    }
    pub fn map_ast<F>(&mut self, f: F) -> syn::Result<()>
    where
        F: FnOnce(&mut syn::Item) -> syn::Result<()>,
    {
        let item = self.ensure_ast_mut()?;
        f(item)
    }
}

impl ToTokens for InputItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            InputItem::Tokens(ts) => ts.clone(),
            InputItem::Item(item) => quote! { #item },
        })
    }
}
