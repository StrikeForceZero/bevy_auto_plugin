use crate::macro_api::prelude::*;
use crate::syntax::extensions::lit::LitExt;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct NameArgs {
    pub name: Option<syn::Lit>,
}

impl AttributeIdent for NameArgs {
    const IDENT: &'static str = "auto_name";
}

pub type Name =
    ItemAttribute<Composed<NameArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QNameArgs<'a> = Q<'a, Name>;
pub type QQNameArgs<'a> = QQ<'a, Name>;

impl RequiredUseQTokens for QNameArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let args = &self.args.args.base;
        for concrete_path in self.args.concrete_paths() {
            let name = args
                .name
                .as_ref()
                .map(|name| name.unquoted_string())
                .unwrap_or_else(|| {
                    // TODO: move to util fn
                    quote!(#concrete_path)
                        .to_string()
                        .replace(" < ", "<")
                        .replace(" >", ">")
                        .replace(" ,", ",")
                    // TODO: offer option to only remove all spaces?
                    //  .replace(" ", "")
                });
            let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
            tokens.extend(quote! {
                #app_param.register_required_components_with::<#concrete_path, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new(#name));
            });
        }
    }
}

impl ToTokens for QQNameArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        if let Some(name) = &self.args.args.base.name {
            args.push(quote! { name = #name });
        }
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
