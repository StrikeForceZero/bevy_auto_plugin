use crate::__private::attribute::AutoPluginAttribute;
use crate::__private::item_with_attr_match;
use crate::__private::item_with_attr_match::ItemWithAttributeMatch;
use crate::__private::util::meta::fn_meta::FnMeta;
use syn::ItemMod;

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
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddEvent => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitResource => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InsertResource => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitState => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::Name => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::RegisterStateType => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddSystem => {
            item_with_attr_match::items_with_attribute_macro::<FnMeta>(items, attribute)?
        }
        AutoPluginAttribute::AddObserver => {
            item_with_attr_match::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
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
