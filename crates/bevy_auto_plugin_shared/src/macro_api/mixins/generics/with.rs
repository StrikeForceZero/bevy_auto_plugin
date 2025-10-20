use crate::macro_api::mixins::HasKeys;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(derive_syn_parse)]
pub struct WithGenerics {
    #[darling(multiple, default, rename = "generics")]
    pub generics: Vec<TypeList>,
}

impl WithGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithGenerics {
    fn keys() -> &'static [&'static str] {
        WithGenerics::KEYS
    }
}

impl HasGenerics for WithGenerics {
    fn generics(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokens for WithGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.generics.is_empty() {
            for g in &self.generics {
                tokens.extend(quote! { generics(#g) });
            }
        }
    }
}
