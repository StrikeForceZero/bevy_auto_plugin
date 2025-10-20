use crate::macro_api::mixins::HasKeys;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
pub struct WithPlugin {
    #[darling(rename = "plugin")]
    pub plugin: syn::Path,
}

impl WithPlugin {
    pub const KEYS: &'static [&'static str] = &["plugin"];
}

impl ToTokens for WithPlugin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! { plugin = #self.plugin });
    }
}

impl HasKeys for WithPlugin {
    fn keys() -> &'static [&'static str] {
        WithPlugin::KEYS
    }
}
