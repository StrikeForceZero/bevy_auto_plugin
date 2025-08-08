use crate::attribute::AutoPluginAttribute;
use crate::attribute_args::{InsertResourceArgs, StructOrEnumAttributeArgs};
use crate::type_list::TypeList;
use crate::util::extensions::path::PathExt;
use crate::util::generics::{CountGenerics, HasGenericCollection};
use crate::util::meta::IdentGenericsAttrsMeta;
use crate::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::util::path_fmt;
use darling::FromMeta;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Error, Generics, Item, Path, PathArguments, parse_quote, parse2};

pub fn resolve_paths_from_item_or_args<'a, T>(
    item: &'a Item,
    args: impl HasGenericCollection + CountGenerics,
) -> syn::Result<impl Iterator<Item = Path>>
where
    T: IdentGenericsAttrsMeta<'a>,
{
    let item_meta = T::try_from(item)?;
    let ident = item_meta.ident();
    let paths = if args.count_generics() > 0 {
        let generics = args.generics()?;
        generics
            .into_iter()
            .map(|generics| {
                let path_tokens = quote! { #ident::<#generics> };
                let path = Path::from_string(&path_tokens.to_string())?;
                validate_generic_counts(item_meta.generics(), &path)?;
                Ok(path)
            })
            .collect::<syn::Result<Vec<_>>>()?
    } else {
        vec![path_fmt::ident_to_path(ident)]
    };
    Ok(paths.into_iter())
}

/// Build the concrete paths for an item based on user-provided generic values.
pub fn resolve_user_provided_generic_paths(
    target: AutoPluginAttribute,
    attr: &Attribute,
    struct_or_enum_ref: &StructOrEnumMeta,
    base_path: &Path,
    #[cfg(feature = "legacy_path_param")] item: &Item,
) -> syn::Result<Vec<Path>> {
    // A small utility for turning generic lists into `Path`s.
    let build_paths = |lists: Vec<TypeList>| -> Vec<Path> {
        if lists.is_empty() {
            vec![base_path.clone()]
        } else {
            lists
                .into_iter()
                .map(|generics| parse_quote!(#base_path::<#generics>))
                .collect()
        }
    };

    match target {
        // --------------------------------- InsertResource ---------------------------------
        // InsertResource uses its own darling args type and never allowed the legacy path
        // syntax, so the parsing is straightforward.
        AutoPluginAttribute::InsertResource => {
            let insert_args = InsertResourceArgs::from_meta(&attr.meta).map_err(Error::from)?;
            validate_generic_counts(struct_or_enum_ref.generics, &insert_args)?;
            let lists = insert_args
                .generics
                .clone()
                .map(|g| vec![g])
                .unwrap_or_default();
            Ok(build_paths(lists))
        }

        // ------------------------------- All other targets -------------------------------
        _ => {
            // Parse modern form first …
            let modern_args_res =
                StructOrEnumAttributeArgs::from_meta(&attr.meta).map_err(Error::from);

            // … then, if enabled, fall back to the legacy `Foo<T>` path syntax.
            #[cfg(feature = "legacy_path_param")]
            let modern_args_res = match modern_args_res {
                Ok(v) => Ok(v),
                Err(err) => StructOrEnumMeta::try_from(item)
                    .and_then(|se_ref| {
                        legacy_generics_from_path(&se_ref, attr.meta.require_list()?.tokens.clone())
                    })
                    .map(StructOrEnumAttributeArgs::from)
                    .map_err(|legacy_err| {
                        Error::new(err.span(), format!("\nnew: {err}\nlegacy: {legacy_err}"))
                    }),
            };

            let modern_args = modern_args_res?;
            validate_generic_counts(struct_or_enum_ref.generics, &modern_args)?;
            Ok(build_paths(modern_args.generics.clone()))
        }
    }
}

pub fn validate_generic_counts<T>(generics: &Generics, cg: &T) -> syn::Result<()>
where
    T: CountGenerics,
{
    let expected_generics_count = generics.type_params().count();
    if expected_generics_count > 0 {
        let count = cg.count_generics();
        if count != expected_generics_count {
            return Err(Error::new(
                cg.get_span(),
                format!("expected {expected_generics_count} generic parameters, found {count}"),
            ));
        }
    }
    Ok(())
}

pub fn generics_from_path(path: &Path) -> syn::Result<TypeList> {
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
    struct_or_enum_ref: &StructOrEnumMeta,
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
    PathExt::generics(&path)
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
        let struct_or_enum_ref = StructOrEnumMeta::try_from(&item)?;
        let generics = legacy_generics_from_path(&struct_or_enum_ref, attribute)?;
        assert_eq!("T , U", generics.to_token_stream().to_string().trim());
        Ok(())
    }
}
