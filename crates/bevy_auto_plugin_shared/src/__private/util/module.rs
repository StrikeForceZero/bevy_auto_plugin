use crate::__private::attribute_args::ItemAttributeArgs;
use crate::__private::item_with_attr_match::ItemWithAttributeMatch;
use syn::ItemMod;

pub fn get_all_items_in_module_by_attribute<A>(
    module: &ItemMod,
) -> syn::Result<Vec<ItemWithAttributeMatch<'_, A>>>
where
    A: ItemAttributeArgs,
{
    let Some((_, items)) = &module.content else {
        return Ok(vec![]);
    };

    // Find all items with the provided [`attribute_name`] #[...] attribute
    let matched_items = A::match_items(items)?;
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
