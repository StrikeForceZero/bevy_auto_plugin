use crate::__private::attribute_args::GenericsArgs;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::Span;
use smart_default::SmartDefault;

#[derive(FromMeta, Debug, SmartDefault, Clone, PartialEq)]
#[darling(derive_syn_parse, default)]
pub struct AutoPluginArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[default(Ident::new("init", Span::call_site()))]
    pub init_name: Ident,
}

impl GenericsArgs for AutoPluginArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}
