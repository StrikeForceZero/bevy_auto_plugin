use crate::syntax::ast::type_list::TypeList;
use quote::ToTokens;
use syn::{
    Path,
    PathArguments,
    parse2,
};

pub fn generics_from_path(path: &Path) -> syn::Result<TypeList> {
    let mut generics = TypeList::empty();
    for segment in &path.segments {
        if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
            let type_list = parse2::<TypeList>(angle_bracketed.args.to_token_stream())?;
            generics.0.extend(type_list.0);
        }
    }
    Ok(generics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::analysis::path::generics_from_path;
    use internal_test_proc_macro::xtest;
    use quote::quote;

    #[xtest]
    fn test_generics_from_path() -> Result<(), syn::Error> {
        let item = parse2::<Path>(quote! {
            foo::bar::<u32, i32>
        })?;
        let generics = generics_from_path(&item).expect("no generics");
        let generics = quote! { #generics };
        assert_eq!("u32 , i32", generics.to_string());
        Ok(())
    }
}
