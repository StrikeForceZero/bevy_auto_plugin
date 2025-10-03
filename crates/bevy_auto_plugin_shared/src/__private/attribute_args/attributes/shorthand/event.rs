use crate::__private::attribute_args::attributes::prelude::RegisterTypeAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::attributes::shorthand::{
    AutoPluginShortHandAttribute, ExpandAttrs, Mode, ShortHandAttribute, tokens,
};
use crate::__private::attribute_args::{AutoPluginAttributeKind, GenericsArgs};
use crate::__private::flag_or_list::FlagOrList;
use crate::__private::flag_or_meta::FlagOrMeta;
use crate::__private::non_empty_path::NonEmptyPath;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
enum Propagate {
    #[default]
    Default,
    Custom(syn::Path),
}

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct EntityEventOpts {
    #[darling(default, with = parse_propagate)]
    propagate: Option<Propagate>,
    auto_propagate: bool,
}

fn parse_propagate(meta: &syn::Meta) -> darling::Result<Option<Propagate>> {
    use darling::Error;
    use syn::{Expr, ExprPath, Meta, MetaNameValue};

    match meta {
        // `propagate`
        Meta::Path(p) if p.is_ident("propagate") => Ok(Some(Propagate::Default)),

        // `propagate = <Path>`
        Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("propagate") => {
            match value {
                Expr::Path(ExprPath { path, .. }) => Ok(Some(Propagate::Custom(path.clone()))),
                other => Err(Error::custom("expected a path after `propagate =`").with_span(other)),
            }
        }

        // `propagate(<Path>)`
        Meta::List(list) if list.path.is_ident("propagate") => {
            let p: syn::Path =
                syn::parse2(list.tokens.clone()).map_err(|e| Error::custom(e).with_span(list))?;
            Ok(Some(Propagate::Custom(p)))
        }

        // Not our field
        _ => Ok(None),
    }
}

impl EntityEventOpts {
    pub(crate) fn is_empty(&self) -> bool {
        self.propagate.is_none() && !self.auto_propagate
    }
}

impl ToTokens for EntityEventOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut items = vec![];
        if let Some(propagate) = &self.propagate {
            match propagate {
                Propagate::Default => {
                    items.push(quote! { propagate });
                }
                Propagate::Custom(path) => {
                    items.push(quote! { propagate = #path });
                }
            }
        }
        if self.auto_propagate {
            items.push(quote! { auto_propagate });
        }
        tokens.extend(quote! { #(#items),* });
    }
}

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct EventAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    // exclusive items
    pub global: bool,
    pub entity: FlagOrMeta<EntityEventOpts>,
}

impl GenericsArgs for EventAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for EventAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::Event
    }
}

