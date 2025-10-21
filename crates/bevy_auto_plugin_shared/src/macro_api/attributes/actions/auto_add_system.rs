use crate::macro_api::attributes::{AllowFn, AttributeIdent, GenericsCap, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::prelude::{WithPlugin, WithZeroOrManyGenerics};
use crate::macro_api::q::{Q, RequiredUseQTokens};
use crate::macro_api::schedule_config::ScheduleWithScheduleConfigArgs;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AddSystemArgs {
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl AttributeIdent for AddSystemArgs {
    const IDENT: &'static str = "auto_add_system";
}

pub type AddSystem =
    ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type QAddSystemArgs<'a> = Q<'a, AddSystem>;

impl RequiredUseQTokens for QAddSystemArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let schedule = &self.args.args.base.schedule_config.schedule;
        let config_tokens = self.args.args.base.schedule_config.config.to_token_stream();
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.add_systems(#schedule, #concrete_path #config_tokens);
            });
        }
    }
}
