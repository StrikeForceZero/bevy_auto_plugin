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

pub type IaInsertResource = ItemAttribute<
    Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
    AllowStructOrEnum,
>;
pub type QInsertResource = AppMutationEmitter<IaInsertResource>;
pub type QQInsertResource = AttrEmitter<IaInsertResource>;

impl EmitAppMutationTokens for QInsertResource {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let resource = &self.args.args.base.resource;
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.insert_resource({ let resource: #concrete_path = #resource; resource});
            });
        }
    }
}

impl ToTokens for QQInsertResource {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let resource = &self.args.args.base.resource;
        args.push(quote! { resource = #resource });
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
