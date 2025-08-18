use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::attributes::shorthand::{
    AutoPluginShortHandAttribute, ExpandAttrs, Mode, ShortHandAttribute, tokens,
};
use crate::__private::attribute_args::{AutoPluginAttributeKind, GenericsArgs};
use crate::__private::flag_or_list::FlagOrList;
use crate::__private::non_empty_path::NonEmptyPath;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::quote;
use syn::parse_quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ResourceAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl GenericsArgs for ResourceAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for ResourceAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::Resource
    }
}

impl ArgsBackToTokens for ResourceAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut items = vec![];
        items.extend(self.generics().to_attribute_arg_vec_tokens());
        if self.derive.present {
            items.push(self.derive.to_outer_tokens("derive"));
        }
        if self.reflect.present {
            items.push(self.derive.to_outer_tokens("reflect"));
        }
        if self.register {
            items.push(quote!(register));
        }
        if self.init {
            items.push(quote!(init));
        }
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for ResourceAttributeArgs {
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
            expanded_attrs
                .attrs
                .push(tokens::derive_resource(&self.derive.items));
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Resource);
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
        if self.init {
            let macro_path = mode.resolve_macro_path(AutoPluginItemAttribute::InitResource);
            expanded_attrs.attrs.push(quote! {
                #[#macro_path(#args)]
            });
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use crate::__private::attribute_args::attributes::prelude::{
        InitResourceAttributeArgs, RegisterTypeAttributeArgs,
    };
    use crate::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use crate::__private::attribute_args::attributes::shorthand::resource::ResourceAttributeArgs;
    use crate::__private::attribute_args::attributes::shorthand::tokens;
    use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsWithMode;
    use crate::__private::attribute_args::attributes::shorthand::{ExpandAttrs, Mode};
    use crate::__private::attribute_args::{AutoPluginAttributeKind, GlobalArgs};
    use crate::__private::non_empty_path::NonEmptyPath;
    use crate::__private::type_list::TypeList;
    use crate::__private::util::extensions::from_meta::FromMetaExt;
    use proc_macro2::{Ident, TokenStream};
    use quote::{ToTokens, quote};
    use syn::{Attribute, parse_quote};

    fn reflect_resource(extra_items: impl IntoIterator<Item = Ident>) -> ExpandAttrs {
        let resource_ident: Ident = parse_quote!(Resource);
        let mut items = vec![&resource_ident];
        let extra_items = extra_items.into_iter().collect::<Vec<_>>();
        items.extend(extra_items.iter());
        tokens::reflect(items)
    }

    fn derive_resource(extra_items: impl IntoIterator<Item = NonEmptyPath>) -> TokenStream {
        let extra_items = extra_items.into_iter().collect::<Vec<_>>();
        tokens::derive_resource(&extra_items)
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_module() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(derive, reflect, register, init)] };
        let args = ResourceAttributeArgs::from_meta_ext(&attr.meta)?;
        let mode = Mode::Module;
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([]);
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_init_resource =
            tokens::auto_init_resource(mode.clone(), InitResourceAttributeArgs::default());
        assert_eq!(
            args.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_flat_file() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(derive, reflect, register, init)] };
        let args = ResourceAttributeArgs::from_meta_ext(&attr.meta)?;
        let mode = Mode::FlatFile;
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([]);
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_init_resource =
            tokens::auto_init_resource(mode.clone(), InitResourceAttributeArgs::default());
        assert_eq!(
            args.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_global() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_resource(plugin = Test, derive, reflect, register, init)] };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([]);
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_init_resource =
            tokens::auto_init_resource(mode.clone(), InitResourceAttributeArgs::default());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_global_generics() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(plugin = Test, generics(u8, bool), generics(u32, f32), derive, reflect, register, init)] };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([]);
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
        let auto_init_resource = tokens::auto_init_resource(
            mode.clone(),
            InitResourceAttributeArgs {
                generics: generics.clone(),
            },
        );
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_multiple_reflect() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(plugin = Test, derive, reflect(Debug, Default), register, init)] };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([parse_quote!(Debug), parse_quote!(Default)]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([]);
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_init_resource =
            tokens::auto_init_resource(mode.clone(), InitResourceAttributeArgs::default());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_multiple_derive() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(plugin = Test, derive(Debug, Default), reflect, register, init)] };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        let mode = Mode::Global {
            plugin: parse_quote!(Test),
        };
        let (reflect_resource_use, reflect_resource_attrs) =
            reflect_resource([]).to_use_attr_ts_tuple();
        let derive_resource = derive_resource([parse_quote!(Debug), parse_quote!(Default)]);
        let derive_reflect = tokens::derive_reflect();
        let auto_register_type =
            tokens::auto_register_type(mode.clone(), RegisterTypeAttributeArgs::default());
        let auto_init_resource =
            tokens::auto_init_resource(mode.clone(), InitResourceAttributeArgs::default());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            quote! {
                #reflect_resource_use

                #derive_resource
                #derive_reflect
                #reflect_resource_attrs
                #auto_register_type
                #auto_init_resource
            }
            .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        let macro_path = Mode::Global {
            plugin: parse_quote!(Test),
        }
        .resolve_macro_path(ResourceAttributeArgs::attribute());
        let input = quote! { #[#macro_path (plugin = Test, derive(Debug, Default), reflect(Debug, Default), register, init)] };
        let attr: Attribute = parse_quote! { #input };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        assert_eq!(
            ArgsWithMode::from(args).to_token_stream().to_string(),
            input.to_string()
        );
        Ok(())
    }
}
