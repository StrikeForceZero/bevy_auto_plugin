use crate::__private::attribute_args::GlobalArgs;
use crate::__private::attribute_args::attributes::prelude::RegisterTypeAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::{
    ExpandAttrs, Mode, ShortHandAttribute, tokens,
};
use crate::__private::non_empty_path::NonEmptyPath;
use anyhow::anyhow;
use darling::ast::NestedMeta;
use darling::{FromMeta, ToTokens};
use proc_macro2::TokenStream;
use syn::__private::quote::quote;
use syn::parse::Parser;
use syn::{Attribute, Meta};

pub(crate) enum Side {
    Left,
    #[allow(dead_code)]
    Right,
}

pub(crate) fn _inject_derive(
    derives: &mut Vec<TokenStream>,
    additional_derive_items: &[NonEmptyPath],
    side: Side,
) {
    if additional_derive_items.is_empty() {
        return;
    }
    // hacky but works
    if let Some(index) = derives
        .iter()
        .position(|ts| ts.to_string().contains("derive"))
    {
        let existing_ts = derives.remove(index);
        let mut attrs = Attribute::parse_outer
            .parse2(existing_ts)
            .expect("existing derive not in expected format")
            .into_iter();
        let derive_attr = attrs.next().expect("empty - impossible?");
        assert!(
            attrs.next().is_none(),
            "expected exactly 1 derive per entry"
        );
        let list = match &derive_attr.meta {
            Meta::List(list) => list,
            _ => panic!("expected list attribute #[derive(...)]"),
        };
        let path = &list.path;
        let inner = &list.tokens;

        // Check if there are existing items for comma placement
        let combined = if inner.is_empty() {
            quote!(#[#path( #(#additional_derive_items),* )])
        } else {
            match side {
                Side::Left => quote!(#[#path( #(#additional_derive_items),*, #inner )]),
                Side::Right => quote!(#[#path( #inner, #(#additional_derive_items),* )]),
            }
        };
        derives.insert(index, combined);
    } else {
        derives.insert(0, tokens::derive_from(additional_derive_items));
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TestParams<T: FromMeta> {
    pub args: GlobalArgs<T>,
    pub expected_derives: ExpandAttrs,
    pub expected_reflect: ExpandAttrs,
    pub expected_extras: ExpandAttrs,
}

impl<T: FromMeta + Clone + ShortHandAttribute> TestParams<T>
where
    for<'a> RegisterTypeAttributeArgs: From<&'a T>,
{
    pub(crate) fn new(args: GlobalArgs<T>) -> Self {
        Self {
            args,
            expected_derives: ExpandAttrs::default(),
            expected_reflect: ExpandAttrs::default(),
            expected_extras: ExpandAttrs::default(),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn from_list(nested_metas: &[NestedMeta]) -> syn::Result<Self> {
        let args = GlobalArgs::<T>::from_list(nested_metas)?;
        Ok(Self::new(args))
    }
    #[allow(dead_code)]
    pub(crate) fn from_nested_meta(nested_meta: NestedMeta) -> syn::Result<Self> {
        let args = GlobalArgs::<T>::from_nested_meta(&nested_meta)?;
        Ok(Self::new(args))
    }

    pub(crate) fn from_args(args: TokenStream) -> syn::Result<Self> {
        let items = NestedMeta::parse_meta_list(args)?;
        Self::from_list(&items)
    }

    #[allow(dead_code)]
    pub(crate) fn from_attribute(attr: Attribute) -> syn::Result<Self> {
        let list = match &attr.meta {
            Meta::List(l) => l,
            _ => panic!("expected list"),
        };
        let nested = NestedMeta::parse_meta_list(list.tokens.clone())?;
        Self::from_list(&nested)
    }

    fn mode(&self) -> Mode {
        Mode::Global {
            plugin: self.args.plugin.clone(),
        }
    }

    /// calling order matters
    pub(crate) fn with_derive(mut self, extras: Vec<NonEmptyPath>) -> Self {
        self.expected_derives
            .attrs
            .push(tokens::derive_from(&extras));
        self
    }

    /// calling order matters
    pub(crate) fn with_reflect(mut self, extras: Vec<NonEmptyPath>, derive: bool) -> Self {
        if derive {
            self.expected_derives
                .attrs
                .push(tokens::derive_from(&[tokens::derive_reflect_path()]));
        }
        if !extras.is_empty() {
            self.expected_reflect
                .append(tokens::reflect(extras.iter().map(NonEmptyPath::last_ident)));
        }
        self
    }

    /// calling order matters
    pub(crate) fn with_register(mut self) -> Self {
        self.expected_extras.attrs.insert(
            0,
            tokens::auto_register_type(self.mode(), (&self.args.inner).into()),
        );
        self
    }

    fn build_into(self) -> ExpandAttrs {
        ExpandAttrs::default()
            .with(self.expected_derives)
            .with(self.expected_reflect)
            .with(self.expected_extras)
    }

    pub(crate) fn build_clone(&self) -> ExpandAttrs {
        self.clone().build_into()
    }

    pub(crate) fn test(&self) -> anyhow::Result<()> {
        let got = self
            .args
            .inner
            .expand_attrs(&self.mode())
            .to_token_stream()
            .to_string();
        let expected = self.build_clone().to_token_stream().to_string();
        if got != expected {
            return Err(anyhow!("\nexpected: {expected:#?}\n     got: {got:#?}"));
        }
        Ok(())
    }
}
