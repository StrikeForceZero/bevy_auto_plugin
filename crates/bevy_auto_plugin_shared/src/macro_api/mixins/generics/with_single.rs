use crate::macro_api::mixins::HasKeys;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(derive_syn_parse)]
pub struct WithZeroOrOneGenerics {
    #[darling(multiple, default, rename = "generics")]
    pub generics: Vec<TypeList>,
}

impl WithZeroOrOneGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithZeroOrOneGenerics {
    fn keys() -> &'static [&'static str] {
        WithZeroOrOneGenerics::KEYS
    }
}

impl HasGenerics for WithZeroOrOneGenerics {
    fn generics(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokens for WithZeroOrOneGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.generics.is_empty() {
            return;
        }

        let sets = self.generics.iter().map(|g| quote! { generics(#g) });
        tokens.extend(quote! {
            #(#sets),*
        });
    }
}
