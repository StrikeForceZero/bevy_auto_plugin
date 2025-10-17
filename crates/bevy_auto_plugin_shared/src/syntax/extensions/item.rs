use syn::{Attribute, Item};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TakeAndPutAttrsError {
    #[error("Item does not have attrs")]
    ItemDoesNotHaveAttrs,
}

pub trait ItemAttrsExt {
    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>, TakeAndPutAttrsError>;
    fn take_attrs(&mut self) -> Result<Vec<Attribute>, TakeAndPutAttrsError>;
    fn put_attrs(&mut self, attrs: Vec<Attribute>) -> Result<Vec<Attribute>, TakeAndPutAttrsError>;
}

impl ItemAttrsExt for Item {
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
