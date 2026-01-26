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

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct InsertResourceArgs {
    // TODO: after removing legacy fields, remove _resolved, make insert required
    pub insert: Option<AnyExprCallClosureMacroPath>,
    pub resource: Option<AnyExprCallClosureMacroPath>,
    pub init: Option<AnyExprCallClosureMacroPath>,
    #[darling(skip)]
    _resolved: Option<AnyExprCallClosureMacroPath>,
}

impl InsertResourceArgs {
    fn validate(self) -> darling::Result<Self> {
        Ok(Self { _resolved: Some(Self::resolve_resource(&self)?.clone()), ..self })
    }
    fn resolve_resource(&self) -> darling::Result<&AnyExprCallClosureMacroPath> {
        if let Some(resolved) = self._resolved.as_ref() {
            Ok(resolved)
        } else {
            match (self.insert.as_ref(), self.resource.as_ref(), self.init.as_ref()) {
                (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
                    Err(darling::Error::custom(
                        "insert is mutually exclusive with init and resource, use only insert",
                    ))
                }
                (Some(res), None, None) => Ok(res),
                (None, Some(_), Some(_)) => Err(darling::Error::custom(
                    "resource and init are mutually exclusive, use only one",
                )),
                (None, None, None) => Err(darling::Error::custom(
                    "missing field: insert (or legacy init/resource)",
                )),
                (None, Some(res), None) => Ok(res),
                (None, None, Some(res)) => Ok(res),
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
        let resource = self.args.args.base.resolve_resource().map_err(syn::Error::from)?;
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #app_param.insert_resource({ let resource: #concrete_path = #resource; resource});
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
            args.push(quote! { insert = #insert });
        } else if let Some(init) = &base.init {
            args.push(quote! { init = #init });
        } else {
            let resource = &base.resource;
            args.push(quote! { resource = #resource });
        }
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
