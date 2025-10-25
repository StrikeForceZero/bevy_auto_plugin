use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::util::macros::impl_from_default;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ObserverArgs {}

impl AttributeIdent for ObserverArgs {
    const IDENT: &'static str = "auto_observer";
}

impl<'a> From<&'a ObserverArgs> for RegisterTypeArgs {
    fn from(_: &'a ObserverArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a ObserverArgs> for AddObserverArgs {
    fn from(_: &'a ObserverArgs) -> Self {
        AddObserverArgs {}
    }
}

pub type IaObserver =
    ItemAttribute<Composed<ObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type RewriteQObserver = RewriteQ<IaObserver>;

impl RewriteQToExpandAttr for RewriteQObserver {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        expand_attrs
            .attrs
            .push(tokens::auto_add_observer(self.into()));
    }
}

impl_from_default!(ObserverArgs => (AddObserverArgs));
