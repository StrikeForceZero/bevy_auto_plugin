use crate::attribute_args::{InsertResourceArgs, StructOrEnumAttributeArgs};
use crate::type_list::TypeList;
use crate::util::extensions::path::PathExt;
use crate::util::struct_or_enum_ref::StructOrEnumRef;
use proc_macro2::Span;
use syn::Path;

pub trait CountGenerics {
    fn get_span(&self) -> Span;
    fn count(&self) -> usize;
}

impl CountGenerics for Path {
    fn get_span(&self) -> Span {
        syn::spanned::Spanned::span(&self)
    }

    fn count(&self) -> usize {
        self.generic_count().unwrap_or(0)
    }
}

impl CountGenerics for StructOrEnumAttributeArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics
            .first()
            .map(|g| g.span())
            .unwrap_or(Span::call_site())
    }

    fn count(&self) -> usize {
        let iter = self.generics.iter().map(|g| g.len()).collect::<Vec<_>>();
        let &max = iter.iter().max().unwrap_or(&0);
        let &min = iter.iter().min().unwrap_or(&0);
        // TODO: return result
        assert_eq!(
            max, min,
            "inconsistent number of generics specified min: {min}, max: {max}"
        );
        max
    }
}

impl CountGenerics for InsertResourceArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count(&self) -> usize {
        let Some(generics) = &self.generics else {
            return 0;
        };
        generics.len()
    }
}

impl CountGenerics for TypeList {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.span()
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl CountGenerics for StructOrEnumRef<'_> {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count(&self) -> usize {
        self.generics.params.len()
    }
}
