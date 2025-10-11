use bevy_auto_plugin::prelude::*;
use quote::quote;

#[inheritable]
#[allow(dead_code)]
#[derive(Debug)]
struct Foo;

inherit_Foo! {
    #[derive(Copy, Clone)]
    struct Bar;
}

#[test]
fn test_inheritable() {
    let tokens = test_inherit_Foo! {
        #[derive(Copy, Clone)]
        struct Bar;
    };
    assert_eq!(
        tokens.to_string(),
        quote! {
            #[::bevy_auto_plugin::__private::_inherit_or_merge(target(allow, derive))]
            #[allow(dead_code)]
            #[derive(Debug)]
            #[::bevy_auto_plugin::__private::_end_inherit_or_merge]
            #[derive(Copy, Clone)]
            struct Bar;
        }
        .to_string(),
        "failed to inherit Foo"
    );
}
