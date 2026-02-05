use crate::{
    macro_api::mixins::HasKeys,
    syntax::ast::flag::Flag,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(Debug, Clone, FromMeta, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct WithPlugin {
    #[darling(rename = "plugin")]
    pub plugin: syn::Path,
    pub post_build: Flag,
}

impl WithPlugin {
    pub const KEYS: &'static [&'static str] = &["plugin", "post_build"];
}

impl ToTokens for WithPlugin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let plugin = &self.plugin;
        if self.post_build.is_present() {
            tokens.extend(quote! { plugin = #plugin, post_build });
        } else {
            tokens.extend(quote! { plugin = #plugin });
        }
    }
}

impl HasKeys for WithPlugin {
    fn keys() -> &'static [&'static str] {
        WithPlugin::KEYS
    }
}
