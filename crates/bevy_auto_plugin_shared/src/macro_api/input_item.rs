use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse2;

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
    pub fn get_ident(&mut self) -> syn::Result<Option<&syn::Ident>> {
        use syn::Item;
        let item = self.ensure_ast()?;
        Ok(match item {
            Item::Const(item) => Some(&item.ident),
            Item::Enum(item) => Some(&item.ident),
            Item::ExternCrate(item) => Some(&item.ident),
            Item::Fn(item) => Some(&item.sig.ident),
            Item::ForeignMod(_) => None,
            Item::Impl(_) => None,
            Item::Macro(item) => item.ident.as_ref(),
            Item::Mod(item) => Some(&item.ident),
            Item::Static(item) => Some(&item.ident),
            Item::Struct(item) => Some(&item.ident),
            Item::Trait(item) => Some(&item.ident),
            Item::TraitAlias(item) => Some(&item.ident),
            Item::Type(item) => Some(&item.ident),
            Item::Union(item) => Some(&item.ident),
            // TODO: implement
            Item::Use(_) => None,
            Item::Verbatim(_) => None,
            _ => None,
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
