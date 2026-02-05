use crate::{
    macro_api::{
        prelude::*,
        schedule_config::ScheduleWithScheduleConfigArgs,
    },
    syntax::ast::{
        any_expr::AnyExprCallClosureMacroPath,
        any_expr_list::AnyExprList,
    },
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AddSystemArgs {
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
    #[darling(default)]
    pub pipe_in: Option<AnyExprList<AnyExprCallClosureMacroPath>>,
}

impl AttributeIdent for AddSystemArgs {
    const IDENT: &'static str = "auto_add_system";
}

pub type IaAddSystem =
    ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFnOrUse>;
pub type AddSystemAppMutEmitter = AppMutationEmitter<IaAddSystem>;
pub type AddSystemAttrEmitter = AttrEmitter<IaAddSystem>;

impl EmitAppMutationTokens for AddSystemAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let schedule = &self.args.args.base.schedule_config.schedule;
        let config_tokens = self.args.args.base.schedule_config.config.to_token_stream();
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            let system_tokens = match &self.args.args.base.pipe_in {
                Some(pipe_in) if pipe_in.is_empty() => quote! { #concrete_path },
                Some(pipe_in) => {
                    let mut iter = pipe_in.iter();
                    let first =
                        iter.next().expect("pipe_in should not be empty after is_empty check");
                    let mut expr_tokens = quote! { (#first) };
                    for item in iter {
                        expr_tokens = quote! { (#expr_tokens).pipe(#item) };
                    }
                    quote! { (#expr_tokens).pipe(#concrete_path) }
                }
                None => quote! { #concrete_path },
            };
            tokens.extend(quote! {
                #app_param . add_systems(#schedule, #system_tokens #config_tokens);
            });
        }
        Ok(())
    }
}

impl ToTokens for AddSystemAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let schedule = &self.args.args.base.schedule_config.schedule;
        args.push(quote! { schedule = #schedule });
        if let Some(pipe_in) = &self.args.args.base.pipe_in {
            args.push(quote! { pipe_in = [#pipe_in] });
        }
        let config = self.args.args.base.schedule_config.config.to_inner_arg_tokens_vec();
        if !config.is_empty() {
            args.push(quote! { config( #(#config),* )});
        }
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
