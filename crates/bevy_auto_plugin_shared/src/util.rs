use crate::attribute::AutoPluginAttribute;
use crate::attribute_args::{
    AddSystemArgs, InsertResourceArgs, InsertResourceArgsWithPath, StructOrEnumAttributeArgs,
};
use crate::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Error, FnArg, Generics, Item, ItemFn, ItemMod, Pat, Path, PathArguments,
    PathSegment, Type, TypeReference, parse_quote, parse2,
};
use thiserror::Error;

pub fn resolve_paths_from_item_or_args<'a, T>(
    item: &'a Item,
    args: StructOrEnumAttributeArgs,
) -> syn::Result<impl Iterator<Item = Path>>
where
    T: IdentGenericsAttrs<'a>,
{
    let struct_or_enum = T::try_from(item)?;
    let ident = struct_or_enum.ident();
    let paths = if args.has_generics() {
        let generics = &args.generics;
        generics
            .iter()
            .map(|generics| {
                let path_tokens = quote! { #ident::<#generics> };
                let path = Path::from_string(&path_tokens.to_string())?;
                validate_generic_counts(struct_or_enum.generics(), &path)?;
                Ok(path)
            })
            .collect::<syn::Result<Vec<_>>>()?
    } else {
        vec![ident_to_path(ident)]
    };
    Ok(paths.into_iter())
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

#[derive(Debug, Clone, Copy)]
pub enum TargetRequirePath {
    RegisterTypes,
    RegisterStateTypes,
    AddEvents,
    InitResources,
    InitStates,
    RequiredComponentAutoName,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TargetData {
    RegisterTypes(Path),
    RegisterStateTypes(Path),
    AddEvents(Path),
    InitResources(Path),
    InitStates(Path),
    RequiredComponentAutoName(Path),
    InsertResource(InsertResourceArgsWithPath),
    AddSystem { system: Path, params: AddSystemArgs },
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
        use syn::spanned::Spanned;
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
        use syn::spanned::Spanned;
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
    use syn::spanned::Spanned;
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

pub trait CountGenerics {
    fn get_span(&self) -> Span;
    fn count(&self) -> usize;
}

impl CountGenerics for Path {
    fn get_span(&self) -> Span {
        syn::spanned::Spanned::span(&self)
    }

    fn count(&self) -> usize {
        self.generic_count().unwrap_or(0)
    }
}

impl CountGenerics for StructOrEnumRef<'_> {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count(&self) -> usize {
        self.generics.params.len()
    }
}

impl CountGenerics for StructOrEnumAttributeArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics
            .first()
            .map(|g| g.span())
            .unwrap_or(Span::call_site())
    }

    fn count(&self) -> usize {
        let iter = self.generics.iter().map(|g| g.len()).collect::<Vec<_>>();
        let &max = iter.iter().max().unwrap_or(&0);
        let &min = iter.iter().min().unwrap_or(&0);
        // TODO: return result
        assert_eq!(
            max, min,
            "inconsistent number of generics specified min: {min}, max: {max}"
        );
        max
    }
}

impl CountGenerics for InsertResourceArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count(&self) -> usize {
        let Some(generics) = &self.generics else {
            return 0;
        };
        generics.len()
    }
}

impl CountGenerics for TypeList {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.span()
    }

    fn count(&self) -> usize {
        self.len()
    }
}

