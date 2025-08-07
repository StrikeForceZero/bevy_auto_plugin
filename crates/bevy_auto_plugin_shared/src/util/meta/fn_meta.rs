use crate::util::meta::IdentGenericsAttrsMeta;
use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item, ItemFn};

pub struct FnMeta<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a Vec<Attribute>,
}

impl<'a> FnMeta<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a Vec<Attribute>) -> Self {
        Self {
            ident,
            generics,
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Item> for FnMeta<'a> {
    type Error = Error;

    fn try_from(item: &'a Item) -> std::result::Result<Self, Self::Error> {
        use syn::spanned::Spanned;
        Ok(match item {
            Item::Fn(fn_item) => {
                Self::new(&fn_item.sig.ident, &fn_item.sig.generics, &fn_item.attrs)
            }
            _ => return Err(Error::new(item.span(), "expected fn")),
        })
    }
}

impl<'a> IdentGenericsAttrsMeta<'a> for FnMeta<'a> {
    fn ident(&self) -> &Ident {
        self.ident
    }
    fn generics(&self) -> &Generics {
        self.generics
    }
    fn attributes(&self) -> &Vec<Attribute> {
        self.attributes
    }
}

pub struct FnParamMutabilityCheckErrMessages {
    pub not_mutable_message: String,
    pub not_found_message: String,
}

pub fn is_fn_param_mutable_reference(
    item: &ItemFn,
    param_ident: &Ident,
    messages: FnParamMutabilityCheckErrMessages,
) -> syn::Result<()> {
    use crate::util::ty_classify;
    use syn::spanned::Spanned;
    use syn::{FnArg, Pat};
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
