use crate::__private::util::meta::fn_meta::require_fn_param_mutable_reference;

pub mod flat_file;
pub mod global;
pub mod module;

use syn::{FnArg, Ident, ItemFn, Pat, spanned::Spanned};

pub fn resolve_app_param_name<'a>(
    input: &'a ItemFn,
    app_param_name: Option<&'a Ident>,
) -> syn::Result<&'a Ident> {
    // Helper: pick a useful Span for errors
    let err_span = || {
        app_param_name
            .map(Ident::span)
            .unwrap_or_else(|| input.sig.span())
    };

    // Helper: try to get &Ident from a typed arg
    fn ident_from_typed_arg(arg: &FnArg) -> Option<&Ident> {
        match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_ty) => match &*pat_ty.pat {
                Pat::Ident(pat_ident) => Some(&pat_ident.ident),
                _ => None,
            },
        }
    }

    let has_self = input
        .sig
        .inputs
        .iter()
        .any(|a| matches!(a, FnArg::Receiver(_)));

    // collect all named params
    let named = input
        .sig
        .inputs
        .iter()
        .filter_map(ident_from_typed_arg)
        .collect::<Vec<_>>();

    // If user explicitly provided a name, validate it exists and return it
    if let Some(given) = app_param_name.as_ref() {
        if let Some(found) = named.iter().copied().find(|id| id == given) {
            return Ok(found);
        }
        return Err(syn::Error::new(
            err_span(),
            format!(
                "auto_plugin provided app_param: `{}` but it was not found in the function signature",
                given
            ),
        ));
    }

    // Otherwise infer. We need exactly one named param after any receiver(s).
    match named.as_slice() {
        [] => {
            if has_self {
                Err(syn::Error::new(
                    err_span(),
                    "auto_plugin requires a method taking at least one parameter in addition to `&self`",
                ))
            } else {
                Err(syn::Error::new(
                    err_span(),
                    "auto_plugin requires a function taking at least one named parameter",
                ))
            }
        }
        [only] => {
            // Enforce `&mut App` rule if you have such a checker:
            require_fn_param_mutable_reference(input, only, "auto_plugin")?;
            Ok(*only)
        }
        _more => Err(syn::Error::new(
            err_span(),
            "auto_plugin requires specifying the name of the `&mut bevy::app::App` parameter \
             when there’s more than one parameter, e.g.:
             #[auto_plugin(app_param=my_app)]
             fn my_plugin(my_app: &mut App) { /* ... */ }",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[internal_test_proc_macro::xtest]
    #[should_panic = "auto_plugin provided app_param: `bar` but it was not found in the function signature"]
    fn test_resolve_app_param_name_wrong_specified() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo: &mut App) {}
        };
        let target_ident = Ident::new("bar", Span::call_site());
        let ident = resolve_app_param_name(&item, Some(&target_ident)).unwrap();
        assert_eq!(ident, "foo");
    }

    #[internal_test_proc_macro::xtest]
    fn test_resolve_app_param_name_specified() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo1: &mut App, foo2: &mut App, foo3: &mut App) {}
        };
        let target_ident = Ident::new("foo2", Span::call_site());
        let ident = resolve_app_param_name(&item, Some(&target_ident)).unwrap();
        assert_eq!(ident, "foo2");
    }

    #[internal_test_proc_macro::xtest]
    fn test_resolve_app_param_name_default() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo: &mut App) {}
        };
        let ident = resolve_app_param_name(&item, None).unwrap();
        assert_eq!(ident, "foo");
    }

    #[internal_test_proc_macro::xtest]
    #[should_panic = "auto_plugin requires a function taking at least one named parameter"]
    fn test_resolve_app_param_name_default_no_params() {
        let item = syn::parse_quote! {
            fn foo() {}
        };
        match resolve_app_param_name(&item, None) {
            Ok(_) => {}
            Err(err) => panic!("{err:?}"),
        }
    }

    #[internal_test_proc_macro::xtest]
    #[should_panic = "auto_plugin requires a method taking at least one parameter in addition to `&self`"]
    fn test_resolve_app_param_name_default_missing_param() {
        let item = syn::parse_quote! {
            fn foo(&mut self) {}
        };
        match resolve_app_param_name(&item, None) {
            Ok(_) => {}
            Err(err) => panic!("{err:?}"),
        }
    }

    #[internal_test_proc_macro::xtest]
    #[should_panic = "auto_plugin - the function: foo param: foo is not mutable"]
    fn test_resolve_app_param_name_default_wrong_type() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo: &Bar) {}
        };
        match resolve_app_param_name(&item, None) {
            Ok(_) => {}
            Err(err) => panic!("{err:?}"),
        }
    }
}
