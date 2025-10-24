use crate::__private::attribute::RewriteAttribute;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ObserverArgs {}

impl AttributeIdent for ObserverArgs {
    const IDENT: &'static str = "auto_observer";
}

impl<'a> From<&'a ObserverArgs> for RegisterTypeArgs {
    fn from(value: &'a ObserverArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a ObserverArgs> for AddObserverArgs {
    fn from(value: &'a ObserverArgs) -> Self {
        AddObserverArgs {}
    }
}

impl RewriteAttribute for ObserverArgs {
    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();
        expanded_attrs
            .attrs
            .push(tokens::auto_add_observer(plugin.clone(), self.into()));
        expanded_attrs
    }
}

pub type IaObserver =
    ItemAttribute<Composed<ObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type QObserver<'a> = Q<'a, IaObserver>;
impl ToTokens for QObserver<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}
