use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::analysis::item::IdentFromItemResult;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use std::hash::Hash;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Item, parse_quote, parse2};

mod actions;
mod auto_plugin;
mod rewrites;
mod traits;

pub mod prelude {
    pub use super::auto_plugin::{
        AutoPluginFnArgs, AutoPluginStructOrEnumArgs, resolve_app_param_name,
    };
    pub use crate::macro_api::attributes::actions::auto_add_message::AddMessageArgs;
    pub use crate::macro_api::attributes::actions::auto_add_observer::AddObserverArgs;
    pub use crate::macro_api::attributes::actions::auto_add_plugin::AddPluginArgs;
    pub use crate::macro_api::attributes::actions::auto_add_system::AddSystemArgs;
    pub use crate::macro_api::attributes::actions::auto_configure_system_set::{
        ConfigureSystemSetArgs,
        with_plugin_args_from_attr_input as configure_system_set_args_from_attr_input,
    };
    pub use crate::macro_api::attributes::actions::auto_init_resource::InitResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_init_state::InitStateArgs;
    pub use crate::macro_api::attributes::actions::auto_init_sub_state::InitSubStateArgs;
    pub use crate::macro_api::attributes::actions::auto_insert_resource::InsertResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_name::NameArgs;
    pub use crate::macro_api::attributes::actions::auto_register_state_type::RegisterStateTypeArgs;
    pub use crate::macro_api::attributes::actions::auto_register_type::RegisterTypeArgs;
    pub use crate::macro_api::attributes::actions::auto_run_on_build::RunOnBuildArgs;
    pub use crate::macro_api::attributes::rewrites::auto_component::ComponentArgs;
    pub use crate::macro_api::attributes::rewrites::auto_event::EventArgs;
    pub use crate::macro_api::attributes::rewrites::auto_message::MessageArgs;
    pub use crate::macro_api::attributes::rewrites::auto_observer::ObserverArgs;
    pub use crate::macro_api::attributes::rewrites::auto_resource::ResourceArgs;
    pub use crate::macro_api::attributes::rewrites::auto_states::StatesArgs;
    pub use crate::macro_api::attributes::rewrites::auto_system::SystemArgs;
    pub use crate::macro_api::attributes::traits::prelude::*;
}

pub trait AttributeIdent {
    const IDENT: &'static str;
    fn full_attribute_path() -> NonEmptyPath {
        let ident = format_ident!("{}", Self::IDENT);
        parse_quote!( ::bevy_auto_plugin::prelude::#ident )
    }
}

pub trait ItemAttributeArgs:
    AttributeIdent + FromMeta + Parse + ToTokensWithConcreteTargetPath + Hash + Clone
{
    fn global_build_prefix() -> Ident {
        format_ident!("_auto_plugin_{}_", Self::IDENT)
    }
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_>;
}

pub struct ItemAttribute<T> {
    pub args: T,
    pub context: Context,
    pub input_item: InputItem,
    pub target: syn::Ident,
}

impl<T> ItemAttribute<T>
where
    T: Parse,
{
    pub fn from_attr_input(
        attr: TokenStream,
        input: TokenStream,
        context: Context,
    ) -> syn::Result<Self> {
        let mut input_item = InputItem::new(input);
        let Some(target) = input_item.get_ident()?.cloned() else {
            return Err(syn::Error::new(
                input_item.span(),
                "Unable to resolve target ident",
            ));
        };
        Ok(Self {
            args: parse2::<T>(attr)?,
            context,
            input_item,
            target,
        })
    }
}

impl<T, M2> ItemAttribute<Composed<T, WithPlugin, M2>> {
    pub fn plugin(&self) -> &syn::Path {
        self.args.plugin()
    }
}

impl<T, M1, M2> ItemAttribute<Composed<T, M1, M2>>
where
    M2: HasGenerics,
{
    pub fn concrete_paths(&self) -> Vec<syn::Path> {
        self.args.concrete_paths(&self.target.clone().into())
    }
}

impl<T1> ItemAttribute<T1> {
    fn convert_into<T2>(value: ItemAttribute<T1>) -> ItemAttribute<T2>
    where
        T2: From<T1>,
    {
        ItemAttribute {
            args: T2::from(value.args),
            context: value.context,
            input_item: value.input_item,
            target: value.target,
        }
    }
}
