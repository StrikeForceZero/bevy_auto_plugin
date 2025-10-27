use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_expr::FlagOrExpr;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddPluginArgs {
    #[darling(default)]
    pub init: FlagOrExpr,
}

impl AttributeIdent for AddPluginArgs {
    const IDENT: &'static str = "auto_add_plugin";
}

pub type IaAddPlugin =
    ItemAttribute<Composed<AddPluginArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type AddPluginAppMutEmitter = AppMutationEmitter<IaAddPlugin>;
pub type AddPluginAttrEmitter = AttrEmitter<IaAddPlugin>;

impl EmitAppMutationTokens for AddPluginAppMutEmitter {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            if let Some(expr) = &self.args.args.base.init.expr {
                tokens.extend(quote! {
                    #app_param.add_plugins({ let plugin: #concrete_path = #expr; plugin });
                });
            } else if self.args.args.base.init.present {
                tokens.extend(quote! {
                    #app_param.add_plugins(#concrete_path::default());
                });
            } else {
                tokens.extend(quote! {
                    #app_param.add_plugins(#concrete_path);
                });
            }
        }
    }
}

impl ToTokens for AddPluginAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