impl<'a> From<&'a EventAttributeArgs> for RegisterTypeAttributeArgs {
    fn from(value: &'a EventAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl ArgsBackToTokens for EventAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut items = vec![];
        items.extend(self.generics().to_attribute_arg_vec_tokens());
        if self.derive.present {
            items.push(self.derive.to_outer_tokens("derive"));
        }
        if self.reflect.present {
            items.push(self.reflect.to_outer_tokens("reflect"));
        }
        if self.register {
            items.push(quote!(register));
        }
        if self.global {
            items.push(quote!(global));
        }
        if self.entity.present {
            if let Some(entity_inner_meta) = &self.entity.inner_meta {
                items.push(quote!(entity( #entity_inner_meta )));
            } else {
                items.push(quote!(entity));
            }
        }
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for EventAttributeArgs {
    fn expand_args(&self, mode: &Mode) -> MacroStream {
        let mut args = Vec::new();
        if let Mode::Global { plugin } = &mode {
            args.push(quote! { plugin = #plugin });
        };
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        quote! { #(#args),* }
    }

    fn expand_attrs(&self, mode: &Mode) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();

        if self.derive.present {
            if !self.global && !self.entity.present {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_from(&self.derive.items));
            }
            if self.global {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_event(&self.derive.items));
            }
            if self.entity.present {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_entity_event(&self.derive.items));
            }
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            expanded_attrs.append(tokens::reflect(&self.reflect.items))
        }
        if self.register {
            expanded_attrs
                .attrs
                .push(tokens::auto_register_type(mode.clone(), self.into()));
        }
        if self.entity.present
            && let Some(entity_event_opts) = &self.entity.inner_meta
            && !entity_event_opts.is_empty()
        {
            expanded_attrs
                .attrs
                .push(quote! { #[entity_event( #entity_event_opts )] });
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::attributes::shorthand::Mode;
    use crate::__private::util::combo::combos_one_per_group_or_skip;
    use crate::__private::util::test_params::{_inject_derive, Side, TestParams as _TestParams};
    use crate::assert_vec_args_expand;
    use internal_test_util::extract_punctuated_paths;
    use syn::parse_quote;

    type TestParams = _TestParams<EventAttributeArgs>;

    pub trait TestParamsExt {
        fn with_global(self, derive: bool) -> Self;
        fn with_entity_event(self, entity_event_opts: EntityEventOpts, derive: bool) -> Self;
    }

    impl TestParamsExt for TestParams {
        /// calling order matters
        fn with_global(mut self, derive: bool) -> Self {
            if derive {
                _inject_derive(
                    &mut self.expected_derives.attrs,
                    &[tokens::derive_event_path()],
                    Side::Left,
                );
            }
            self
        }

        /// calling order matters
        fn with_entity_event(mut self, entity_event_opts: EntityEventOpts, derive: bool) -> Self {
            if derive {
                _inject_derive(
                    &mut self.expected_derives.attrs,
                    &[tokens::derive_entity_event_path()],
                    Side::Left,
                );
            }
            if !entity_event_opts.is_empty() {
                self.expected_extras
                    .attrs
                    .push(quote!( #[entity_event( #entity_event_opts )] ));
            }
            self
        }
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        for mode in [
            Mode::Module,
            Mode::FlatFile,
            Mode::Global {
                plugin: parse_quote!(Test),
            },
        ] {
            for args in combos_one_per_group_or_skip(&[
                vec![quote!(derive), quote!(derive(Debug, Default))],
                vec![quote!(reflect), quote!(reflect(Debug, Default))],
                vec![quote!(register)],
            ]) {
                println!(
                    "checking mode: {}, args: {}",
                    mode.as_str(),
                    quote! { #(#args),*}
                );
                assert_vec_args_expand!(mode, EventAttributeArgs, args);
            }
        }
        Ok(())
    }

    fn extras() -> Vec<NonEmptyPath> {
        extract_punctuated_paths(parse_quote!(Debug, Default))
            .into_iter()
            .map(NonEmptyPath::try_from)
            .collect::<syn::Result<Vec<_>>>()
            .expect("failed to extract punctuated paths")
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_default() -> anyhow::Result<()> {
        TestParams::from_args(quote! {
            plugin = Test,
        })?
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_no_global_or_entity_flags() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global_event() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            global,
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_global(true)
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global_event_no_derive() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            global,
            reflect(#(#extras),*),
            register,
        })?
        .with_global(false)
        .with_reflect(extras.clone(), false)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            entity,
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(EntityEventOpts::default(), true)
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_no_derive() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            entity,
            reflect(#(#extras),*),
            register,
        })?
        .with_entity_event(EntityEventOpts::default(), false)
        .with_reflect(extras.clone(), false)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_propagate() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            entity(propagate),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(
            EntityEventOpts {
                propagate: Some(Propagate::Default),
                ..Default::default()
            },
            true,
        )
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_propagate_custom() -> anyhow::Result<()> {
        let extras = extras();
        let propagator = parse_quote!(TestEventPropagator);
        TestParams::from_args(quote! {
            plugin = Test,
            entity(propagate = #propagator),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(
            EntityEventOpts {
                propagate: Some(Propagate::Custom(propagator)),
                ..Default::default()
            },
            true,
        )
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_propagate_custom_and_auto_propagate() -> anyhow::Result<()> {
        let extras = extras();
        let propagator = parse_quote!(TestEventPropagator);
        TestParams::from_args(quote! {
            plugin = Test,
            entity(propagate = #propagator, auto_propagate),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(
            EntityEventOpts {
                propagate: Some(Propagate::Custom(propagator)),
                auto_propagate: true,
            },
            true,
        )
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_auto_propagate() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            entity(auto_propagate),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(
            EntityEventOpts {
                auto_propagate: true,
                ..Default::default()
            },
            true,
        )
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }
}
