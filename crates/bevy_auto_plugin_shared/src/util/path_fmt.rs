use quote::{ToTokens, quote};
use syn::Path;
use syn::parse::Parse;
pub fn path_to_string(path: &Path, strip_spaces: bool) -> String {
    let path_string = quote!(#path).to_string();
    if strip_spaces {
        path_string.replace(" ", "")
    } else {
        path_string
    }
}

pub fn path_to_string_with_spaces(path: &Path) -> String {
    path_to_string(path, false)
}
