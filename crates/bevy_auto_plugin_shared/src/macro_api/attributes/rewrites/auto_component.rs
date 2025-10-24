use crate::__private::attribute::RewriteAttribute;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::ast::flag_or_lit::FlagOrLit;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use crate::util::macros::impl_from_default;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};
use syn::parse_quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ComponentArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub auto_name: FlagOrLit,
}

impl AttributeIdent for ComponentArgs {
    const IDENT: &'static str = "auto_component";
}

impl<'a> From<&'a ComponentArgs> for RegisterTypeArgs {
    fn from(value: &'a ComponentArgs) -> Self {
        Self {}
    }
}

impl<'a> From<&'a ComponentArgs> for NameArgs {
    fn from(value: &'a ComponentArgs) -> Self {
        Self {
            name: value.auto_name.lit.clone(),
        }
    }
}

pub type IaComponent =
    ItemAttribute<Composed<ComponentArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type RewriteQComponent = RewriteQ<IaComponent>;

impl RewriteQToExpandAttr for RewriteQComponent {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs) {
        if self.args.args.base.derive.present {
            expand_attrs
                .attrs
                .push(tokens::derive_component(&self.args.args.base.derive.items));
        }
        if self.args.args.base.reflect.present {
            if self.args.args.base.derive.present {
                expand_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Component);
            let items =
                std::iter::once(&component_ident).chain(self.args.args.base.reflect.items.iter());
            expand_attrs.append(tokens::reflect(items))
        }
        if self.args.args.base.register {
            expand_attrs
                .attrs
                .push(tokens::auto_register_type(self.into()));
        }
        if self.args.args.base.auto_name.present {
            expand_attrs.attrs.push(tokens::auto_name(self.into()));
        }
    }
}

impl_from_default!(ComponentArgs => (RegisterTypeArgs, NameArgs));
