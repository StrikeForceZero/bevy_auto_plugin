use crate::__private::attribute::{AutoPluginShortHandAttribute, ShortHandAttribute};
use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::{ExpandAttrs, tokens};
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::global_args::AutoPluginAttributeKind;
use crate::macro_api::global_args::GenericsArgs;
use crate::syntax::ast::flag_or_list::FlagOrList;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::{ToTokens, quote};

#[derive(FromMeta, Default, Debug, Copy, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub enum EventTarget {
    #[default]
    #[darling(rename = "global")]
    Global,
    #[darling(rename = "entity")]
    Entity,
}

impl ToTokens for EventTarget {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            EventTarget::Global => {
                tokens.extend(quote!(global));
            }
            EventTarget::Entity => {
                tokens.extend(quote!(entity));
            }
        }
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
    pub target: EventTarget,
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
        let target = self.target;
        items.push(quote! { target(#target) });
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
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for EventAttributeArgs {
    fn expand_args(&self, plugin: &NonEmptyPath) -> MacroStream {
        let mut args = Vec::new();
        args.push(quote! { plugin = #plugin });
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        quote! { #(#args),* }
    }

    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();

        if self.derive.present {
            if matches!(self.target, EventTarget::Global) {
                expanded_attrs
                    .attrs
                    .push(tokens::derive_event(&self.derive.items));
            }
            if matches!(self.target, EventTarget::Entity) {
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
                .push(tokens::auto_register_type(plugin.clone(), self.into()));
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::macros::*;
    use crate::util::combo::combos_one_per_group_or_skip_with;
    use crate::util::test_params::{_inject_derive, Side, TestParams as _TestParams};
    use internal_test_util::extract_punctuated_paths;
    use syn::parse_quote;

    type TestParams = _TestParams<EventAttributeArgs>;

    pub trait TestParamsExt {
        fn with_global(self, derive: bool) -> Self;
        fn with_entity_event(self, derive: bool) -> Self;
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
        fn with_entity_event(mut self, derive: bool) -> Self {
            if derive {
                _inject_derive(
                    &mut self.expected_derives.attrs,
                    &[tokens::derive_entity_event_path()],
                    Side::Left,
                );
            }
            self
        }
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        for args in combos_one_per_group_or_skip_with(
            &[
                vec![quote!(derive), quote!(derive(Debug, Default))],
                vec![quote!(reflect), quote!(reflect(Debug, Default))],
                vec![quote!(register)],
            ],
            // TODO: target(global) is always emitted when no target is provided
            quote!(target(global)),
        ) {
            println!("checking args: {}", quote! { #(#args),*});
            assert_vec_args_expand!(plugin!(parse_quote!(Test)), EventAttributeArgs, args);
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
        .with_global(true)
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
            target(global),
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
            target(global),
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
            target(entity),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(true)
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
            target(entity),
            reflect(#(#extras),*),
            register,
        })?
        .with_entity_event(false)
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
            target(entity),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(true)
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_propagate_custom() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            target(entity),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(true)
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_entity_event_propagate_custom_and_auto_propagate() -> anyhow::Result<()> {
        let extras = extras();
        TestParams::from_args(quote! {
            plugin = Test,
            target(entity),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(true)
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
            target(entity),
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
        })?
        .with_derive(extras.clone())
        .with_entity_event(true)
        .with_reflect(extras.clone(), true)
        .with_register()
        .test()?;
        Ok(())
    }
}
