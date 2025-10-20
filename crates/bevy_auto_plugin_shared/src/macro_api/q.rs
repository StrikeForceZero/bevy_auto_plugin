use crate::macro_api::attributes::ItemAttribute;
use crate::macro_api::attributes::prelude::*;
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
    for Q<'_, ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<AddMessageArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<AddPluginArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.init_resource::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens for Q<'_, ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.init_state::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens for Q<'_, ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.init_sub_state::<#concrete_path>();
            }});
        }
    }
}

impl ToTokens
    for Q<'_, ItemAttribute<Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>>>
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
    for Q<'_, ItemAttribute<Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                #concrete_path(app);
            }});
        }
    }
}
