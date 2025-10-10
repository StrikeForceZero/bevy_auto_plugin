use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::generics::GenericsCollection;

pub trait GenericsArgs {
    // TODO: see impl ToTokens for Generics
    const TURBOFISH: bool = false;
    fn type_lists(&self) -> &[TypeList];
    fn generics(&self) -> GenericsCollection {
        GenericsCollection(self.type_lists().to_vec())
    }
}
