use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::util::concrete_path::ConcreteTargetPathWithGenericsCollection;
use crate::__private::util::extensions::from_meta::FromMetaExt;
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use crate::__private::util::path_fmt::PathWithoutGenerics;
use syn::{Attribute, Item};

#[derive(Debug, Clone)]
pub struct ItemWithAttributeMatch<'a, A> {
    pub item: &'a Item,
    pub path: PathWithoutGenerics,
    pub matched_attribute: &'a Attribute,
    pub attributes: &'a [Attribute],
    pub args: A,
}

impl<'a, T> From<ItemWithAttributeMatch<'a, T>> for ConcreteTargetPathWithGenericsCollection
where
    T: ItemAttributeArgs,
{
    fn from(value: ItemWithAttributeMatch<T>) -> Self {
        ConcreteTargetPathWithGenericsCollection::from_args(value.path, &value.args)
    }
}

impl<'a, T> From<&ItemWithAttributeMatch<'a, T>> for ConcreteTargetPathWithGenericsCollection
where
    T: ItemAttributeArgs,
{
    fn from(value: &ItemWithAttributeMatch<T>) -> Self {
        ConcreteTargetPathWithGenericsCollection::from_args(value.path.clone(), &value.args)
    }
}

pub fn items_with_attribute_match<'items, 'meta, T, A>(
    items: &'items [Item],
) -> syn::Result<Vec<ItemWithAttributeMatch<'items, A>>>
where
    T: IdentGenericsAttrsMeta<'items> + 'meta,
    A: ItemAttributeArgs,
    'meta: 'items,
{
    items
        .iter()
        .filter_map(|item| {
            T::try_from(item)
                .ok()
                .map(|meta| (item, meta.ident(), meta.attributes()))
        })
        .flat_map(|(item, ident, attributes)| {
            attributes
                .iter()
                .filter(|a| a.meta.path().is_ident(A::attribute().ident_str()))
                .map(|attr| {
                    Ok(ItemWithAttributeMatch {
                        item,
                        path: ident.into(),
                        matched_attribute: attr,
                        attributes,
                        args: A::from_meta_ext(&attr.meta)?,
                    })
                })
        })
        .collect::<syn::Result<Vec<_>>>()
}
