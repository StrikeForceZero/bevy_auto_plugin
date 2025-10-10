use syn::{Item, Pat, Type};

#[allow(dead_code)]
pub fn pat_kind(pat: &Pat) -> &'static str {
    match pat {
        Pat::Ident(_) => "Pat::Ident",
        Pat::Wild(_) => "Pat::Wild",
        Pat::Path(_) => "Pat::Path",
        Pat::Tuple(_) => "Pat::Tuple",
        Pat::Struct(_) => "Pat::Struct",
        Pat::TupleStruct(_) => "Pat::TupleStruct",
        Pat::Or(_) => "Pat::Or",
        Pat::Slice(_) => "Pat::Slice",
        Pat::Reference(_) => "Pat::Reference",
        Pat::Type(_) => "Pat::Type",
        Pat::Lit(_) => "Pat::Lit",
        Pat::Range(_) => "Pat::Range",
        Pat::Macro(_) => "Pat::Macro",
        Pat::Verbatim(_) => "Pat::Verbatim",
        Pat::Const(_) => "Pat::Const",
        Pat::Paren(_) => "Pat::Paren",
        Pat::Rest(_) => "Pat::Rest",
        _ => "<unknown>",
    }
}

#[allow(dead_code)]
pub fn ty_kind(ty: &Type) -> &'static str {
    match ty {
        Type::Array(_) => "Array",
        Type::BareFn(_) => "BareFn",
        Type::Group(_) => "Group",
        Type::ImplTrait(_) => "ImplTrait",
        Type::Infer(_) => "Infer",
        Type::Macro(_) => "Macro",
        Type::Never(_) => "Never",
        Type::Paren(_) => "Paren",
        Type::Path(_) => "Path",
        Type::Ptr(_) => "Ptr",
        Type::Reference(_) => "Reference",
        Type::Slice(_) => "Slice",
        Type::TraitObject(_) => "TraitObject",
        Type::Tuple(_) => "Tuple",
        Type::Verbatim(_) => "Verbatim",
        _ => "<unknown>",
    }
}

#[allow(dead_code)]
pub fn item_kind(item: &Item) -> &'static str {
    match item {
        Item::Const(_) => "Const",
        Item::Enum(_) => "Enum",
        Item::ExternCrate(_) => "ExternCrate",
        Item::Fn(_) => "Fn",
        Item::ForeignMod(_) => "ForeignMod",
        Item::Impl(_) => "Impl",
        Item::Macro(_) => "Macro",
        Item::Mod(_) => "Mod",
        Item::Static(_) => "Static",
        Item::Struct(_) => "Struct",
        Item::Trait(_) => "Trait",
        Item::TraitAlias(_) => "TraitAlias",
        Item::Type(_) => "Type",
        Item::Union(_) => "Union",
        Item::Use(_) => "Use",
        Item::Verbatim(_) => "Verbatim",
        _ => "<unknown>",
    }
}
