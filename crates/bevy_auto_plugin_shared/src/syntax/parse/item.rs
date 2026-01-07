use proc_macro2::TokenStream;
use syn::{
    File,
    Item,
    ItemMacro,
    Macro,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SingleItemWithErrorsCheckError {
    #[error("failed to parse token stream as a single item: {0}")]
    ParseFailed(#[from] syn::Error),
    #[error("token stream did not contain an item")]
    NoRealItem,
    #[error("token stream contained more than one item")]
    MultipleRealItems,
}

/// Returns the single Item (struct/enum/etc) if the token stream is:
///     `Item` [+ zero or more `compile_error!` items]
/// Fails otherwise.
pub fn expect_single_item_any_compile_errors(
    ts: TokenStream,
) -> Result<(Item, Vec<ItemMacro>), SingleItemWithErrorsCheckError> {
    let file: File = syn::parse2(ts).map_err(SingleItemWithErrorsCheckError::ParseFailed)?;

    let mut compile_errors = vec![];
    let mut found_item: Option<Item> = None;

    for item in file.items.into_iter() {
        if let Some(compiler_error) = as_compile_error_item(&item) {
            compile_errors.push(compiler_error.clone());
            continue;
        }

        // It's a "real" item
        match &found_item {
            None => {
                // first real item we've seen — keep it
                found_item = Some(item);
            }
            Some(_) => {
                // second real item → reject
                return Err(SingleItemWithErrorsCheckError::MultipleRealItems);
            }
        }
    }

    match found_item {
        Some(item) => Ok((item, compile_errors)),
        None => Err(SingleItemWithErrorsCheckError::NoRealItem),
    }
}

/// returns `Some(ItemMacro)` if `compile_error!(...)` ?
fn as_compile_error_item(item: &Item) -> Option<&ItemMacro> {
    match item {
        Item::Macro(item_macro) => {
            let ItemMacro { mac: Macro { path, .. }, .. } = item_macro;
            if path.is_ident("compile_error") { Some(item_macro) } else { None }
        }
        _ => None,
    }
}
