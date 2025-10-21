use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
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
    pub use crate::macro_api::attributes::actions::auto_configure_system_set::ConfigureSystemSetArgs;
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

pub trait ItemAttributeArgs: AttributeIdent + Hash + Clone {
    fn global_build_prefix() -> Ident {
        format_ident!("_auto_plugin_{}_", Self::IDENT)
    }
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

pub trait PluginCap {
    fn plugin_path(&self) -> &syn::Path;
}

pub trait GenericsCap {
    fn generics(&self) -> &[TypeList];
    fn concrete_paths(&self) -> Vec<syn::Path>;
}

pub struct ItemAttribute<T, Resolver> {
    pub args: T,
    pub context: Context,
    pub input_item: InputItem,
    pub target: syn::Path,
    pub _resolver: PhantomData<Resolver>,
}

// TODO: where should this live?
impl<T, R> ItemAttribute<T, R>
where
    T: ItemAttributeArgs + Hash,
{
    pub fn _concat_ident_hash(&self, ident: &Ident) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.args.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn _get_unique_ident(&self, prefix: Ident, ident: &Ident) -> Ident {
        let hash = self._concat_ident_hash(ident);
        format_ident!("{prefix}_{hash}")
    }

    pub fn get_unique_ident(&self, ident: &Ident) -> Ident {
        self._get_unique_ident(T::global_build_prefix(), ident)
    }
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

impl<C, M2, R> PluginCap for ItemAttribute<Composed<C, WithPlugin, M2>, R> {
    fn plugin_path(&self) -> &syn::Path {
        &self.args.plugin.plugin
    }
}

impl<C, M1, M2, R> GenericsCap for ItemAttribute<Composed<C, M1, M2>, R>
where
    M2: HasGenerics,
{
    fn generics(&self) -> &[TypeList] {
        self.args.generics.generics()
    }
    fn concrete_paths(&self) -> Vec<syn::Path> {
        let target = &self.target;
        if self.args.generics.generics().is_empty() {
            vec![target.clone()]
        } else {
            self.args
                .generics
                .generics()
                .iter()
                .map(|g| syn::parse_quote!(#target::<#g>))
                .collect()
        }
    }
}
