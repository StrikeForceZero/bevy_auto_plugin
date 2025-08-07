use crate::attribute::AutoPluginAttribute;
use crate::item_with_attr_match::ItemWithAttributeMatch;
use crate::util::item::IdentGenericsAttrs;
use crate::util::{concrete_path, path};
use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

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

fn struct_or_enum_item_with_attribute_macro(
    item: &Item,
    struct_or_enum_ref: &StructOrEnumRef,
    attr: &Attribute,
    attrs: &[Attribute],
    target: AutoPluginAttribute,
) -> syn::Result<impl Iterator<Item = ItemWithAttributeMatch>> {
    let path = path::ident_to_path(struct_or_enum_ref.ident());
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
