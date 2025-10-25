pub mod tokens;

use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};

#[derive(Debug, Default, Clone)]
pub struct ExpandAttrs {
    pub attrs: Vec<MacroStream>,
    pub use_items: Vec<MacroStream>,
}

impl PartialEq for ExpandAttrs {
    fn eq(&self, other: &Self) -> bool {
        quote!(#self).to_token_stream().to_string() == quote!(#other).to_token_stream().to_string()
    }
}

impl ExpandAttrs {
    pub fn append(&mut self, other: Self) {
        self.attrs.extend(other.attrs);
        self.use_items.extend(other.use_items);
    }
}

impl ToTokens for ExpandAttrs {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        let use_items = &self.use_items;
        tokens.extend(quote! {
            #(#use_items)*

        });
        let attrs = &self.attrs;
        tokens.extend(quote! {
            #(#attrs)*
        });
    }
}
