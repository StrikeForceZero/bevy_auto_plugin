use crate::type_list::TypeList;
use crate::util::concrete_path;
use syn::Path;

pub trait PathExt {
    fn has_generics(&self) -> syn::Result<bool>;
    fn generics(&self) -> syn::Result<TypeList>;
    fn generic_count(&self) -> syn::Result<usize> {
        Ok(self.generics()?.len())
    }
}

impl PathExt for Path {
    fn has_generics(&self) -> syn::Result<bool> {
        Ok(!self.generics()?.is_empty())
    }

    fn generics(&self) -> syn::Result<TypeList> {
        concrete_path::generics_from_path(self)
    }
}
