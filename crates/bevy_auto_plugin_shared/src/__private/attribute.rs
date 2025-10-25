use crate::codegen::ExpandAttrs;
use crate::syntax::validated::non_empty_path::NonEmptyPath;

pub trait RewriteAttribute {
    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs;
}
