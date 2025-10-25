use crate::syntax::ast::type_list::TypeList;

pub mod none;
pub mod with_many;
pub mod with_single;

pub trait HasGenerics {
    fn generics(&self) -> &[TypeList];
}

pub mod prelude {
    pub use super::HasGenerics;
    use super::*;
    pub use none::*;
    pub use with_many::*;
    pub use with_single::*;
}
