use crate::{
    macro_api::prelude::*,
    syntax::ast::any_expr::AnyExprCallClosureMacroPath,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ObserverConfigArgs {
    #[darling(multiple)]
    pub run_if: Vec<AnyExprCallClosureMacroPath>,
}

impl ObserverConfigArgs {
    pub fn is_empty(&self) -> bool {
        self.run_if.is_empty()
    }

    pub fn to_inner_arg_tokens_vec(&self) -> Vec<TokenStream> {
        self.run_if
            .iter()
            .map(|run_if| {
                quote! {
                    run_if = #run_if
                }
            })
            .collect()
    }
}

impl ToTokens for ObserverConfigArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for run_if in &self.run_if {
            tokens.extend(quote! {
                .run_if(#run_if)
            });
        }
    }
}

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddObserverArgs {
    pub config: ObserverConfigArgs,
}

impl AttributeIdent for AddObserverArgs {
    const IDENT: &'static str = "auto_add_observer";
}

pub type IaAddObserver =
    ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFnOrUse>;
pub type AddObserverAppMutEmitter = AppMutationEmitter<IaAddObserver>;
pub type AddObserverAttrEmitter = AttrEmitter<IaAddObserver>;

impl EmitAppMutationTokens for AddObserverAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let config_tokens = self.args.args.base.config.to_token_stream();
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #app_param.add_observer( #concrete_path #config_tokens );
            });
        }
        Ok(())
    }
}

impl ToTokens for AddObserverAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let config = self.args.args.base.config.to_inner_arg_tokens_vec();
        if !config.is_empty() {
            args.push(quote! { config( #(#config),* )});
        }
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
