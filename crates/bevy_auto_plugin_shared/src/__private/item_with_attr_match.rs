use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::util::concrete_path::ConcreteTargetPathWithGenericsCollection;
use crate::__private::util::extensions::from_meta::FromMetaExt;
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use crate::__private::util::path_fmt::PathWithoutGenerics;
use syn::{Attribute, Item};

#[derive(Debug, Clone)]
pub struct ItemWithAttributeMatch<A> {
    pub item: Item,
    pub path: PathWithoutGenerics,
    pub matched_attribute: Attribute,
    pub attributes: Vec<Attribute>,
    pub args: A,
}

impl<T> From<ItemWithAttributeMatch<T>> for ConcreteTargetPathWithGenericsCollection
where
    T: ItemAttributeArgs,
{
    fn from(value: ItemWithAttributeMatch<T>) -> Self {
        ConcreteTargetPathWithGenericsCollection::from_args(value.path, &value.args)
    }
}

impl<T> From<&ItemWithAttributeMatch<T>> for ConcreteTargetPathWithGenericsCollection
where
    T: ItemAttributeArgs,
{
    fn from(value: &ItemWithAttributeMatch<T>) -> Self {
        ConcreteTargetPathWithGenericsCollection::from_args(value.path.clone(), &value.args)
    }
}

pub fn items_with_attribute_match<'a, T, A>(
    items: &'a [Item],
) -> syn::Result<Vec<ItemWithAttributeMatch<A>>>
where
    T: IdentGenericsAttrsMeta<'a>,
    A: ItemAttributeArgs,
{
    let mut matched_items = vec![];
    for item in items {
        let Ok(matched_item) = T::try_from(item) else {
            continue;
        };
        for attr in matched_item
            .attributes()
            .iter()
            .filter(|a| a.meta.path().is_ident(A::attribute().ident_str()))
        {
            matched_items.push(ItemWithAttributeMatch {
                item: item.clone(),
                path: matched_item.ident().into(),
                matched_attribute: attr.clone(),
                attributes: matched_item.attributes().to_vec(),
                args: A::from_meta_ext(&attr.meta)?,
            })
        }
    }
    Ok(matched_items)
}
