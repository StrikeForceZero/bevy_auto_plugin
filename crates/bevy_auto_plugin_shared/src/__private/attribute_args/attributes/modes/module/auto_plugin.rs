use crate::__private::attribute_args::GenericsArgs;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::Ident;

#[derive(FromMeta, Debug, Default, Clone, PartialEq)]
#[darling(derive_syn_parse, default)]
pub struct AutoPluginArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(default)]
    pub init_name: Option<Ident>,
}

impl GenericsArgs for AutoPluginArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}
