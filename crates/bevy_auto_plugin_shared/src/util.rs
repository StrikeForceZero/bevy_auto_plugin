use crate::{AddSystemParams, AutoPluginAttribute};
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    Attribute, Error, FnArg, Generics, Item, ItemFn, ItemMod, Pat, Path, PathArguments,
    PathSegment, Token, Type, TypeReference,
};
use thiserror::Error;

pub fn resolve_path_from_item_or_args<'a, T>(
    item: &'a Item,
    args: Option<Punctuated<Path, Comma>>,
) -> syn::Result<Path>
where
    T: IdentGenericsAttrs<'a>,
{
    let struct_or_enum = T::try_from(item)?;
    let ident = struct_or_enum.ident();
    if let Some(args) = args {
        let mut args = args.into_iter();
        // Extract the single path
        let Some(path) = args.next() else {
            return Err(Error::new(item.span(), "Attribute arguments expect a path"));
        };
        if let Some(extra_arg) = args.next() {
            return Err(Error::new(
                extra_arg.span(),
                "Attribute arguments expects a single path",
            ));
        }
        let path_ident = path
            .segments
            .get(0)
            .map(|segment| &segment.ident)
            .unwrap_or_else(|| unreachable!());
        if path_ident != ident {
            let provided_path_string = path_to_string(&path, true);
            return Err(Error::new(
                path.span(),
                format!(
                    "Attribute arguments path does not match the items ident, got: {provided_path_string}, expected: {ident} (with generics if applicable)"
                ),
            ));
        }
        validate_generic_counts(struct_or_enum.generics(), &path)?;
        Ok(path)
    } else {
        Ok(ident_to_path(ident))
    }
}

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

pub enum TargetRequirePath {
    RegisterTypes,
    RegisterStateTypes,
    AddEvents,
    InitResources,
    InitStates,
    RequiredComponentAutoName,
}

pub enum TargetData {
    RegisterTypes(Path),
    RegisterStateTypes(Path),
    AddEvents(Path),
    InitResources(Path),
    InitStates(Path),
    RequiredComponentAutoName(Path),
    AddSystem {
        system: Path,
        params: AddSystemParams,
    },
}

impl TargetData {
    pub fn from_target_require_path(target_require_path: TargetRequirePath, path: Path) -> Self {
        match target_require_path {
            TargetRequirePath::RegisterTypes => Self::RegisterTypes(path),
            TargetRequirePath::RegisterStateTypes => Self::RegisterStateTypes(path),
            TargetRequirePath::AddEvents => Self::AddEvents(path),
            TargetRequirePath::InitResources => Self::InitResources(path),
            TargetRequirePath::InitStates => Self::InitStates(path),
            TargetRequirePath::RequiredComponentAutoName => Self::RequiredComponentAutoName(path),
        }
    }
}

pub struct StructOrEnumRef<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a Vec<Attribute>,
}

impl<'a> StructOrEnumRef<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a Vec<Attribute>) -> Self {
        Self {
            ident,
            generics,
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Item> for StructOrEnumRef<'a> {
    type Error = Error;

    fn try_from(item: &'a Item) -> std::result::Result<Self, Self::Error> {
        Ok(match item {
            Item::Struct(struct_item) => StructOrEnumRef::new(
                &struct_item.ident,
                &struct_item.generics,
                &struct_item.attrs,
            ),
            Item::Enum(enum_item) => {
                StructOrEnumRef::new(&enum_item.ident, &enum_item.generics, &enum_item.attrs)
            }
            _ => return Err(Error::new(item.span(), "expected struct or enum")),
        })
    }
}

pub struct FnRef<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a Vec<Attribute>,
}

impl<'a> FnRef<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a Vec<Attribute>) -> Self {
        Self {
            ident,
            generics,
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Item> for FnRef<'a> {
    type Error = Error;

    fn try_from(item: &'a Item) -> std::result::Result<Self, Self::Error> {
        Ok(match item {
            Item::Fn(fn_item) => {
                Self::new(&fn_item.sig.ident, &fn_item.sig.generics, &fn_item.attrs)
            }
            _ => return Err(Error::new(item.span(), "expected fn")),
        })
    }
}

pub trait IdentGenericsAttrs<'a>: TryFrom<&'a Item, Error = Error> {
    fn ident(&self) -> &Ident;
    fn generics(&self) -> &Generics;
    fn attributes(&self) -> &Vec<Attribute>;
}

