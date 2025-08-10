use darling::FromMeta;
use syn::Meta;

pub trait FromMetaExt: FromMeta {
    fn from_meta_ext(meta: &Meta) -> darling::Result<Self>;
}

impl<T: FromMeta> FromMetaExt for T {
    fn from_meta_ext(meta: &Meta) -> darling::Result<Self> {
        match meta {
            Meta::Path(_) => T::from_list(&[]),
            _ => T::from_meta(meta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{Attribute, parse_quote};

    #[derive(FromMeta, Debug, Default, PartialEq)]
    #[darling(default)]
    struct TestList {
        a: Option<String>,
        b: Option<String>,
    }

    #[derive(FromMeta, Debug, Default, PartialEq)]
    #[darling(default)]
    struct TestSingle(Option<String>);

    #[internal_test_proc_macro::xtest]
    #[should_panic(expected = "Unexpected meta-item format `word`")]
    fn test_from_meta_word_panic() {
        let attr: Attribute = parse_quote!(#[foo]);
        match TestList::from_meta(&attr.meta) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }
    }

    #[internal_test_proc_macro::xtest]
    fn test_from_meta_ext_word() {
        let attr: Attribute = parse_quote!(#[foo]);
        assert_eq!(
            TestList::from_meta_ext(&attr.meta).ok(),
            Some(TestList::default())
        );
    }

    #[internal_test_proc_macro::xtest]
    fn test_from_meta_ext_list() {
        let attr: Attribute = parse_quote!(#[foo(a = "bar")]);
        assert_eq!(
            TestList::from_meta_ext(&attr.meta).ok(),
            Some(TestList {
                a: Some("bar".to_string()),
                ..Default::default()
            })
        );
        let attr: Attribute = parse_quote!(#[foo(a = "bar", b = "baz")]);
        assert_eq!(
            TestList::from_meta_ext(&attr.meta).ok(),
            Some(TestList {
                a: Some("bar".to_string()),
                b: Some("baz".to_string()),
            })
        );
    }

    #[internal_test_proc_macro::xtest]
    // Meta::Path on tuple struct unsupported
    #[should_panic(expected = "Unexpected meta-item format `word`")]
    fn test_from_meta_ext_name_value_empty() {
        let attr: Attribute = parse_quote!(#[foo]);
        match TestList::from_meta(&attr.meta) {
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }
    }

    #[internal_test_proc_macro::xtest]
    fn test_from_meta_ext_name_value() {
        let attr: Attribute = parse_quote!(#[foo = "bar"]);
        assert_eq!(
            TestSingle::from_meta_ext(&attr.meta).ok(),
            Some(TestSingle(Some("bar".to_string())))
        );
    }
}
