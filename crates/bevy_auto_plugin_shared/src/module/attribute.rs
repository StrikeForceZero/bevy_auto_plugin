use proc_macro2::{Ident, Span};
use syn::meta::ParseNestedMeta;

#[derive(Default)]
pub struct AutoPluginAttributes {
    pub init_name: Option<Ident>,
}

impl AutoPluginAttributes {
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("init_name") {
            self.init_name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    }
    pub fn init_name(&self) -> Ident {
        self.init_name
            .as_ref()
            .cloned()
            .unwrap_or(Ident::new("init", Span::call_site()))
    }
}