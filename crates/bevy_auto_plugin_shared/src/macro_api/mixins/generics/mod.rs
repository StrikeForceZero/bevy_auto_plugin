use crate::syntax::ast::type_list::TypeList;

pub mod with;
pub mod without;

pub trait HasGenerics {
    fn generics(&self) -> &[TypeList];
}
