use crate::syntax::ast::type_list::TypeList;

pub mod none;
pub mod with_many;
pub mod with_single;

pub trait HasGenerics {
    fn generics(&self) -> &[TypeList];
}
