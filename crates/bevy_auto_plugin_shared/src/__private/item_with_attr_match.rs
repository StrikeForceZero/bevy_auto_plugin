use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::util::concrete_path::ConcreteTargetPathWithGenericsCollection;
use crate::__private::util::extensions::from_meta::FromMetaExt;
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
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

impl<A> ItemWithAttributeMatch<A> {
    pub fn path_owned(self) -> PathWithoutGenerics {
        self.path
    }
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

pub fn items_with_attribute_macro<'a, T, A>(
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

fn struct_or_enum_item_with_attribute_macro<A>(
    item: Item,
    struct_or_enum_ref: &StructOrEnumMeta,
    attr: Attribute,
    attrs: &[Attribute],
) -> syn::Result<ItemWithAttributeMatch<A>>
where
    A: ItemAttributeArgs,
{
    let path = PathWithoutGenerics::from(struct_or_enum_ref.ident());
    Ok(ItemWithAttributeMatch {
        item,
        path,
        args: A::from_meta_ext(&attr.meta)?,
        matched_attribute: attr,
        attributes: attrs.to_vec(),
    })
}

fn do_with_struct_or_enum_items_with_attribute_macro<F, R, A>(
    items: &[Item],
    cb: F,
) -> syn::Result<Vec<R>>
where
    F: Fn(&Item, &StructOrEnumMeta, &Attribute, &[Attribute], A) -> syn::Result<Vec<R>>,
    A: ItemAttributeArgs,
{
    let is_marker =
        |attr: &&Attribute| -> bool { attr.path().is_ident(A::attribute().ident_str()) };

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
                A::from_meta_ext(&attr.meta)?,
            )?;
            matched_items.extend(matched_item);
        }
    }
    Ok(matched_items)
}

pub fn struct_or_enum_items_with_attribute_macro<A>(
    items: &[Item],
) -> syn::Result<Vec<ItemWithAttributeMatch<A>>>
where
    A: ItemAttributeArgs,
{
    do_with_struct_or_enum_items_with_attribute_macro::<_, _, A>(
        items,
        |item, struct_or_enum_ref, attr, attrs, args| {
            let concrete_paths = ConcreteTargetPathWithGenericsCollection::from_args(
                A::resolve_item_ident(item)?.into(),
                &args,
            );
            let matched = struct_or_enum_item_with_attribute_macro::<A>(
                item.clone(),
                struct_or_enum_ref,
                attr.clone(),
                attrs,
            )?;
            Ok(concrete_paths
                .into_iter()
                .map(|_| matched.clone())
                .collect::<Vec<_>>())
        },
    )
}
