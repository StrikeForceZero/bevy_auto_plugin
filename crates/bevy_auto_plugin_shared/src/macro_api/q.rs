use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{AllowFn, AllowStructOrEnum, GenericsCap, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::mixins::generics::none::WithNoGenerics;
use crate::macro_api::mixins::generics::with_many::WithZeroOrManyGenerics;
use crate::macro_api::mixins::generics::with_single::WithZeroOrOneGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

/// for codegen attaching to bevy app
struct Q<'a, T> {
    args: &'a T,
    context: &'a Context,
    input_item: &'a InputItem,
}

impl ToTokens
    for Q<'_, ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schedule = &self.args.args.base.schedule_config.schedule;
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.add_system(#schedule, #concrete_path);
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<AddMessageArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.add_message::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<'_, ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.add_observer::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<AddPluginArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.add_plugin::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.init_resource::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<'_, ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let target = &self.args.target;
        tokens.extend(quote! { |app| {
            app.init_state::<#target>();
        }});
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let target = &self.args.target;
        tokens.extend(quote! { |app| {
            app.init_sub_state::<#target>();
        }});
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.insert_resource(#concrete_path::default());
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.register_state_type::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.register_type::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                #concrete_path(app);
            }});
        }
    }
}
