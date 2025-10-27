use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute,
    ItemFn,
    parse_macro_input,
};

/// #[xtest] => #[wasm_bindgen_test] on wasm32, else #[xtest]
#[proc_macro_attribute]
pub fn xtest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    // Keep all attrs except any existing #[xtest] / #[wasm_bindgen_test]
    let attrs: Vec<Attribute> = func
        .attrs
        .into_iter()
        .filter(|a| {
            let p = a.path();
            !(p.is_ident("test") || p.is_ident("wasm_bindgen_test"))
        })
        .collect();

    // Emit attributes conditionally via cfg_attr
    let out = quote! {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        #(#attrs)*
        #vis #sig #block
    };
    out.into()
}

#[test]
fn it_works() -> syn::Result<()> {
    let func: ItemFn = syn::parse_quote! {
        #[xtest]
        #[bar]
        fn foo() {}
    };
    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    // Remove self attribute
    let attrs: Vec<Attribute> = func
        .attrs
        .into_iter()
        .filter(|a| {
            let p = a.path();
            !(p.is_ident("xtest"))
        })
        .collect();

    // Emit attributes conditionally via cfg_attr
    let output: ItemFn = syn::parse_quote! {
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        #(#attrs)*
        #vis #sig #block
    };

    fn map_ident(a: &Attribute) -> String {
        a.path().get_ident().unwrap().to_string()
    }

    assert_eq!(
        output.attrs.iter().map(map_ident).collect::<Vec<_>>(),
        vec!["cfg_attr", "cfg_attr", "bar"]
    );

    Ok(())
}
