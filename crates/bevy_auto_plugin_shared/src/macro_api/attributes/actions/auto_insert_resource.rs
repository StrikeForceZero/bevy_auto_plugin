use crate::{
    macro_api::prelude::*,
    syntax::ast::any_expr::AnyExprCallClosureMacroPath,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};
use std::hash::Hash;
use syn::parse_quote;

#[derive(FromMeta, Default, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct InsertResourceArgs {
    pub insert: Option<MetaExpr>,
}

#[derive(Debug, Clone)]
pub struct MetaExpr {
    value: AnyExprCallClosureMacroPath,
}

impl MetaExpr {
    pub fn from_value(value: AnyExprCallClosureMacroPath) -> Self {
        Self { value }
    }
}

impl PartialEq for MetaExpr {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for MetaExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl AsRef<AnyExprCallClosureMacroPath> for MetaExpr {
    fn as_ref(&self) -> &AnyExprCallClosureMacroPath {
        &self.value
    }
}

impl FromMeta for MetaExpr {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        let value = AnyExprCallClosureMacroPath::from_meta(item).map_err(|e| e.with_span(item))?;
        Ok(Self { value })
    }

    fn from_nested_meta(item: &darling::ast::NestedMeta) -> darling::Result<Self> {
        match item {
            darling::ast::NestedMeta::Meta(meta) => Self::from_meta(meta),
            darling::ast::NestedMeta::Lit(lit) => {
                let value =
                    AnyExprCallClosureMacroPath::from_value(lit).map_err(|e| e.with_span(lit))?;
                Ok(Self { value })
            }
        }
    }
}

impl InsertResourceArgs {
    pub fn from_insert(insert: AnyExprCallClosureMacroPath) -> Self {
        Self { insert: Some(MetaExpr::from_value(insert)) }
    }
    fn validate(self) -> darling::Result<Self> {
        if self.insert.is_none() {
            return Err(darling::Error::missing_field("insert"));
        }
        Ok(self)
    }
    fn resolve_resource(&self) -> darling::Result<&AnyExprCallClosureMacroPath> {
        self.insert
            .as_ref()
            .map(AsRef::as_ref)
            .ok_or_else(|| darling::Error::missing_field("insert"))
    }
}

impl AttributeIdent for InsertResourceArgs {
    const IDENT: &'static str = "auto_insert_resource";
}

pub type IaInsertResource = ItemAttribute<
    Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
    AllowStructOrEnumOrUse,
>;
pub type InsertResourceAppMutEmitter = AppMutationEmitter<IaInsertResource>;
pub type InsertResourceAttrEmitter = AttrEmitter<IaInsertResource>;

impl EmitAppMutationTokens for InsertResourceAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let resource = self.args.args.base.resolve_resource().map_err(syn::Error::from)?;
        let concrete_paths = self.args.concrete_paths()?;
        let placeholder_path = if self.args.args.generics().is_empty() {
            let type_params = self.args.input_item.type_param_idents()?;
            if type_params.is_empty() {
                None
            } else {
                let placeholders: Vec<syn::Type> =
                    (0..type_params.len()).map(|_| parse_quote!(_)).collect();
                let target = &self.args.target;
                Some(parse_quote!(#target::<#(#placeholders),*>))
            }
        } else {
            None
        };
        for concrete_path in concrete_paths {
            let ty_path = placeholder_path.as_ref().unwrap_or(&concrete_path);
            tokens.extend(quote! {
                #app_param.insert_resource({ let resource: #ty_path = #resource; resource});
            });
        }
        Ok(())
    }
}

impl ToTokens for InsertResourceAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let base = &self.args.args.base;
        if let Some(insert) = &base.insert {
            let insert = insert.as_ref();
            args.push(quote! { insert = #insert });
        }
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
