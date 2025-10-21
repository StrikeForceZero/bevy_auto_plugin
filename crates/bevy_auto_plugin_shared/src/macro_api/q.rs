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
        let config_tokens = self.args.args.base.schedule_config.config.to_token_stream();
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                .add_systems(#schedule, #concrete_path #config_tokens)
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
            if let Some(expr) = &self.args.args.base.init.expr {
                tokens.extend(quote! { |app| {
                    app.add_plugins({ let plugin: #concrete_path = #expr; plugin })
                }});
            } else if self.args.args.base.init.present {
                tokens.extend(quote! { |app| {
                    app.add_plugins(#concrete_path::default())
                }});
            } else {
                tokens.extend(quote! { |app| {
                    app.add_plugins(#concrete_path)
                }});
            }
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

impl ToTokens
    for Q<
        '_,
        ItemAttribute<
            Composed<ConfigureSystemSetArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = &self.args.args;
        let generics = args.generics();
        let base = &self.args.args.base;
        let schedule = &args.base.schedule_config.schedule;
        let config_tokens = args.base.schedule_config.config.to_token_stream();
        for concrete_path in self.args.concrete_paths() {
            if let Some(inner) = &base.inner {
                // enum
                let chained = if base.chain.is_present() {
                    quote! { .chain() }
                } else if base.chain_ignore_deferred.is_present() {
                    quote! { .chain_ignore_deferred() }
                } else {
                    quote! {}
                };
                let mut entries = vec![];
                for (ident, entry) in inner.entries.iter() {
                    let chained = if entry.chain.is_present() {
                        quote! { .chain() }
                    } else if entry.chain_ignore_deferred.is_present() {
                        quote! { .chain_ignore_deferred() }
                    } else {
                        quote! {}
                    };
                    let config_tokens = entry.config.to_token_stream();
                    entries.push(quote! {
                        #concrete_path :: #ident #chained #config_tokens
                    });
                }
                if !entries.is_empty() {
                    tokens.extend(quote! {
                         .configure_sets(#schedule, (#(#entries),*) #chained #config_tokens)
                    });
                }
            } else {
                // struct
                if generics.is_empty() {
                    tokens.extend(quote! {
                        .configure_sets(#schedule, #concrete_path #config_tokens)
                    });
                } else {
                    // TODO: generics are kind of silly here
                    //  but if someone does use them we'll assume its just a marker type
                    //  that can be initialized via `Default::default()`
                    tokens.extend(quote! {
                        .configure_sets(#schedule, #concrete_path::default() #config_tokens)
                    });
                }
            }
        }
    }
}
