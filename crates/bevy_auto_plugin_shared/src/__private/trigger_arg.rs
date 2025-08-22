use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::trigger_type_arg::TriggerTypeArg;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{Meta, Type, parse_quote};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TriggerArg {
    pub name: Option<Ident>,
    pub trigger_type: TriggerTypeArg,
}

impl TriggerArg {
    pub fn back_to_inner_arg_tokens(&self) -> TokenStream {
        let mut items = vec![];
        if let Some(name) = &self.name {
            items.push(quote! { name = #name });
        }
        items.push(self.trigger_type.event.to_token_stream());
        if let Some(bundle) = &self.trigger_type.bundle {
            items.push(bundle.to_token_stream());
        }
        quote! { (#(#items),*) }
    }
}

impl ToTokens for TriggerArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.clone().unwrap_or_else(|| parse_quote!(trigger));
        let trigger_type = &self.trigger_type;
        tokens.extend(quote! {
            #name: #trigger_type,
        });
    }
}

impl From<&TriggerArg> for TokenStream {
    fn from(value: &TriggerArg) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl From<TriggerArg> for TokenStream {
    fn from(value: TriggerArg) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl FromMeta for TriggerArg {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut name: Option<Ident> = None;
        let mut positional: Vec<Type> = Vec::new();

        for item in items {
            match item {
                // name = foo_ident
                NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("name") => {
                    if let syn::Expr::Path(ep) = &nv.value {
                        if let Some(id) = ep.path.get_ident() {
                            name = Some(id.clone());
                        } else {
                            return Err(darling::Error::custom("`name` must be a bare ident")
                                .with_span(&nv.value));
                        }
                    } else if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = &nv.value
                    {
                        name = Some(Ident::new(&s.value(), s.span()));
                    } else {
                        return Err(darling::Error::custom("`name` must be ident or string")
                            .with_span(&nv.value));
                    }
                }
                // positional type like A, or path like my::A
                NestedMeta::Meta(Meta::Path(p)) => {
                    positional.push(Type::Path(syn::TypePath {
                        qself: None,
                        path: p.clone(),
                    }));
                }
                // allow a stringified type: "A<B>"
                NestedMeta::Lit(syn::Lit::Str(s)) => {
                    positional.push(syn::parse_str::<Type>(&s.value()).map_err(|e| {
                        darling::Error::custom(format!("bad type: {e}")).with_span(s)
                    })?);
                }
                // anything else is unsupported in this wrapper
                other => {
                    return Err(darling::Error::custom(
                        "unexpected argument; expected `name = ident` or a type",
                    )
                    .with_span(other));
                }
            }
        }

        if positional.is_empty() || positional.len() > 2 {
            return Err(darling::Error::custom(
                "expected 1-2 positional types after optional `name = `",
            ));
        }

        let trigger_type = TriggerTypeArg {
            event: positional.remove(0),
            bundle: positional.pop(),
        };
        Ok(Self { name, trigger_type })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{Attribute, parse_quote};

    #[internal_test_proc_macro::xtest]
    fn test_from_meta() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[trigger(name = foo, A, B)] };
        let trigger_args = TriggerArg::from_meta(&attr.meta)?;
        assert_eq!(trigger_args.name, Some(parse_quote!(foo)));
        assert_eq!(trigger_args.trigger_type.event, parse_quote!(A));
        assert_eq!(trigger_args.trigger_type.bundle, Some(parse_quote!(B)));
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_from_meta_no_name() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[trigger(A, B)] };
        let trigger_args = TriggerArg::from_meta(&attr.meta)?;
        assert_eq!(trigger_args.name, None);
        assert_eq!(trigger_args.trigger_type.event, parse_quote!(A));
        assert_eq!(trigger_args.trigger_type.bundle, Some(parse_quote!(B)));
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_from_meta_no_bundle() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[trigger(name = foo, A)] };
        let trigger_args = TriggerArg::from_meta(&attr.meta)?;
        assert_eq!(trigger_args.name, Some(parse_quote!(foo)));
        assert_eq!(trigger_args.trigger_type.event, parse_quote!(A));
        assert_eq!(trigger_args.trigger_type.bundle, None);
        Ok(())
    }
}
