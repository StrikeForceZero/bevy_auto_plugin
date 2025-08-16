use crate::__private::attribute_args::attributes::prelude::*;
use crate::__private::attribute_args::{ItemAttributeArgs, ToTokensWithConcreteTargetPath};
use crate::__private::util::concrete_path::ConcreteTargetPath;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ToTokenStringValue<T> {
    pub token_string: String,
    _marker: PhantomData<T>,
}

impl<T> Eq for ToTokenStringValue<T> where T: PartialEq {}

impl<T> ToTokenStringValue<T> {
    fn new(token_string: String) -> Self {
        Self {
            token_string,
            _marker: PhantomData,
        }
    }
}

impl<T> From<(ConcreteTargetPath, T)> for ToTokenStringValue<T>
where
    T: ToTokensWithConcreteTargetPath + SupportsAutoPluginContextInsert,
{
    fn from(value: (ConcreteTargetPath, T)) -> Self {
        let (concrete_path, args) = value;
        Self::from((concrete_path, &args))
    }
}

impl<T> From<(ConcreteTargetPath, &T)> for ToTokenStringValue<T>
where
    T: ToTokensWithConcreteTargetPath + SupportsAutoPluginContextInsert,
{
    fn from(value: (ConcreteTargetPath, &T)) -> Self {
        let (concrete_path, args) = value;
        let tokens = args.to_token_stream_with_concrete_target_path(&concrete_path);
        Self::new(tokens.to_string())
    }
}

impl<T> ToTokens for ToTokenStringValue<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // unwrap because we control the ToTokens above
        let new_tokens = TokenStream::from_str(&self.token_string).unwrap();
        tokens.extend(new_tokens);
    }
}

#[derive(Default)]
pub struct AutoPluginContext {
    pub register_types: HashSet<ToTokenStringValue<RegisterTypeAttributeArgs>>,
    pub register_state_types: HashSet<ToTokenStringValue<RegisterStateTypeAttributeArgs>>,
    pub add_events: HashSet<ToTokenStringValue<AddEventAttributeArgs>>,
    pub init_resources: HashSet<ToTokenStringValue<InitResourceAttributeArgs>>,
    pub insert_resources: HashSet<ToTokenStringValue<InsertResourceAttributeArgs>>,
    pub init_states: HashSet<ToTokenStringValue<InitStateAttributeArgs>>,
    pub auto_names: HashSet<ToTokenStringValue<AutoNameAttributeArgs>>,
    pub add_systems: HashSet<ToTokenStringValue<AddSystemAttributeArgs>>,
    pub add_observers: HashSet<ToTokenStringValue<AddObserverAttributeArgs>>,
}

pub trait SupportsAutoPluginContextInsert {}

pub trait AutoPluginContextInsert {
    type Item: ItemAttributeArgs + SupportsAutoPluginContextInsert;
    fn insert_self(self, context: &mut AutoPluginContext) -> bool;
}

macro_rules! impl_traits {
    ($var:ident, $ident:ident) => {
        impl SupportsAutoPluginContextInsert for $ident {}
        impl AutoPluginContextInsert for ToTokenStringValue<$ident> {
            type Item = $ident;
            fn insert_self(self, context: &mut AutoPluginContext) -> bool {
                context.$var.insert(self)
            }
        }
    };
}

impl_traits!(register_types, RegisterTypeAttributeArgs);
impl_traits!(register_state_types, RegisterStateTypeAttributeArgs);
impl_traits!(add_events, AddEventAttributeArgs);
impl_traits!(init_resources, InitResourceAttributeArgs);
impl_traits!(insert_resources, InsertResourceAttributeArgs);
impl_traits!(init_states, InitStateAttributeArgs);
impl_traits!(auto_names, AutoNameAttributeArgs);
impl_traits!(add_systems, AddSystemAttributeArgs);
impl_traits!(add_observers, AddObserverAttributeArgs);

impl AutoPluginContext {
    fn tokens(&self) -> impl Iterator<Item = TokenStream> {
        let item = std::iter::empty();
        macro_rules! chain {
            ($var:ident, $ident:ident) => {
                let $var = $var.chain(self.$ident.iter().map(ToTokenStringValue::to_token_stream));
            };
        }

        chain!(item, register_types);
        chain!(item, register_state_types);
        chain!(item, add_events);
        chain!(item, init_resources);
        chain!(item, insert_resources);
        chain!(item, init_states);
        chain!(item, auto_names);
        chain!(item, add_systems);
        chain!(item, add_observers);

        item
    }
    pub fn expand_build(&self, app_ident: &Ident) -> TokenStream {
        self.tokens()
            .map(|tokens| {
                quote! {
                    #app_ident #tokens;
                }
            })
            .collect()
    }
}