impl<'a> IdentGenericsAttrs<'a> for StructOrEnumRef<'a> {
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

impl<'a> IdentGenericsAttrs<'a> for FnRef<'a> {
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
    for arg in &item.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let Pat::Ident(pat_ident) = &*pat_type.pat else {
                continue;
            };
            if *param_ident != pat_ident.ident {
                continue;
            }
            if !is_mutable_reference(&pat_type.ty) {
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

/// Check if the type is `&mut _`
pub fn is_mutable_reference(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Reference(TypeReference {
            mutability: Some(_),
            ..
        })
    )
}

pub fn validate_generic_counts(generics: &Generics, path: &Path) -> syn::Result<()> {
    let expected_generics_count = generics.type_params().count();
    if expected_generics_count > 0 {
        let paths_count = count_generics(path);
        if paths_count != expected_generics_count {
            return Err(Error::new(
                path.span(),
                format!(
                    "expected {expected_generics_count} generic parameters, found {paths_count}"
                ),
            ));
        }
    }
    Ok(())
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

pub fn count_generics(path: &Path) -> usize {
    // Iterate through the segments of the path
    for segment in &path.segments {
        // Check if the segment has angle-bracketed arguments
        if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
            // Count the number of arguments inside the angle brackets
            return angle_bracketed.args.len();
        }
    }
    // If no angle-bracketed arguments are found, return 0
    0
}

