use crate::__private::attribute::AutoPluginAttribute;
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use crate::__private::util::{concrete_path, path_fmt};
use syn::{Attribute, Item, Path};

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
        path_fmt::path_to_string(&self.path, false)
    }
}

pub fn items_with_attribute_macro<'a, T>(
    items: &'a Vec<Item>,
    target: AutoPluginAttribute,
) -> syn::Result<Vec<ItemWithAttributeMatch>>
where
    T: IdentGenericsAttrsMeta<'a>,
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
                path: path_fmt::ident_to_path(matched_item.ident()),
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
    struct_or_enum_ref: &StructOrEnumMeta,
    attr: &Attribute,
    attrs: &[Attribute],
    target: AutoPluginAttribute,
) -> syn::Result<impl Iterator<Item = ItemWithAttributeMatch>> {
    let path = path_fmt::ident_to_path(struct_or_enum_ref.ident());
    let mut has_args = false;
    let _ = attr.parse_nested_meta(|_| {
        has_args = true;
        Ok(())
    });

    let paths = if has_args {
        concrete_path::resolve_user_provided_generic_paths(
            target,
            attr,
            struct_or_enum_ref,
            &path,
            #[cfg(feature = "legacy_path_param")]
            item,
        )?
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
    items: &Vec<Item>,
    target: AutoPluginAttribute,
    cb: F,
) -> syn::Result<Vec<ItemWithAttributeMatch>>
where
    F: Fn(
        &Item,
        &StructOrEnumMeta,
        &Attribute,
        &[Attribute],
        AutoPluginAttribute,
    ) -> syn::Result<Vec<ItemWithAttributeMatch>>,
{
    let is_marker = |attr: &&Attribute| -> bool { attr.path().is_ident(target.ident_str()) };

    let mut matched_items = vec![];
    for item in items {
        let Ok(struct_or_enum_ref) = StructOrEnumMeta::try_from(item) else {
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
    items: &Vec<Item>,
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
