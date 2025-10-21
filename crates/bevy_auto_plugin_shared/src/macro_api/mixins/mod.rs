use darling::ast::NestedMeta;

pub mod generics;
pub mod nothing;
pub mod with_plugin;

pub trait HasKeys {
    fn keys() -> &'static [&'static str];
}

pub trait Mixin: Sized {
    /// Keys this mixin recognizes (top-level names).
    fn keys() -> &'static [&'static str];

    /// Parse from just the metas that were routed to this mixin.
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self>;
}

impl<T> Mixin for T
where
    T: darling::FromMeta + HasKeys,
{
    fn keys() -> &'static [&'static str] {
        <T as HasKeys>::keys()
    }
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        <T as darling::FromMeta>::from_list(items)
    }
}

pub mod prelude {
    use super::*;
    pub use generics::*;
    pub use nothing::*;
    pub use with_plugin::*;
}
