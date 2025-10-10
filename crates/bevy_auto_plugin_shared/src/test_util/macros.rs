macro_rules! parse_attribute_args_with_plugin {
    // with meta args
    ($plugin:expr, $args_ident:ident, $tokens:expr $(,)?) => {{
        use quote::quote;
        use $crate::codegen::tokens::ArgsWithPlugin;
        use $crate::macro_api::global_args::{AttributeIdent, GlobalArgs};
        let plugin = $plugin.clone();
        let macro_path = <$args_ident as AttributeIdent>::full_attribute_path();

        let mut args = vec![
            quote!(plugin = #plugin)
        ];

        if !$tokens.is_empty() {
            args.push($tokens);
        }

        let input = quote! { #[#macro_path( #(#args),* )] };
        let attr: syn::Attribute = syn::parse_quote! { #input };
        let args_with_plugin = ArgsWithPlugin::from(GlobalArgs::<$args_ident>::from_meta(&attr.meta)?);
        (plugin, input, args_with_plugin)
    }};
}

macro_rules! parse_vec_args {
    ($plugin:expr, $args_ident:ident, $args:expr $(,)?) => {{
        let args = $args;
        $crate::test_util::macros::parse_attribute_args_with_plugin!(
            $plugin,
            $args_ident,
            quote! { #(#args),* }
        )
    }};

    ($plugin:expr, $args_ident:ident $(,)?) => {{ $crate::test_util::macros::parse_attribute_args_with_plugin!($plugin, $args_ident) }};
}

macro_rules! assert_vec_args_expand {
    ($plugin:expr, $args_ident:ident, $args:expr $(,)?) => {{
        let (plugin, input, args) =
            $crate::test_util::macros::parse_vec_args!($plugin, $args_ident, $args);
        $crate::test_util::assert_tokens_match(plugin, input, args);
    }};

    ($plugin:expr, $args_ident:ident $(,)?) => {{
        let (plugin, input, args) = $crate::parse_vec_args!($plugin, $args_ident);
        $crate::test_util::assert_tokens_match(plugin, input, args);
    }};
}

macro_rules! plugin {
    ($plugin:expr) => {{ $crate::syntax::validated::non_empty_path::NonEmptyPath::new_unchecked($plugin) }};
}

#[rustfmt::skip]
pub(crate) use {
    parse_attribute_args_with_plugin,
    parse_vec_args,
    assert_vec_args_expand,
    plugin,
};
