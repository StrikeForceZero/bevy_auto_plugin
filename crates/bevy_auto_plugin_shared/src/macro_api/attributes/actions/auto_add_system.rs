use crate::macro_api::{
    prelude::*,
    schedule_config::ScheduleWithScheduleConfigArgs,
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
}

impl AttributeIdent for AddSystemArgs {
    const IDENT: &'static str = "auto_add_system";
}

pub type IaAddSystem =
    ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type AddSystemAppMutEmitter = AppMutationEmitter<IaAddSystem>;
pub type AddSystemAttrEmitter = AttrEmitter<IaAddSystem>;

impl EmitAppMutationTokens for AddSystemAppMutEmitter {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let schedule = &self.args.args.base.schedule_config.schedule;
        let config_tokens = self.args.args.base.schedule_config.config.to_token_stream();
        let concrete_paths = match self.args.concrete_paths() {
            Ok(paths) => paths,
            Err(err) => {
                tokens.extend(err.to_compile_error());
                return;
            }
        };
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #app_param . add_systems(#schedule, #concrete_path #config_tokens);
            });
        }
    }
}

impl ToTokens for AddSystemAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        // TODO: cleanup
        args.extend(self.args.args.base.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
