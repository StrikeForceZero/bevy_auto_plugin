use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::GenericsArgs;
use crate::__private::attribute_args::attributes::shorthand::{
    ExpandAttrs, Mode, ShortHandAttribute, tokens,
};
use crate::__private::flag_or_list::FlagOrList;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use syn::parse_quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ComponentAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: bool,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub auto_name: bool,
}

impl GenericsArgs for ComponentAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ShortHandAttribute for ComponentAttributeArgs {
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

        if self.derive {
            expanded_attrs.attrs.push(tokens::derive_component());
        }
        if self.reflect.present {
            if self.derive {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Component);
            let items = std::iter::once(&component_ident).chain(self.reflect.items.iter());
            expanded_attrs.append(tokens::reflect(items))
        }

        let args = self.expand_args(mode);
        // TODO: use the tokens::auto_register_type(..)
        if self.register {
            let macro_path = mode.resolve_macro_path(AutoPluginItemAttribute::RegisterType);
            expanded_attrs.attrs.push(quote! {
                #[#macro_path(#args)]
            });
        }
        // TODO: use the tokens::auto_name(..)
        if self.auto_name {
            let macro_path = mode.resolve_macro_path(AutoPluginItemAttribute::AutoName);
            expanded_attrs.attrs.push(quote! {
                #[#macro_path(#args)]
            });
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::attribute_args::attributes::auto_name::AutoNameAttributeArgs;
    use crate::__private::attribute_args::attributes::prelude::RegisterTypeAttributeArgs;
    use crate::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use crate::__private::attribute_args::attributes::shorthand::component::ComponentAttributeArgs;
    use crate::__private::attribute_args::attributes::shorthand::tokens;
    use crate::__private::attribute_args::attributes::shorthand::{ExpandAttrs, Mode};
    use crate::__private::type_list::TypeList;
    use crate::__private::util::extensions::from_meta::FromMetaExt;
    use proc_macro2::Ident;
    use quote::{ToTokens, quote};
    use syn::{Attribute, parse_quote};

    fn reflect_component(extra_items: impl IntoIterator<Item = Ident>) -> ExpandAttrs {
        let mut items = vec![parse_quote!(Component)];
        items.extend(extra_items.into_iter());
        tokens::reflect(items.iter())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_module() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_component(derive, reflect, register, auto_name)] };
        let args = ComponentAttributeArgs::from_meta_ext(&attr.meta)?;
        let mode = Mode::Module;
        let (reflect_component_use, reflect_component_attrs) =
            reflect_component([]).to_use_attr_ts_tuple();
        let derive_component = tokens::derive_component();
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_name = tokens::auto_name(mode.clone(), AutoNameAttributeArgs::default());
        assert_eq!(
            args.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_component_use

                #derive_component
                #derive_reflect
                #reflect_component_attrs
                #auto_register_type
                #auto_name
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_flat_file() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_component(derive, reflect, register, auto_name)] };
        let args = ComponentAttributeArgs::from_meta_ext(&attr.meta)?;
        let mode = Mode::FlatFile;
        let (reflect_component_use, reflect_component_attrs) =
            reflect_component([]).to_use_attr_ts_tuple();
        let derive_component = tokens::derive_component();
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_name = tokens::auto_name(mode.clone(), AutoNameAttributeArgs::default());
        assert_eq!(
            args.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_component_use

                #derive_component
                #derive_reflect
                #reflect_component_attrs
                #auto_register_type
                #auto_name
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_global() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_component(plugin = Test, derive, reflect, register, auto_name)] };
        let args = GlobalArgs::<ComponentAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_component_use, reflect_component_attrs) =
            reflect_component([]).to_use_attr_ts_tuple();
        let derive_component = tokens::derive_component();
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_name = tokens::auto_name(mode.clone(), AutoNameAttributeArgs::default());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_component_use

                #derive_component
                #derive_reflect
                #reflect_component_attrs
                #auto_register_type
                #auto_name
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_global_generics() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_component(plugin = Test, generics(u8, bool), generics(u32, f32), derive, reflect, register, auto_name)] };
        let args = GlobalArgs::<ComponentAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_component_use, reflect_component_attrs) =
            reflect_component([]).to_use_attr_ts_tuple();
        let derive_component = tokens::derive_component();
        let derive_reflect = tokens::derive_reflect();
        let generics = vec![
            TypeList(vec![parse_quote!(u8), parse_quote!(bool)]),
            TypeList(vec![parse_quote!(u32), parse_quote!(f32)]),
        ];
        let auto_register_type = tokens::auto_register_type(
            mode.clone(),
            RegisterTypeAttributeArgs {
                generics: generics.clone(),
            },
        );
        let auto_name = tokens::auto_name(
            mode.clone(),
            AutoNameAttributeArgs {
                generics: generics.clone(),
            },
        );
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_component_use

                #derive_component
                #derive_reflect
                #reflect_component_attrs
                #auto_register_type
                #auto_name
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_multiple_reflect() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_component(plugin = Test, derive, reflect(Debug, Default), register, auto_name)] };
        let args = GlobalArgs::<ComponentAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_component_use, reflect_component_attrs) =
            reflect_component([parse_quote!(Debug), parse_quote!(Default)]).to_use_attr_ts_tuple();
        let derive_component = tokens::derive_component();
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_name = tokens::auto_name(mode.clone(), AutoNameAttributeArgs::default());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_component_use

                #derive_component
                #derive_reflect
                #reflect_component_attrs
                #auto_register_type
                #auto_name
            }
            .to_string()
        );
        Ok(())
    }
}
