use crate::__private::attribute::RewriteAttribute;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::prelude::*;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StatesArgs {
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl AttributeIdent for StatesArgs {
    const IDENT: &'static str = "auto_states";
}

impl<'a> From<&'a StatesArgs> for RegisterTypeArgs {
    fn from(_value: &'a StatesArgs) -> Self {
        Self::default()
    }
}

impl<'a> From<&'a StatesArgs> for InitStateArgs {
    fn from(_value: &'a StatesArgs) -> Self {
        Self::default()
    }
}

impl RewriteAttribute for StatesArgs {
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
            expanded_attrs.append(tokens::derive_states(&self.derive.items));
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            expanded_attrs.append(tokens::reflect(&self.reflect.items))
        }
        if self.register {
            expanded_attrs
                .attrs
                .push(tokens::auto_register_type(plugin.clone(), self.into()));
        }
        if self.init {
            expanded_attrs
                .attrs
                .push(tokens::auto_init_states(plugin.clone(), self.into()));
        }
        expanded_attrs
    }
}

impl ToTokens
    for Q<
        '_,
        ItemAttribute<Composed<StatesArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>,
    >
{
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}
