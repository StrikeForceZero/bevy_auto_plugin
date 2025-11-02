pub mod token_stream;
pub mod ui_util;

use bevy_app::App;
use bevy_internal::MinimalPlugins;
use darling::ast::NestedMeta;
use std::any::TypeId;
use syn::{
    punctuated::Punctuated,
    token::Comma,
};

pub fn create_minimal_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

pub fn type_id_of<T>() -> TypeId
where
    T: 'static,
{
    TypeId::of::<T>()
}

pub fn extract_punctuated_paths(punctuated: Punctuated<NestedMeta, Comma>) -> Vec<syn::Path> {
    punctuated
        .into_iter()
        .map(|nested_meta| match nested_meta {
            NestedMeta::Meta(meta) => {
                meta.require_path_only().cloned().expect("expected path only")
            }
            NestedMeta::Lit(_) => panic!("unexpected literal"),
        })
        .collect::<Vec<_>>()
}

/// `vec_spread![a, ..iter_or_collection, b, ..more]`
/// Works with any `IntoIterator` after `..`.
#[macro_export]
macro_rules! vec_spread {
    // empty
    [] => { ::std::vec::Vec::new() };

    // one spread, optional trailing comma
    [ .. $iter:expr $(,)? ] => {{
        ::std::iter::IntoIterator::into_iter($iter).collect::<::std::vec::Vec<_>>()
    }};

    // one element, optional trailing comma
    [ $elem:expr $(,)? ] => {{
        ::std::vec![$elem]
    }};

    // first is spread; then more
    [ .. $iter:expr, $($rest:tt)+ ] => {{
        let mut __v = ::std::iter::IntoIterator::into_iter($iter).collect::<::std::vec::Vec<_>>();
        #[allow(clippy::vec_init_then_push)]
        __v.extend($crate::vec_spread![$($rest)+]);
        __v
    }};

    // first is element; then more
    [ $elem:expr, $($rest:tt)+ ] => {{
        let mut __v = ::std::vec![$elem];
        #[allow(clippy::vec_init_then_push)]
        __v.extend($crate::vec_spread![$($rest)+]);
        __v
    }};
}

#[cfg(test)]
mod tests {
    use internal_test_proc_macro::xtest;
    #[xtest]
    fn test_vec_spread() {
        assert_eq!(vec_spread![1], vec![1]);
        assert_eq!(vec_spread![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(vec_spread![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(vec_spread![1, 2, 3, ..[4, 5, 6], 7, 8], vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(vec_spread![..[4, 5, 6], 7, 8], vec![4, 5, 6, 7, 8]);
        assert_eq!(vec_spread![1, 2, 3, ..[4, 5, 6]], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(vec_spread![1, 2, 3, ..[4, 5, 6], 7, 8, ..[9]], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
