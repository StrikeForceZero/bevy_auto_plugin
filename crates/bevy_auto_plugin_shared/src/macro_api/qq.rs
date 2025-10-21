use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{AllowFn, AllowStructOrEnum, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::macro_paths::MacroPathProvider;
use crate::macro_api::mixins::generics::none::WithNoGenerics;
use crate::macro_api::mixins::generics::with_many::WithZeroOrManyGenerics;
use crate::macro_api::mixins::generics::with_single::WithZeroOrOneGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

/// for codegen re-emitting macro args
struct QQ<'a, T> {
    args: &'a T,
    context: &'a Context,
    input_item: &'a mut InputItem,
}

impl<T> QQ<'_, T>
where
    T: MacroPathProvider,
    Self: ToTokens,
{
    fn inject_attribute_macro(&mut self) -> syn::Result<()> {
        let args = self.to_token_stream();
        self.input_item.map_ast(|item| {
            let macro_path = T::macro_path(self.context);
            // insert attribute tokens
            let mut attrs = item
                .take_attrs()
                .map_err(|err| syn::Error::new(item.span(), err))?;
            attrs.insert(0, parse_quote!(#[#macro_path(#args)]));
            item.put_attrs(attrs).unwrap(); // infallible
            Ok(())
        })
    }
}

impl ToTokens
    for QQ<'_, ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>>
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schedule = &self.args.args.base.schedule_config.schedule;
        let mut args = self.args.args.extra_args();
        args.push(quote! { schedule = #schedule });
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<AddMessageArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<AddPluginArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let resource = &self.args.args.base.resource;
        args.push(quote! { resource = #resource });
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

impl ToTokens
    for QQ<
        '_,
        ItemAttribute<
            Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>,
            AllowStructOrEnum,
        >,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
