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
use std::marker::PhantomData;
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

pub trait IdentPathResolver {
    const NOT_ALLOWED_MESSAGE: &'static str = "Unable to resolve ident path";
    fn resolve_ident_path(item: &Item) -> Option<syn::Path>;
}

pub struct AllowStructOrEnum;
impl IdentPathResolver for AllowStructOrEnum {
    const NOT_ALLOWED_MESSAGE: &'static str = "Only allowed on Struct Or Enum items";
    fn resolve_ident_path(item: &Item) -> Option<syn::Path> {
        Some(match item {
            Item::Struct(item) => item.ident.clone().into(),
            Item::Enum(item) => item.ident.clone().into(),
            _ => return None,
        })
    }
}

pub struct AllowFn;
impl IdentPathResolver for AllowFn {
    const NOT_ALLOWED_MESSAGE: &'static str = "Only allowed on Fn items";
    fn resolve_ident_path(item: &Item) -> Option<syn::Path> {
        Some(match item {
            Item::Fn(item) => item.sig.ident.clone().into(),
            _ => return None,
        })
    }
}

pub struct ItemAttribute<T, Resolver> {
    pub args: T,
    pub context: Context,
    pub input_item: InputItem,
    pub target: syn::Path,
    pub _resolver: PhantomData<Resolver>,
}

impl<T, Resolver> ItemAttribute<T, Resolver>
where
    T: Parse,
    Resolver: IdentPathResolver,
{
    pub fn from_attr_input(
        attr: TokenStream,
        input: TokenStream,
        context: Context,
    ) -> syn::Result<Self> {
        let mut input_item = InputItem::new(input);
        let item = input_item.ensure_ast()?;
        let Some(target) = Resolver::resolve_ident_path(item) else {
            return Err(syn::Error::new(
                input_item.span(),
                Resolver::NOT_ALLOWED_MESSAGE,
            ));
        };
        Ok(Self {
            args: parse2::<T>(attr)?,
            context,
            input_item,
            target,
            _resolver: PhantomData,
        })
    }
}

impl<T, M2, Resolver> ItemAttribute<Composed<T, WithPlugin, M2>, Resolver> {
    pub fn plugin(&self) -> &syn::Path {
        self.args.plugin()
    }
}

impl<T, M1, M2, Resolver> ItemAttribute<Composed<T, M1, M2>, Resolver>
where
    M2: HasGenerics,
{
    pub fn concrete_paths(&self) -> Vec<syn::Path> {
        self.args.concrete_paths(&self.target.clone().into())
    }
}

impl<T1, Resolver> ItemAttribute<T1, Resolver> {
    fn convert_into<T2>(value: ItemAttribute<T1, Resolver>) -> ItemAttribute<T2, Resolver>
    where
        T2: From<T1>,
    {
        ItemAttribute {
            args: T2::from(value.args),
            context: value.context,
            input_item: value.input_item,
            target: value.target,
            _resolver: PhantomData,
        }
    }
}