pub fn get_all_items_in_module_by_attribute(
    module: &ItemMod,
    attribute: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>> {
    let Some((_, items)) = &module.content else {
        return Ok(vec![]);
    };

    // Find all items with the provided [`attribute_name`] #[...] attribute
    let matched_items = items_with_attribute_macro(items, attribute)?;
    Ok(matched_items)
}

pub fn inject_module(
    module: &mut ItemMod,
    func: impl FnOnce() -> syn::Result<syn::Item>,
) -> syn::Result<()> {
    // Combine the original module with the generated code
    if let Some((_brace, items)) = module.content.as_mut() {
        // Inject code into the module block
        items.push(func()?);
    }

    Ok(())
}

pub struct ItemWithAttributeMatch {
    pub item: Item,
    pub path: Path,
    pub attributes: Attribute,
}

impl ItemWithAttributeMatch {
    pub fn into_path_string(self) -> String {
        path_to_string(&self.path, false)
    }
}

pub fn items_with_attribute_macro(
    items: &Vec<syn::Item>,
    attribute: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>> {
    let is_marker = |attr: &&Attribute| -> bool { attr.path().is_ident(attribute.ident_str()) };

    fn parse(ident: &Ident, attr: &Attribute) -> syn::Result<syn::Path> {
        let mut has_args = false;
        let _ = attr.parse_nested_meta(|_| {
            has_args = true;
            Ok(())
        });
        if has_args {
            let paths =
                attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)?;

            // Ensure exactly one path is present
            if paths.len() == 1 {
                // Extract the single path
                let path = paths.into_iter().next().unwrap_or_else(|| unreachable!());
                let path_ident = path
                    .segments
                    .get(0)
                    .map(|segment| &segment.ident)
                    .unwrap_or_else(|| unreachable!());
                if path_ident == ident {
                    Ok(path)
                } else {
                    let provided_path_string = quote!(#path).to_string().replace(" ", "");
                    Err(syn::Error::new(
                        path.span(),
                        format!(
                            "Attribute arguments path does not match the items ident, got: {provided_path_string}, expected: {ident} (with generics if applicable)"
                        ),
                    ))
                }
            } else {
                Err(syn::Error::new(
                    attr.span(),
                    "Attribute arguments expect exactly one path",
                ))
            }
        } else {
            // allow #[attribute] without args
            if let Some(segment) = attr.path().segments.last() {
                if !segment.arguments.is_empty() {
                    // this should be unreachable from testing
                    Err(syn::Error::new(
                        attr.span(),
                        "Unexpected arguments (bad proc macro logic)",
                    ))
                } else {
                    Ok(ident_to_path(ident))
                }
            } else {
                Err(syn::Error::new(
                    attr.span(),
                    "Attribute arguments expect exactly one path",
                ))
            }
        }
    }

    let mut matched_items = vec![];
    for item in items {
        let Ok(matched_item) = StructOrEnumRef::try_from(item) else {
            continue;
        };
        for attr in matched_item.attributes.iter().filter(is_marker) {
            let path = parse(matched_item.ident, attr)?;
            validate_generic_counts(matched_item.generics, &path)?;
            matched_items.push(ItemWithAttributeMatch {
                item: item.clone(),
                path,
                attributes: attr.clone(),
            });
        }
    }
    Ok(matched_items)
}

#[derive(Error, Debug)]
pub enum RustcDetectionError {
    #[error("could not query current executable: {source}")]
    CurrentExe {
        #[from]
        source: std::io::Error,
    },
    #[error("env::current_exe() path has no file‑name component: {path}")]
    NoFileName { path: std::path::PathBuf },
}

/// Checks if the current executable looks like rustc
pub fn is_rustc() -> Result<bool, RustcDetectionError> {
    use std::env;
    use std::ffi::OsStr;
    let exe = env::current_exe().map_err(RustcDetectionError::from)?;
    let Some(stem) = exe.file_stem().and_then(OsStr::to_str) else {
        return Err(RustcDetectionError::NoFileName { path: exe });
    };
    Ok(stem.eq_ignore_ascii_case("rustc"))
}

#[derive(Error, Debug)]
pub enum LocalFileError {
    /// `Span::local_file()` came back `None` – the span is virtual or remapped.
    #[error("span does not refer to a real on‑disk file")]
    VirtualSpan,

    /// Something went wrong while determining if called from rustc.
    #[error(transparent)]
    RustcDetection(#[from] RustcDetectionError),
}

pub enum LocalFile {
    File(String),
    #[cfg(feature = "lang_server_noop")]
    Noop,
    Error(LocalFileError),
}

/// Panics if called from outside a procedural macro.
///
/// TODO: remove when rust-analyzer fully implements local_file https://github.com/rust-lang/rust/blob/4e973370053a5fe87ee96d43c506623e9bd1eb9d/src/tools/rust-analyzer/crates/proc-macro-srv/src/server_impl/rust_analyzer_span.rs#L144-L147
pub fn resolve_local_file() -> LocalFile {
    match crate::flat_file::file_state::get_file_path() {
        Some(p) => LocalFile::File(p),
        None => {
            #[cfg(feature = "lang_server_noop")]
            {
                match is_rustc() {
                    Ok(false) => return LocalFile::Noop,
                    Err(e) => return LocalFile::Error(e.into()),
                    _ => {} // fall through
                }
            }
            LocalFile::Error(LocalFileError::VirtualSpan)
        }
    }
}

pub fn debug_pat(pat: &Pat) -> &'static str {
    match pat {
        Pat::Ident(_) => "Pat::Ident",
        Pat::Wild(_) => "Pat::Wild",
        Pat::Path(_) => "Pat::Path",
        Pat::Tuple(_) => "Pat::Tuple",
        Pat::Struct(_) => "Pat::Struct",
        Pat::TupleStruct(_) => "Pat::TupleStruct",
        Pat::Or(_) => "Pat::Or",
        Pat::Slice(_) => "Pat::Slice",
        Pat::Reference(_) => "Pat::Reference",
        Pat::Type(_) => "Pat::Type",
        Pat::Lit(_) => "Pat::Lit",
        Pat::Range(_) => "Pat::Range",
        Pat::Macro(_) => "Pat::Macro",
        Pat::Verbatim(_) => "Pat::Verbatim",
        Pat::Const(_) => "Pat::Const",
        Pat::Paren(_) => "Pat::Paren",
        Pat::Rest(_) => "Pat::Rest",
        _ => "<unknown>",
    }
}

pub fn debug_ty(ty: &Type) -> &'static str {
    match ty {
        Type::Array(_) => "Array",
        Type::BareFn(_) => "BareFn",
        Type::Group(_) => "Group",
        Type::ImplTrait(_) => "ImplTrait",
        Type::Infer(_) => "Infer",
        Type::Macro(_) => "Macro",
        Type::Never(_) => "Never",
        Type::Paren(_) => "Paren",
        Type::Path(_) => "Path",
        Type::Ptr(_) => "Ptr",
        Type::Reference(_) => "Reference",
        Type::Slice(_) => "Slice",
        Type::TraitObject(_) => "TraitObject",
        Type::Tuple(_) => "Tuple",
        Type::Verbatim(_) => "Verbatim",
        _ => "<unknown>",
    }
}
