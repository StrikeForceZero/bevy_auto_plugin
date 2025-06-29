use proc_macro2::Ident;
use syn::meta::ParseNestedMeta;

#[derive(Default)]
pub struct AutoPluginAttributes {
    pub app_param_name: Option<Ident>,
}

impl AutoPluginAttributes {
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("app") {
            self.app_param_name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    }
}
