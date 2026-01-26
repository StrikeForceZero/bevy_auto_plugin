use crate::{
    macro_api::prelude::*,
    syntax::ast::any_expr::AnyExprCallClosureMacroPath,
};
use darling::FromMeta;
use proc_macro2::{
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
    quote_spanned,
};
use std::hash::Hash;
use syn::{
    parse_quote,
    spanned::Spanned,
};

#[derive(FromMeta, Default, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct InsertResourceArgs {
    // TODO: after removing legacy fields, remove _resolved, make insert required
    pub insert: Option<MetaExpr>,
    pub resource: Option<MetaExpr>,
    pub init: Option<MetaExpr>,
    #[darling(skip)]
    _resolved: Option<AnyExprCallClosureMacroPath>,
}

#[derive(Debug, Clone)]
pub struct MetaExpr {
    value: AnyExprCallClosureMacroPath,
    span: Span,
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

impl MetaExpr {
    fn span(&self) -> Span {
        self.span
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
        let span = match item {
            syn::Meta::Path(path) => path.span(),
            syn::Meta::List(list) => list.path.span(),
            syn::Meta::NameValue(nv) => nv.path.span(),
        };
        Ok(Self { value, span })
    }

    fn from_nested_meta(item: &darling::ast::NestedMeta) -> darling::Result<Self> {
        match item {
            darling::ast::NestedMeta::Meta(meta) => Self::from_meta(meta),
            darling::ast::NestedMeta::Lit(lit) => {
                let value =
                    AnyExprCallClosureMacroPath::from_value(lit).map_err(|e| e.with_span(lit))?;
                Ok(Self { value, span: item.span() })
            }
        }
    }
}

impl InsertResourceArgs {
    pub fn from_init(init: AnyExprCallClosureMacroPath) -> Self {
        Self { init: Some(init), ..Default::default() }
    }
    fn validate(self) -> darling::Result<Self> {
        Ok(Self { _resolved: Some(Self::resolve_resource(&self)?.clone()), ..self })
    }
    fn resolve_resource(&self) -> darling::Result<&AnyExprCallClosureMacroPath> {
        if let Some(resolved) = self._resolved.as_ref() {
            Ok(resolved)
        } else {
            match (self.insert.as_ref(), self.resource.as_ref(), self.init.as_ref()) {
                (Some(insert), Some(_), _) | (Some(insert), _, Some(_)) => {
                    Err(darling::Error::custom(
                        "insert is mutually exclusive with init and resource, use only insert",
                    )
                    .with_span(&insert.span()))
                }
                (Some(res), None, None) => Ok(res.as_ref()),
                (None, Some(_), Some(_)) => Err(darling::Error::custom(
                    "resource and init are mutually exclusive, use only one",
                )),
                (None, None, None) => {
                    Err(darling::Error::custom("missing field: insert (or legacy init/resource)"))
                }
                (None, Some(res), None) => Ok(res.as_ref()),
                (None, None, Some(res)) => Ok(res.as_ref()),
            }
        }
    }
}

impl AttributeIdent for InsertResourceArgs {
    const IDENT: &'static str = "auto_insert_resource";
}

pub type IaInsertResource = ItemAttribute<
    Composed<InsertResourceArgs, WithPlugin, WithZeroOrOneGenerics>,
    AllowStructOrEnum,
>;
pub type InsertResourceAppMutEmitter = AppMutationEmitter<IaInsertResource>;
pub type InsertResourceAttrEmitter = AttrEmitter<IaInsertResource>;

impl EmitAppMutationTokens for InsertResourceAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let base = &self.args.args.base;
        if base.insert.is_none() {
            if let Some(init) = &base.init {
                emit_deprecated_insert_resource_warning(
                    tokens,
                    init.span(),
                    "auto_insert_resource(init(...)) is deprecated; use insert(...) instead",
                );
            } else if let Some(resource) = &base.resource {
                emit_deprecated_insert_resource_warning(
                    tokens,
                    resource.span(),
                    "auto_insert_resource(resource(...)) is deprecated; use insert(...) instead",
                );
            }
        }
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

fn emit_deprecated_insert_resource_warning(
    tokens: &mut TokenStream,
    span: proc_macro2::Span,
    message: &'static str,
) {
    tokens.extend(quote_spanned! { span=>
        {
            #[deprecated(note = #message)]
            fn __bevy_auto_plugin_deprecated_auto_insert_resource() {}
            __bevy_auto_plugin_deprecated_auto_insert_resource();
        }
    });
}

impl ToTokens for InsertResourceAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        let base = &self.args.args.base;
        if let Some(insert) = &base.insert {
            let insert = insert.as_ref();
            args.push(quote! { insert = #insert });
        } else if let Some(init) = &base.init {
            let init = init.as_ref();
            args.push(quote! { init = #init });
        } else {
            let resource = base.resource.as_ref().map(|res| res.as_ref());
            args.push(quote! { resource = #resource });
        }
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
