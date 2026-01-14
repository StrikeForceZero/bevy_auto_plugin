use crate::{
    macro_api::prelude::*,
    syntax::validated::non_empty_path::NonEmptyPath,
    util::macros::impl_from_default,
};
use darling::{
    FromMeta,
    ast::NestedMeta,
};
use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::format_ident;
use std::{
    hash::Hash,
    marker::PhantomData,
};
use syn::{
    Item,
    Path,
    Token,
    parse::Parser,
    parse_quote,
    punctuated::Punctuated,
};

mod actions;
mod auto_plugin;
mod rewrites;

pub mod prelude {
    pub use super::{
        AllowAny,
        AllowFn,
        AllowStructOrEnum,
        AttributeIdent,
        GenericsCap,
        ItemAttribute,
        ItemAttributeArgs,
        ItemAttributeContext,
        ItemAttributeInput,
        ItemAttributeParse,
        ItemAttributePlugin,
        ItemAttributeTarget,
        ItemAttributeUniqueIdent,
        auto_plugin::{
            AutoPluginFnArgs,
            AutoPluginStructOrEnumArgs,
            resolve_app_param_name,
        },
    };
    pub use crate::macro_api::attributes::{
        actions::prelude::*,
        rewrites::prelude::*,
    };
}

pub trait AttributeIdent {
    const IDENT: &'static str;
    #[allow(dead_code)]
    // TODO: should we use this over the context macro_paths?
    //  context macro paths would allow us to resolve aliased versions of this crate
    fn full_attribute_path() -> NonEmptyPath {
        let ident = format_ident!("{}", Self::IDENT);
        parse_quote!( ::bevy_auto_plugin::prelude::#ident )
    }
}

pub trait ItemAttributeArgs: AttributeIdent + Clone {
    fn global_build_prefix() -> Ident {
        format_ident!("_auto_plugin_{}_", Self::IDENT)
    }
}

impl<T, R> AttributeIdent for ItemAttribute<T, R>
where
    T: AttributeIdent + Clone,
{
    const IDENT: &'static str = T::IDENT;
}
impl<T, P, G, R> ItemAttributeArgs for ItemAttribute<Composed<T, P, G>, R>
where
    T: AttributeIdent + Clone,
    P: Clone,
    G: Clone,
    R: Clone,
{
}

pub trait IdentPathResolver {
    const NOT_ALLOWED_MESSAGE: &'static str = "Unable to resolve ident path";
    fn resolve_ident_path(item: &Item) -> Option<syn::Path>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllowAny;
impl IdentPathResolver for AllowAny {
    fn resolve_ident_path(item: &Item) -> Option<syn::Path> {
        Some(match item {
            Item::Const(item) => item.ident.clone().into(),
            Item::Enum(item) => item.ident.clone().into(),
            Item::ExternCrate(item) => item.ident.clone().into(),
            Item::Fn(item) => item.sig.ident.clone().into(),
            Item::ForeignMod(_) => return None,
            Item::Impl(_) => return None,
            Item::Macro(item) => return item.ident.clone().map(Into::into),
            Item::Mod(item) => item.ident.clone().into(),
            Item::Static(item) => item.ident.clone().into(),
            Item::Struct(item) => item.ident.clone().into(),
            Item::Trait(item) => item.ident.clone().into(),
            Item::TraitAlias(item) => item.ident.clone().into(),
            Item::Type(item) => item.ident.clone().into(),
            Item::Union(item) => item.ident.clone().into(),
            // TODO: implement
            Item::Use(_) => return None,
            Item::Verbatim(_) => return None,
            _ => return None,
        })
    }
}
impl_from_default!(AllowAny => (AllowStructOrEnum, AllowFn));

pub trait GenericsCap {
    fn concrete_paths(&self) -> syn::Result<Vec<syn::Path>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemAttribute<T, Resolver> {
    pub args: T,
    pub context: Context,
    pub input_item: InputItem,
    pub target: syn::Path,
    pub _resolver: PhantomData<Resolver>,
}

// TODO: where should this live?
impl<T, R> ItemAttribute<T, R> {
    pub fn _concat_ident_hash(&self, ident: &Ident) -> String
    where
        T: Hash,
    {
        use std::hash::{
            Hash,
            Hasher,
        };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.args.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn _get_unique_ident(&self, prefix: Ident, ident: &Ident) -> Ident
    where
        T: Hash,
    {
        let hash = self._concat_ident_hash(ident);
        format_ident!("{prefix}_{hash}")
    }
}

pub trait ItemAttributePlugin {
    fn plugin(&self) -> &syn::Path;
}

impl<T, G, Resolver> ItemAttributePlugin for ItemAttribute<Composed<T, WithPlugin, G>, Resolver> {
    fn plugin(&self) -> &Path {
        self.args.plugin()
    }
}

pub trait ItemAttributeContext {
    fn context(&self) -> &Context;
}

impl<T, Resolver> ItemAttributeContext for ItemAttribute<T, Resolver> {
    fn context(&self) -> &Context {
        &self.context
    }
}

pub trait ItemAttributeTarget {
    fn target(&self) -> &syn::Path;
}

impl<T, Resolver> ItemAttributeTarget for ItemAttribute<T, Resolver>
where
    T: AttributeIdent + Hash + Clone,
    Resolver: Clone,
{
    fn target(&self) -> &syn::Path {
        &self.target
    }
}

pub trait ItemAttributeUniqueIdent: ItemAttributeTarget + ItemAttributeArgs {
    fn get_unique_ident(&self) -> Ident;
}

impl<T, Resolver> ItemAttributeUniqueIdent for ItemAttribute<T, Resolver>
where
    ItemAttribute<T, Resolver>: ItemAttributeArgs,
    T: AttributeIdent + Hash + Clone,
    Resolver: Clone,
{
    fn get_unique_ident(&self) -> Ident {
        self._get_unique_ident(
            ItemAttribute::<T, Resolver>::global_build_prefix(),
            self.target.get_ident().unwrap(), // infallible
        )
    }
}

pub trait ItemAttributeInput {
    fn input_item(&self) -> &InputItem;
    fn input_item_mut(&mut self) -> &mut InputItem;
}

impl<T, Resolver> ItemAttributeInput for ItemAttribute<T, Resolver> {
    fn input_item(&self) -> &InputItem {
        &self.input_item
    }
    fn input_item_mut(&mut self) -> &mut InputItem {
        &mut self.input_item
    }
}

pub trait ItemAttributeParse {
    fn from_attr_input_with_context(
        attr: TokenStream,
        input: TokenStream,
        context: Context,
    ) -> syn::Result<Self>
    where
        Self: Sized + ItemAttributeInput + ItemAttributeContext + ItemAttributeArgs;
}

impl<T, Resolver> ItemAttributeParse for ItemAttribute<T, Resolver>
where
    T: FromMeta,
    Resolver: IdentPathResolver,
{
    fn from_attr_input_with_context(
        attr: TokenStream,
        input: TokenStream,
        context: Context,
    ) -> syn::Result<Self> {
        Self::from_attr_input(attr, input, context)
    }
}

impl<T, Resolver> ItemAttribute<T, Resolver>
where
    T: FromMeta,
    Resolver: IdentPathResolver,
{
    pub fn from_attr_input(
        attr: TokenStream,
        input: TokenStream,
        context: Context,
    ) -> syn::Result<Self> {
        let mut input_item = InputItem::Tokens(input);
        let item = input_item.ensure_ast()?;
        let Some(target) = Resolver::resolve_ident_path(item) else {
            // call-site because we want to highlight the attribute
            return Err(syn::Error::new(Span::call_site(), Resolver::NOT_ALLOWED_MESSAGE));
        };
        // Parse the attribute’s inner tokens as: Meta, Meta, ...
        let metas: Punctuated<NestedMeta, Token![,]> =
            Punctuated::<NestedMeta, Token![,]>::parse_terminated.parse2(attr)?;

        // darling expects a slice of `Meta`
        let metas_vec: Vec<NestedMeta> = metas.into_iter().collect();

        Ok(Self {
            args: T::from_list(&metas_vec)?,
            context,
            input_item,
            target,
            _resolver: PhantomData,
        })
    }
}

impl<C, M1, M2, R> GenericsCap for ItemAttribute<Composed<C, M1, M2>, R>
where
    M2: HasGenerics,
{
    fn concrete_paths(&self) -> syn::Result<Vec<syn::Path>> {
        let target = &self.target;
        let target_params = self.input_item.type_param_idents()?;
        // TODO: check if generics counts match?
        if self.args.generics.generics().is_empty() {
            Ok(vec![target.clone()])
        } else {
            let mut paths = Vec::new();
            for generics in self.args.generics.generics() {
                let resolved = generics.resolve_types(&target_params)?;
                let path = if resolved.is_empty() {
                    target.clone()
                } else {
                    syn::parse_quote!(#target::<#(#resolved),*>)
                };
                paths.push(path);
            }
            Ok(paths)
        }
    }
}
