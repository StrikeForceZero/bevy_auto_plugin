use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{
    AllowFn, AllowStructOrEnum, GenericsCap, ItemAttribute, ItemAttributeParse,
};
use crate::macro_api::composed::Composed;
use crate::macro_api::mixins::generics::none::WithNoGenerics;
use crate::macro_api::mixins::generics::with_many::WithZeroOrManyGenerics;
use crate::macro_api::mixins::generics::with_single::WithZeroOrOneGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::extensions::lit::LitExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse::{Parse, ParseStream};

/// for codegen attaching to bevy app
pub(crate) struct Q<'a, T> {
    pub(crate) args: &'a T,
    // TODO: maybe app params should just be part of another wrapper struct?
    pub(crate) app_param: syn::Ident,
}

impl<T> Q<'_, T>
where
    T: ItemAttributeParse,
{
    pub fn from_args(args: &T) -> Q<T> {
        Q::<T> {
            args,
            app_param: format_ident!("app"),
        }
    }
}

pub trait RequiredUseQTokens {
    fn required_uses(&self) -> Vec<TokenStream> {
        vec![]
    }
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident);
}

impl<'a, T> ToTokens for Q<'a, T>
where
    Self: RequiredUseQTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.required_uses());
        RequiredUseQTokens::to_tokens(self, tokens, &self.app_param);
    }
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

pub type AddMessage =
    ItemAttribute<Composed<AddMessageArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QAddMessageArgs<'a> = Q<'a, AddMessage>;
impl RequiredUseQTokens for QAddMessageArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.add_message::<#concrete_path>();
            });
        }
    }
}

pub type AddObserver =
    ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type QAddObserverArgs<'a> = Q<'a, AddObserver>;
impl RequiredUseQTokens for QAddObserverArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.add_observer::<#concrete_path>();
            });
        }
    }
}

pub type AddPlugin =
    ItemAttribute<Composed<AddPluginArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QAddPluginArgs<'a> = Q<'a, AddPlugin>;
impl RequiredUseQTokens for QAddPluginArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            if let Some(expr) = &self.args.args.base.init.expr {
                tokens.extend(quote! {
                    #app_param.add_plugins({ let plugin: #concrete_path = #expr; plugin });
                });
            } else if self.args.args.base.init.present {
                tokens.extend(quote! {
                    #app_param.add_plugins(#concrete_path::default());
                });
            } else {
                tokens.extend(quote! {
                    #app_param.add_plugins(#concrete_path);
                });
            }
        }
    }
}

pub type InitResource = ItemAttribute<
    Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QInitResourceArgs<'a> = Q<'a, InitResource>;
impl RequiredUseQTokens for QInitResourceArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.init_resource::<#concrete_path>();
            });
        }
    }
}

pub type InitState =
    ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type QInitStateArgs<'a> = Q<'a, InitState>;
impl RequiredUseQTokens for QInitStateArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.init_state::<#target>();
        });
    }
}

pub type InitSubState =
    ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type QInitSubStateArgs<'a> = Q<'a, InitSubState>;
impl RequiredUseQTokens for QInitSubStateArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.init_sub_state::<#target>();
        });
    }
}

pub type InsertResource = ItemAttribute<
    Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
    AllowStructOrEnum,
>;
pub type QInsertResourceArgs<'a> = Q<'a, InsertResource>;
impl RequiredUseQTokens for QInsertResourceArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                #app_param.insert_resource(#concrete_path::default());
            }});
        }
    }
}

pub type RegisterStateType = ItemAttribute<
    Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QRegisterStateTypeArgs<'a> = Q<'a, RegisterStateType>;
impl RequiredUseQTokens for QRegisterStateTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            let bevy_state = crate::__private::paths::state::root_path();
            tokens.extend(quote! {
                #app_param.register_type :: < #bevy_state::prelude::State< #concrete_path > >();
                #app_param.register_type :: < #bevy_state::prelude::NextState< #concrete_path > >();
            });
        }
    }
}

pub type RegisterType = ItemAttribute<
    Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QRegisterTypeArgs<'a> = Q<'a, RegisterType>;
impl RequiredUseQTokens for QRegisterTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.register_type::<#concrete_path>();
            });
        }
    }
}

pub type RunOnBuild =
    ItemAttribute<Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QRunOnBuildArgs<'a> = Q<'a, RunOnBuild>;
impl RequiredUseQTokens for QRunOnBuildArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #concrete_path(#app_param);
            });
        }
    }
}

pub type ConfigureSystemSet = ItemAttribute<
    Composed<ConfigureSystemSetArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QConfigureSystemSetArgs<'a> = Q<'a, ConfigureSystemSet>;
impl RequiredUseQTokens for QConfigureSystemSetArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
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
                         #app_param.configure_sets(#schedule, (#(#entries),*) #chained #config_tokens);
                    });
                }
            } else {
                // struct
                if generics.is_empty() {
                    tokens.extend(quote! {
                        #app_param.configure_sets(#schedule, #concrete_path #config_tokens);
                    });
                } else {
                    // TODO: generics are kind of silly here
                    //  but if someone does use them we'll assume its just a marker type
                    //  that can be initialized via `Default::default()`
                    tokens.extend(quote! {
                        #app_param.configure_sets(#schedule, #concrete_path::default() #config_tokens);
                    });
                }
            }
        }
    }
}

pub type Name =
    ItemAttribute<Composed<NameArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QNameArgs<'a> = Q<'a, Name>;
impl RequiredUseQTokens for QNameArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let args = &self.args.args.base;
        for concrete_path in self.args.concrete_paths() {
            let name = args
                .name
                .as_ref()
                .map(|name| name.unquoted_string())
                .unwrap_or_else(|| {
                    // TODO: move to util fn
                    quote!(#concrete_path)
                        .to_string()
                        .replace(" < ", "<")
                        .replace(" >", ">")
                        .replace(" ,", ",")
                    // TODO: offer option to only remove all spaces?
                    //  .replace(" ", "")
                });
            let bevy_ecs = crate::__private::paths::ecs::ecs_root_path();
            tokens.extend(quote! {
                #app_param.register_required_components_with::<#concrete_path, #bevy_ecs::prelude::Name>(|| #bevy_ecs::prelude::Name::new(#name));
            });
        }
    }
}
