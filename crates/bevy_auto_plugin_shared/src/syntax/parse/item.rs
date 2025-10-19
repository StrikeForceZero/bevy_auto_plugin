use crate::syntax::extensions::item::ItemAttrsExt;
use crate::syntax::extensions::path::PathExt;
use proc_macro2::TokenStream;
use syn::{Attribute, Item, parse2};

pub fn ts_item_has_attr(input: TokenStream, path: &syn::Path) -> syn::Result<bool> {
    let item = parse2::<Item>(input)?;
    item_has_attr(item, path)
}

pub fn item_has_attr(item: Item, path: &syn::Path) -> syn::Result<bool> {
    Ok(has_attr(item.attrs().unwrap_or_default(), path))
}

pub fn has_attr(attrs: &[Attribute], path: &syn::Path) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_similar_path_or_ident(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::parse_quote;

    #[xtest]
    fn test_negative_has_attr_empty() {
        let target_path = parse_quote!(a);
        assert!(!has_attr(&[], &target_path));
    }

    #[xtest]
    fn test_negative_has_attr() {
        let target_path = parse_quote!(a);
        let input: Vec<_> = parse_quote! {
            #[foo]
        };
        assert!(!has_attr(&input, &target_path));
    }

    #[xtest]
    fn test_positive_has_attr_single() {
        let target_path = parse_quote!(a);
        let input: Vec<_> = parse_quote! {
            #[#target_path]
        };
        assert!(has_attr(&input, &target_path));
    }

    #[xtest]
    fn test_positive_has_attr_multiple() {
        let target_path = parse_quote!(a);
        let input: Vec<_> = parse_quote! {
            #[A]
            #[#target_path]
            #[B]
        };
        assert!(has_attr(&input, &target_path));
    }
}
