#![allow(dead_code)]

use syn::{Attribute, Item};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TakeAndPutAttrsError {
    #[error("Item does not have attrs")]
    ItemDoesNotHaveAttrs,
}

pub trait ItemAttrsExt {
    fn clone_attrs(&self) -> Option<Vec<Attribute>>;
    fn attrs(&self) -> Option<&[Attribute]>;
    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>, TakeAndPutAttrsError>;
    fn take_attrs(&mut self) -> Result<Vec<Attribute>, TakeAndPutAttrsError>;
    fn put_attrs(&mut self, attrs: Vec<Attribute>) -> Result<Vec<Attribute>, TakeAndPutAttrsError>;
}

impl ItemAttrsExt for Item {
    fn clone_attrs(&self) -> Option<Vec<Attribute>> {
        Some(match self {
            Item::Const(i) => i.attrs.clone(),
            Item::Enum(i) => i.attrs.clone(),
            Item::ExternCrate(i) => i.attrs.clone(),
            Item::Fn(i) => i.attrs.clone(),
            Item::ForeignMod(i) => i.attrs.clone(),
            Item::Impl(i) => i.attrs.clone(),
            Item::Macro(i) => i.attrs.clone(),
            Item::Mod(i) => i.attrs.clone(),
            Item::Static(i) => i.attrs.clone(),
            Item::Struct(i) => i.attrs.clone(),
            Item::Trait(i) => i.attrs.clone(),
            Item::TraitAlias(i) => i.attrs.clone(),
            Item::Type(i) => i.attrs.clone(),
            Item::Union(i) => i.attrs.clone(),
            Item::Use(i) => i.attrs.clone(),
            Item::Verbatim(_) => return None,
            _ => return None,
        })
    }
    fn attrs(&self) -> Option<&[Attribute]> {
        Some(match self {
            Item::Const(i) => &i.attrs,
            Item::Enum(i) => &i.attrs,
            Item::ExternCrate(i) => &i.attrs,
            Item::Fn(i) => &i.attrs,
            Item::ForeignMod(i) => &i.attrs,
            Item::Impl(i) => &i.attrs,
            Item::Macro(i) => &i.attrs,
            Item::Mod(i) => &i.attrs,
            Item::Static(i) => &i.attrs,
            Item::Struct(i) => &i.attrs,
            Item::Trait(i) => &i.attrs,
            Item::TraitAlias(i) => &i.attrs,
            Item::Type(i) => &i.attrs,
            Item::Union(i) => &i.attrs,
            Item::Use(i) => &i.attrs,
            Item::Verbatim(_) => return None,
            _ => return None,
        })
    }
    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>, TakeAndPutAttrsError> {
        Ok(match self {
            Item::Const(i) => &mut i.attrs,
            Item::Enum(i) => &mut i.attrs,
            Item::ExternCrate(i) => &mut i.attrs,
            Item::Fn(i) => &mut i.attrs,
            Item::ForeignMod(i) => &mut i.attrs,
            Item::Impl(i) => &mut i.attrs,
            Item::Macro(i) => &mut i.attrs,
            Item::Mod(i) => &mut i.attrs,
            Item::Static(i) => &mut i.attrs,
            Item::Struct(i) => &mut i.attrs,
            Item::Trait(i) => &mut i.attrs,
            Item::TraitAlias(i) => &mut i.attrs,
            Item::Type(i) => &mut i.attrs,
            Item::Union(i) => &mut i.attrs,
            Item::Use(i) => &mut i.attrs,
            Item::Verbatim(_) => return Err(TakeAndPutAttrsError::ItemDoesNotHaveAttrs),
            _ => return Err(TakeAndPutAttrsError::ItemDoesNotHaveAttrs),
        })
    }
    fn take_attrs(&mut self) -> Result<Vec<Attribute>, TakeAndPutAttrsError> {
        Ok(std::mem::take(self.attrs_mut()?))
    }
    fn put_attrs(&mut self, attrs: Vec<Attribute>) -> Result<Vec<Attribute>, TakeAndPutAttrsError> {
        Ok(std::mem::replace(self.attrs_mut()?, attrs))
    }
}
