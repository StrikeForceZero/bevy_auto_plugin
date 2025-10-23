use crate::macro_api::prelude::*;
use crate::syntax::ast::any_expr::AnyExprCallClosureMacroPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct InsertResourceArgs {
    pub resource: AnyExprCallClosureMacroPath,
}

impl AttributeIdent for InsertResourceArgs {
    const IDENT: &'static str = "auto_insert_resource";
}

pub type InsertResource = ItemAttribute<
    Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
    AllowStructOrEnum,
>;
pub type QInsertResourceArgs<'a> = Q<'a, InsertResource>;
pub type QQInsertResourceArgs<'a> = QQ<'a, InsertResource>;

impl RequiredUseQTokens for QInsertResourceArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                #app_param.insert_resource(#concrete_path::default());
            }});
        }
    }
}

impl ToTokens for QQInsertResourceArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let resource = &self.args.args.base.resource;
        args.push(quote! { resource = #resource });
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
