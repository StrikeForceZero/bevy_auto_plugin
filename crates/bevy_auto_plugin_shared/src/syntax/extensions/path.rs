use crate::syntax::analysis::path;
use crate::syntax::ast::type_list::TypeList;
use syn::Path;

pub trait PathExt {
    fn has_generics(&self) -> syn::Result<bool>;
    fn generics(&self) -> syn::Result<TypeList>;
    #[allow(dead_code)]
    fn generic_count(&self) -> syn::Result<usize> {
        Ok(self.generics()?.len())
    }

    fn is_similar_path_or_ident(&self, other: &Self) -> bool;
}

impl PathExt for Path {
    fn has_generics(&self) -> syn::Result<bool> {
        Ok(!self.generics()?.is_empty())
    }

    fn generics(&self) -> syn::Result<TypeList> {
        path::generics_from_path(self)
    }

    fn is_similar_path_or_ident(&self, other: &Self) -> bool {
        is_similar_path_or_ident(self, other)
    }
}

/// Compares paths from right to left. Only returns false if a segment is different, not missing.
///
/// Examples:
/// `::some::path == some::path == ::path == path`
/// `::some::other::path != some::path`
fn is_similar_path_or_ident(a: &Path, b: &Path) -> bool {
    for (a, b) in a.segments.iter().rev().zip(b.segments.iter().rev()) {
        if a.ident != b.ident || a.arguments != b.arguments {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::parse_quote;

    mod is_similar_path_or_ident {
        use super::*;
        #[xtest]
        #[rustfmt::skip]
        fn test_is_true() {
            let a = parse_quote! { ::some::path };
            assert!(is_similar_path_or_ident(&a, &parse_quote! { some::path }));
            assert!(is_similar_path_or_ident(&a, &parse_quote! { ::path }));
            assert!(is_similar_path_or_ident(&a, &parse_quote! { path }));
        }

        #[xtest]
        #[rustfmt::skip]
        fn test_is_false() {
            let a = parse_quote! { ::some::path };    
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { ::some::other::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { ::other::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { other::path }));
        }

        // these are not valid paths, but we want to make sure they are handled correctly
        #[xtest]
        #[rustfmt::skip]
        fn test_with_arguments_is_true() {
            let a = parse_quote! { ::some<T>::path<T> };    
            assert!(is_similar_path_or_ident(&a, &parse_quote! { some<T>::path<T> }));
            assert!(is_similar_path_or_ident(&a, &parse_quote! { ::path<T> }));
            assert!(is_similar_path_or_ident(&a, &parse_quote! { path<T> }));
        }

        #[xtest]
        #[rustfmt::skip]
        fn test_with_arguments_is_false() {
            let a = parse_quote! { ::some<T>::path<T> };    
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some<T>::path<F> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some<A>::path<T> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some::path<T> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some::path<F> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some<T>::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some<F>::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { some::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { ::path<F> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { ::path }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { path<F> }));
            assert!(!is_similar_path_or_ident(&a, &parse_quote! { path }));
        }
    }
}