pub fn validate_generic_counts<T>(generics: &Generics, cg: &T) -> syn::Result<()>
where
    T: CountGenerics,
{
    let expected_generics_count = generics.type_params().count();
    if expected_generics_count > 0 {
        let count = cg.count();
        if count != expected_generics_count {
            return Err(Error::new(
                cg.get_span(),
                format!("expected {expected_generics_count} generic parameters, found {count}"),
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

pub fn get_all_items_in_module_by_attribute(
    module: &ItemMod,
    attribute: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>> {
    let Some((_, items)) = &module.content else {
        return Ok(vec![]);
    };

    // Find all items with the provided [`attribute_name`] #[...] attribute
    let matched_items = match attribute {
        AutoPluginAttribute::RegisterType => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddEvent => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitResource => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InsertResource => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitState => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::Name => struct_or_enum_items_with_attribute_macro(items, attribute)?,
        AutoPluginAttribute::RegisterStateType => {
            struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddSystem => items_with_attribute_macro::<FnRef>(items, attribute)?,
    };
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

#[derive(Debug)]
pub struct ItemWithAttributeMatch {
    pub item: Item,
    pub path: Path,
    pub target: AutoPluginAttribute,
    pub matched_attribute: Attribute,
    pub attributes: Vec<Attribute>,
}

impl ItemWithAttributeMatch {
    pub fn path_owned(self) -> Path {
        self.path
    }
    pub fn into_path_string(self) -> String {
        path_to_string(&self.path, false)
    }
}

pub fn items_with_attribute_macro<'a, T>(
    items: &'a Vec<Item>,
    target: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>>
where
    T: IdentGenericsAttrs<'a>,
{
    let mut matched_items = vec![];
    for item in items {
        let Ok(matched_item) = T::try_from(item) else {
            continue;
        };
        for attr in matched_item
            .attributes()
            .iter()
            .filter(|a| a.meta.path().is_ident(target.ident_str()))
        {
            matched_items.push(ItemWithAttributeMatch {
                item: item.clone(),
                path: ident_to_path(matched_item.ident()),
                matched_attribute: attr.clone(),
                attributes: matched_item.attributes().to_vec(),
                target,
            })
        }
    }
    Ok(matched_items)
}

fn struct_or_enum_item_with_attribute_macro(
    item: &Item,
    struct_or_enum_ref: &StructOrEnumRef,
    attr: &Attribute,
    attrs: &[Attribute],
    target: AutoPluginAttribute,
) -> syn::Result<impl Iterator<Item = ItemWithAttributeMatch>> {
    let path = ident_to_path(struct_or_enum_ref.ident());
    let mut has_args = false;
    let _ = attr.parse_nested_meta(|_| {
        has_args = true;
        Ok(())
    });

    let paths = if has_args {
        #[derive(Debug)]
        enum UserProvidedGenericValues {
            InsertResource(InsertResourceArgs),
            StructOrEnum(StructOrEnumAttributeArgs),
        }

        impl UserProvidedGenericValues {
            fn generics(&self) -> Vec<TypeList> {
                match self {
                    UserProvidedGenericValues::InsertResource(item) => {
                        item.generics.clone().map(|g| vec![g]).unwrap_or_default()
                    }
                    UserProvidedGenericValues::StructOrEnum(item) => item.generics.clone(),
                }
            }
        }

        impl CountGenerics for UserProvidedGenericValues {
            fn get_span(&self) -> Span {
                match self {
                    Self::InsertResource(item) => CountGenerics::get_span(item),
                    Self::StructOrEnum(item) => CountGenerics::get_span(item),
                }
            }

            fn count(&self) -> usize {
                match self {
                    Self::InsertResource(item) => CountGenerics::count(item),
                    Self::StructOrEnum(item) => CountGenerics::count(item),
                }
            }
        }

        let user_provided_generic_values = match target {
            // insert resource never had legacy path param usage
            AutoPluginAttribute::InsertResource => {
                let user_provided_generic_values =
                    InsertResourceArgs::from_meta(&attr.meta).map_err(Error::from);
                let user_provided_generic_values = user_provided_generic_values?;
                UserProvidedGenericValues::InsertResource(user_provided_generic_values)
            }
            // check if path param is legacy or standard
            _ => {
                let user_provided_generic_values =
                    StructOrEnumAttributeArgs::from_meta(&attr.meta).map_err(Error::from);

                #[cfg(feature = "legacy_path_param")]
                let user_provided_generic_values = match user_provided_generic_values {
                    Ok(v) => Ok(v),
                    Err(err) => StructOrEnumRef::try_from(item)
                        .and_then(|se_ref| {
                            legacy_generics_from_path(
                                &se_ref,
                                attr.meta.require_list()?.tokens.clone(),
                            )
                        })
                        .map(StructOrEnumAttributeArgs::from)
                        .map_err(|legacy_err| {
                            Error::new(err.span(), format!("\nnew: {err}\nlegacy: {legacy_err}"))
                        }),
                };
                let user_provided_generic_values = user_provided_generic_values?;
                UserProvidedGenericValues::StructOrEnum(user_provided_generic_values)
            }
        };
        validate_generic_counts(struct_or_enum_ref.generics, &user_provided_generic_values)?;
        let expand_generics = |item: UserProvidedGenericValues| {
            item.generics()
                .into_iter()
                .map(|generics| parse_quote!(#path::<#generics>))
                .collect::<Vec<Path>>()
        };
        // TODO: convoluted...
        // ensure we return at least one path for insert resource since we always need to have args
        match &user_provided_generic_values {
            UserProvidedGenericValues::InsertResource(item) => {
                if CountGenerics::count(item) == 0 {
                    vec![path]
                } else {
                    expand_generics(user_provided_generic_values)
                }
            }
            UserProvidedGenericValues::StructOrEnum(_) => {
                expand_generics(user_provided_generic_values)
            }
        }
    } else {
        vec![path]
    };
    Ok(paths.into_iter().map(move |path| ItemWithAttributeMatch {
        item: item.clone(),
        path,
        target,
        matched_attribute: attr.clone(),
        attributes: attrs.to_vec(),
    }))
}

fn do_with_struct_or_enum_items_with_attribute_macro<F>(
    items: &Vec<syn::Item>,
    target: AutoPluginAttribute,
    cb: F,
) -> syn::Result<Vec<ItemWithAttributeMatch>>
where
    F: Fn(
        &Item,
        &StructOrEnumRef,
        &Attribute,
        &[Attribute],
        AutoPluginAttribute,
    ) -> syn::Result<Vec<ItemWithAttributeMatch>>,
{
    let is_marker = |attr: &&Attribute| -> bool { attr.path().is_ident(target.ident_str()) };

    let mut matched_items = vec![];
    for item in items {
        let Ok(struct_or_enum_ref) = StructOrEnumRef::try_from(item) else {
            continue;
        };
        for attr in struct_or_enum_ref.attributes.iter().filter(is_marker) {
            let matched_item = cb(
                item,
                &struct_or_enum_ref,
                attr,
                struct_or_enum_ref.attributes,
                target,
            )?;
            matched_items.extend(matched_item);
        }
    }
    Ok(matched_items)
}

pub fn struct_or_enum_items_with_attribute_macro(
    items: &Vec<syn::Item>,
    target: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>> {
    do_with_struct_or_enum_items_with_attribute_macro(
        items,
        target,
        |item, struct_or_enum_ref, attr, attrs, target| {
            // TODO: this got ugly
            Ok(struct_or_enum_item_with_attribute_macro(
                item,
                struct_or_enum_ref,
                attr,
                attrs,
                target,
            )?
            .collect())
        },
    )
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

#[cfg(feature = "mode_flat_file")]
#[derive(Error, Debug)]
pub enum LocalFileError {
    /// `Span::local_file()` came back `None` – the span is virtual or remapped.
    #[error("span does not refer to a real on‑disk file")]
    VirtualSpan,

    /// Something went wrong while determining if called from rustc.
    #[error(transparent)]
    RustcDetection(#[from] RustcDetectionError),
}

#[cfg(feature = "mode_flat_file")]
pub enum LocalFile {
    File(String),
    #[cfg(feature = "lang_server_noop")]
    Noop,
    Error(LocalFileError),
}

#[cfg(feature = "mode_flat_file")]
/// Panics if called from outside a procedural macro.
///
/// TODO: remove when rust-analyzer fully implements local_file https://github.com/rust-lang/rust/blob/4e973370053a5fe87ee96d43c506623e9bd1eb9d/src/tools/rust-analyzer/crates/proc-macro-srv/src/server_impl/rust_analyzer_span.rs#L144-L147
pub fn resolve_local_file() -> LocalFile {
    match crate::modes::flat_file::file_state::get_file_path() {
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

pub trait PathExt {
    fn has_generics(&self) -> syn::Result<bool>;
    fn generics(&self) -> syn::Result<TypeList>;
    fn generic_count(&self) -> syn::Result<usize> {
        Ok(self.generics()?.len())
    }
}

impl PathExt for Path {
    fn has_generics(&self) -> syn::Result<bool> {
        Ok(!self.generics()?.is_empty())
    }

    fn generics(&self) -> syn::Result<TypeList> {
        generics_from_path(self)
    }
}

fn generics_from_path(path: &Path) -> syn::Result<TypeList> {
    let mut generics = TypeList::new();
    for segment in &path.segments {
        if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
            let type_list = parse2::<TypeList>(angle_bracketed.args.to_token_stream())?;
            generics.0.extend(type_list.0);
        }
    }
    Ok(generics)
}

pub fn legacy_generics_from_path(
    struct_or_enum_ref: &StructOrEnumRef,
    attr: MacroStream,
) -> syn::Result<TypeList> {
    let path = parse2::<Path>(attr)?;
    let path_last_segment = path.segments.last();
    let path_maybe_ident = path_last_segment.map(|s| &s.ident);
    if Some(struct_or_enum_ref.ident) != path_maybe_ident {
        use syn::spanned::Spanned;
        return Err(Error::new(
            path.span(),
            format!(
                "path ident {} does not match struct or enum ident {:?}",
                struct_or_enum_ref.ident, path_maybe_ident
            ),
        ));
    }
    validate_generic_counts(struct_or_enum_ref.generics, &path)?;
    path.generics()
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

#[macro_export]
macro_rules! ok_or_return_compiler_error {
    // Case 1: Only expression
    ($expr:expr) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), "failed to parse", $expr)
    };

    // Case 2: Span, Expression
    ($span:expr, $expr:expr) => {
        ok_or_return_compiler_error!(@internal $span, "failed to parse", $expr)
    };

    // Case 3: Expression, message
    ($expr:expr, $message:literal) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), $message, $expr)
    };

    // Case 4: Span, message, Expression
    ($span:expr, $message:literal, $expr:expr) => {
        ok_or_return_compiler_error!(@internal $span, $message, $expr)
    };

    // Internal handler (common logic)
    (@internal $span:expr, $message:expr, $expr:expr) => {{
        let span = $span;
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(span, format!("{}: {}", message, e))
                    .to_compile_error()
                    .into();
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    #[test]
    fn test_generics_from_path() -> Result<(), syn::Error> {
        let item = parse2::<Path>(quote! {
            foo::bar::<u32, i32>
        })?;
        let generics = generics_from_path(&item).expect("no generics");
        let generics = quote! { #generics };
        assert_eq!("u32 , i32", generics.to_string());
        Ok(())
    }

    #[test]
    fn test_legacy_generics_from_path() -> Result<(), syn::Error> {
        let item = parse_quote! {
            #[auto_register_types(Foo<T, U>)]
            struct Foo<T, U>(T, U);
        };
        let attribute = quote! {
            Foo<T, U>
        };
        let struct_or_enum_ref = StructOrEnumRef::try_from(&item)?;
        let generics = legacy_generics_from_path(&struct_or_enum_ref, attribute)?;
        assert_eq!("T , U", generics.to_token_stream().to_string().trim());
        Ok(())
    }
}

pub fn require_fn(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only functions use this attribute macro",
        )),
    }
}

pub fn require_struct_or_enum(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only struct and enum can use this attribute macro",
        )),
    }
}
