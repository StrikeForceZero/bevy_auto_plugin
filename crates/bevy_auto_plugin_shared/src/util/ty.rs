use syn::{Type, TypeReference};

/// Check if the type is `&mut _`
pub fn is_mutable_reference(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Reference(TypeReference {
            mutability: Some(_),
            ..
        })
    )
}
