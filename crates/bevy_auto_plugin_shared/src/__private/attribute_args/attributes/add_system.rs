use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::schedule_config::ScheduleWithScheduleConfigArgs;
use crate::__private::attribute_args::{
    AutoPluginAttributeKind, GenericsArgs, ItemAttributeArgs, ToTokensWithConcreteTargetPath,
};
use crate::__private::item_with_attr_match::{ItemWithAttributeMatch, items_with_attribute_match};
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::ConcreteTargetPath;
use crate::__private::util::meta::fn_meta::FnMeta;
use crate::__private::util::resolve_ident_from_item::{IdentFromItemResult, resolve_ident_from_fn};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Item;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AddSystemAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl AutoPluginAttributeKind for AddSystemAttributeArgs {
    type Attribute = AutoPluginItemAttribute;
    fn attribute() -> AutoPluginItemAttribute {
        AutoPluginItemAttribute::AddSystem
    }
}

impl ItemAttributeArgs for AddSystemAttributeArgs {
    fn global_build_prefix() -> &'static str {
        "_global_auto_plugin_add_systems_"
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_fn(item)
    }
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<'_, Self>>> {
        items_with_attribute_match::<FnMeta, AddSystemAttributeArgs>(items)
    }
}

impl GenericsArgs for AddSystemAttributeArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for AddSystemAttributeArgs {
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

impl ArgsBackToTokens for AddSystemAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut args = vec![];
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        args.extend(self.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! { #(#args),* });
    }
}
