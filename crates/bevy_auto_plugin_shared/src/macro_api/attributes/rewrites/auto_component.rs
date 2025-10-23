use crate::__private::attribute::RewriteAttribute;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::ast::flag_or_lit::FlagOrLit;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
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

impl RewriteAttribute for ComponentArgs {
    fn expand_args(&self, plugin: &NonEmptyPath) -> MacroStream {
        let mut args = Vec::new();
        args.push(quote! { plugin = #plugin });
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        quote! { #(#args),* }
    }

    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();

        if self.derive.present {
            expanded_attrs
                .attrs
                .push(tokens::derive_component(&self.derive.items));
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Component);
            let items = std::iter::once(&component_ident).chain(self.reflect.items.iter());
            expanded_attrs.append(tokens::reflect(items))
        }
        if self.register {
            expanded_attrs
                .attrs
                .push(tokens::auto_register_type(plugin.clone(), self.into()));
        }
        if self.auto_name.present {
            expanded_attrs
                .attrs
                .push(tokens::auto_name(plugin.clone(), self.into()));
        }
        expanded_attrs
    }
}

pub type Component =
    ItemAttribute<Composed<ComponentArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QComponentArgs<'a> = Q<'a, Component>;
impl ToTokens for QComponentArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}
