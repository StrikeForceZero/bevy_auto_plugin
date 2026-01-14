use crate::syntax::{
    extensions::item::ItemAttrsExt,
    parse::item::{
        SingleItemWithErrorsCheckError,
        expect_single_item_any_compile_errors,
    },
};
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

impl AsRef<Self> for InputItem {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<Box<syn::Item>> for InputItem {
    fn from(boxed_item: Box<syn::Item>) -> Self {
        Self::Item(boxed_item)
    }
}

impl From<&syn::Item> for InputItem {
    fn from(item: &syn::Item) -> Self {
        Self::Item(Box::new(item.clone()))
    }
}

impl From<syn::Item> for InputItem {
    fn from(item: syn::Item) -> Self {
        Self::Item(Box::new(item))
    }
}

impl TryFrom<TokenStream> for InputItem {
    type Error = syn::Error;
    fn try_from(tokens: TokenStream) -> Result<Self, Self::Error> {
        Self::from_ts_validated(tokens)
    }
}

impl TryFrom<&TokenStream> for InputItem {
    type Error = syn::Error;
    fn try_from(tokens: &TokenStream) -> Result<Self, Self::Error> {
        Self::from_ts_validated(tokens.clone())
    }
}

impl PartialEq for InputItem {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = self.to_token_stream();
        let other_tokens = other.to_token_stream();
        self_tokens.to_string() == other_tokens.to_string()
    }
}

impl InputItem {
    pub fn from_ts_validated(tokens: TokenStream) -> syn::Result<Self> {
        let mut input_item = Self::Tokens(tokens);
        input_item.ensure_ast()?;
        Ok(input_item)
    }
    fn _upgrade(&mut self) -> syn::Result<()> {
        if let Self::Tokens(tokens) = self {
            *self = Self::Item(parse2(tokens.clone())?);
        }
        Ok(())
    }
    pub fn has_compiler_errors(&self) -> Result<bool, SingleItemWithErrorsCheckError> {
        match self {
            InputItem::Tokens(tokens) => {
                match expect_single_item_any_compile_errors(tokens.clone()) {
                    Ok((_, compiler_errors)) => Ok(!compiler_errors.is_empty()),
                    Err(e) => Err(e),
                }
            }
            InputItem::Item(_) => Ok(false),
        }
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
    pub fn type_param_idents(&self) -> syn::Result<Vec<syn::Ident>> {
        let mut cloned = self.clone();
        let item = cloned.ensure_ast()?;
        let idents = match item {
            syn::Item::Const(_) => vec![],
            syn::Item::Enum(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::ExternCrate(_) => vec![],
            syn::Item::Fn(item) => {
                item.sig.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::ForeignMod(_) => vec![],
            syn::Item::Impl(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::Macro(_) => vec![],
            syn::Item::Mod(_) => vec![],
            syn::Item::Static(_) => vec![],
            syn::Item::Struct(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::Trait(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::TraitAlias(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::Type(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::Union(item) => {
                item.generics.type_params().map(|tp| tp.ident.clone()).collect()
            }
            syn::Item::Use(_) => vec![],
            syn::Item::Verbatim(_) => vec![],
            _ => vec![],
        };
        Ok(idents)
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
