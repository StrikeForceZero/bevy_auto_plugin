use proc_macro2::Ident;
use syn::ItemFn;

pub struct FnParamMutabilityCheckErrMessages {
    pub not_mutable_message: String,
    pub not_found_message: String,
}

pub fn is_fn_param_mutable_reference(
    item: &ItemFn,
    param_ident: &Ident,
    messages: FnParamMutabilityCheckErrMessages,
) -> syn::Result<()> {
    use crate::__private::util::ty_classify;
    use syn::spanned::Spanned;
    use syn::{Error, FnArg, Pat};
    for arg in &item.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let Pat::Ident(pat_ident) = &*pat_type.pat else {
                continue;
            };
            if *param_ident != pat_ident.ident {
                continue;
            }
            if !ty_classify::is_mutable_reference(&pat_type.ty) {
                return Err(Error::new(pat_type.span(), messages.not_mutable_message));
            }
            return Ok(());
        }
    }
    Err(Error::new(
        item.sig.inputs.span(),
        messages.not_found_message,
    ))
}

pub fn require_fn_param_mutable_reference(
    item: &ItemFn,
    param_ident: &Ident,
    context: &str,
) -> syn::Result<()> {
    let fn_ident = &item.sig.ident;
    is_fn_param_mutable_reference(
        item,
        param_ident,
        FnParamMutabilityCheckErrMessages {
            not_mutable_message: format!(
                "{context} - the function: {fn_ident} param: {param_ident} is not mutable"
            ),
            not_found_message: format!(
                "{context} - the function: {fn_ident} param: {param_ident} not found in the function signature."
            ),
        },
    )?;
    Ok(())
}
