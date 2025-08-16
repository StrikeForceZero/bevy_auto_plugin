use crate::__private::attribute::AutoPluginItemAttribute;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Path, parse_quote};

pub mod component;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Global { plugin: Path },
    FlatFile,
    Module,
}

impl Mode {
    fn resolve_macro_path(&self, attr: AutoPluginItemAttribute) -> Path {
        let mode = match self {
            Mode::Global { .. } => "global",
            Mode::FlatFile => "flat_file",
            Mode::Module => "module",
        };
        let mode_ident = quote::format_ident!("{}", mode);
        let macro_ident = quote::format_ident!("{}", attr.ident_str());
        parse_quote!(:: bevy_auto_plugin :: modes :: #mode_ident :: prelude :: #macro_ident)
    }
}

#[derive(Debug, Default)]
pub struct ExpandAttrs {
    pub attrs: Vec<MacroStream>,
    pub use_items: Vec<MacroStream>,
}

impl ToTokens for ExpandAttrs {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        let use_items = &self.use_items;
        tokens.extend(quote! {
            #(#use_items)*

        });
        let attrs = &self.attrs;
        tokens.extend(quote! {
            #(#attrs)*
        });
    }
}

pub trait ShortHandAttribute {
    fn expand_args(&self, mode: &Mode) -> MacroStream;
    fn expand_attrs(&self, mode: &Mode) -> ExpandAttrs;
}
