use crate::attribute::AutoPluginAttribute;
use crate::item_with_attr_match;
use crate::item_with_attr_match::ItemWithAttributeMatch;
use crate::util::item_fn::FnRef;
use crate::util::struct_or_enum_ref;
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
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddEvent => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitResource => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InsertResource => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::InitState => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::Name => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::RegisterStateType => {
            struct_or_enum_ref::struct_or_enum_items_with_attribute_macro(items, attribute)?
        }
        AutoPluginAttribute::AddSystem => {
            item_with_attr_match::items_with_attribute_macro::<FnRef>(items, attribute)?
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
