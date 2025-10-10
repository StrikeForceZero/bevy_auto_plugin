use crate::__private::attribute::AutoPluginItemAttribute;
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::global_args::{AutoPluginAttributeKind, GenericsArgs, ItemAttributeArgs};
use crate::macro_api::schedule_config::ScheduleWithScheduleConfigArgs;
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_fn};
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Item;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AddSystemArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl AutoPluginAttributeKind for AddSystemArgs {
    type Attribute = AutoPluginItemAttribute;
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::AddSystem
    }
}

impl ItemAttributeArgs for AddSystemArgs {
    fn global_build_prefix() -> &'static str {
        "_auto_plugin_add_systems_"
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_fn(item)
    }
}

impl GenericsArgs for AddSystemArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for AddSystemArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        let schedule = &self.schedule_config.schedule;
        let config_tokens = self.schedule_config.config.to_token_stream();
        tokens.extend(quote! {
            .add_systems(#schedule, #target #config_tokens)
        })
    }
}

impl ArgsBackToTokens for AddSystemArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut args = vec![];
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        args.extend(self.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! { #(#args),* });
    }
}
