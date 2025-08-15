use crate::__private::util::debug::debug_pat;
use crate::__private::util::debug::debug_ty;
use crate::__private::util::meta::fn_meta::require_fn_param_mutable_reference;
use proc_macro2::Ident;
use syn::{FnArg, ItemFn, Pat};

pub mod flat_file;
pub mod global;
pub mod module;

pub fn resolve_app_param_name(input: &ItemFn, app_param_name: Option<Ident>) -> syn::Result<Ident> {
    use syn::spanned::Spanned;
    macro_rules! err {
        ($msg:expr) => {
            syn::Error::new(app_param_name.span(), $msg)
        };
    }

    #[derive(Debug, Clone, PartialEq)]
    enum ArgType {
        Named(Ident),
        SelfRef,
    }

    let extract_param = |arg: &FnArg| -> syn::Result<ArgType> {
        Ok(match arg {
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(syn::PatIdent { ident, .. }) = &*pat_type.pat {
                    ArgType::Named(ident.clone())
                } else {
                    let got_pat = debug_pat(&pat_type.pat);
                    let got_ty = debug_ty(&pat_type.ty);

                    return Err(err!(format!(
                        "auto_plugin default expects the functions first parameter to be `&mut bevy::app::App`, got: {got_ty} for the parameter {got_pat}"
                    )));
                }
            }
            // &self
            FnArg::Receiver(_) => ArgType::SelfRef,
        })
    };

    let mut args = input.sig.inputs.iter();

    // If the user explicitly supplied `app_param_name`, just use it:
    if let Some(name) = app_param_name {
        if !args.into_iter().any(|arg| {
            if let FnArg::Typed(pat) = arg
                && let Pat::Ident(syn::PatIdent { ident, .. }) = &*pat.pat
            {
                ident == &name
            } else {
                false
            }
        }) {
            return Err(syn::Error::new(
                name.span(),
                format!(
                    "auto_plugin provided app_param_name: `{name}` but it was not found in the function signature"
                ),
            ));
        }
        return Ok(name);
    }

    // Otherwise we expect exactly one parameter

    // pull out the first arg, or error if there is none
    let first = args
        .next()
        .ok_or_else(|| err!("auto_plugin requires a function taking at least one parameter"))?;

    // extract first param identifier
    let ident = match extract_param(first)? {
        ArgType::Named(name) => name,
        ArgType::SelfRef => {
            // extract second param identifier since first is &self
            let first = args
                .next()
                .ok_or_else(|| err!("auto_plugin requires a method taking at least one parameter in addition to `&self`"))?;
            match extract_param(first)? {
                ArgType::Named(name) => {
                    require_fn_param_mutable_reference(input, &name, "auto_plugin")?;
                    name
                }
                _ => unreachable!("invalid function signature, multiple receivers"),
            }
        }
    };

    // ensure there is *no* additional parameters
    if args.next().is_some() {
        return Err(err!(
            "auto_plugin requires specifying the name of the `&mut bevy::app::App` parameter when there’s more than one parameter, example:\n#[auto_plugin(app_param=my_app)]\nfn my_plugin(my_app: &mut App)"
        ));
    }

    Ok(ident)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[internal_test_proc_macro::xtest]
    #[should_panic = "auto_plugin provided app_param_name: `bar` but it was not found in the function signature"]
    fn test_resolve_app_param_name_wrong_specified() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo: &mut App) {}
        };
        let ident =
            resolve_app_param_name(&item, Some(Ident::new("bar", Span::call_site()))).unwrap();
        assert_eq!(ident, "foo");
    }

    #[internal_test_proc_macro::xtest]
    fn test_resolve_app_param_name_specified() {
        let item = syn::parse_quote! {
            fn foo(&mut self, foo1: &mut App, foo2: &mut App, foo3: &mut App) {}
        };
        let ident =
            resolve_app_param_name(&item, Some(Ident::new("foo2", Span::call_site()))).unwrap();
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
    #[should_panic = "auto_plugin requires a function taking at least one parameter"]
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
