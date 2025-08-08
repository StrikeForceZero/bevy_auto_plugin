use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Path, PathArguments, PathSegment};

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

pub fn ident_to_path(ident: &Ident) -> Path {
    Path {
        leading_colon: None,
        segments: {
            let mut segments = Punctuated::new();
            segments.push(PathSegment {
                ident: ident.clone(),
                arguments: PathArguments::None,
            });
            segments
        },
    }
}
