use crate::attribute::AutoPluginAttribute;
use crate::util::item::IdentGenericsAttrs;
use crate::util::path;
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
        path::path_to_string(&self.path, false)
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
                path: path::ident_to_path(matched_item.ident()),
                matched_attribute: attr.clone(),
                attributes: matched_item.attributes().to_vec(),
                target,
            })
        }
    }
    Ok(matched_items)
}
