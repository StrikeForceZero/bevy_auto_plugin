use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::attribute_args::attributes::shorthand::{
    ExpandAttrs, Mode, ShortHandAttribute,
};
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::TokenStream as MacroStream;
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ResourceAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: bool,
    pub reflect: bool,
    pub register: bool,
    pub init: bool,
}

impl ShortHandAttribute for ResourceAttributeArgs {
    fn expand_args(&self, mode: &Mode) -> MacroStream {
        let mut args = Vec::new();
        if let Mode::Global { plugin } = &mode {
            args.push(quote! { plugin = #plugin });
        };
        args.extend(self
            .generics
            .iter()
            .filter_map(|g| {
                if g.is_empty() {
                    None
                } else {
                    Some(quote! { generics(#g) })
                }
            }));
        quote! { #(#args),* }
    }

    fn expand_attrs(&self, mode: &Mode) -> ExpandAttrs {
        let mut expanded_attrs = ExpandAttrs::default();

        if self.derive {
            expanded_attrs.attrs.push(quote! {
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource)]
            });
        }
        if self.reflect {
            if self.derive {
                expanded_attrs.attrs.push(quote! {
                    #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)]
                });
            }
            expanded_attrs.use_items.push(quote! {
                // Make the helper available for #[reflect(Component)]
                // TODO: we could eliminate the need for globs if we pass the ident in
                //  then we can do `ReflectComponent as ReflectResource$ident`
                //  #[reflect(Resource$ident)]
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
            });
            expanded_attrs.attrs.push(quote! {
                // reflect is helper attribute and expects Ident
                #[reflect(Resource)]
            });
        }

        let args = self.expand_args(mode);
        if self.register {
            let macro_path = mode.resolve_macro_path(AutoPluginItemAttribute::RegisterType);
            expanded_attrs.attrs.push(quote! {
                #[#macro_path(#args)]
            });
        }
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
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::attribute_args::attributes::shorthand::Mode;
    use crate::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use crate::__private::attribute_args::attributes::shorthand::resource::ResourceAttributeArgs;
    use crate::__private::util::extensions::from_meta::FromMetaExt;
    use quote::{ToTokens, quote};
    use syn::{Attribute, parse_quote};

    #[internal_test_proc_macro::xtest]
    fn test_expand_module() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_resource(derive, reflect, register, init)] };
        let args = ResourceAttributeArgs::from_meta_ext(&attr.meta)?;
        assert_eq!(
            args.expand_attrs(&Mode::Module).to_token_stream().to_string(),
            quote! {
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
                
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource)]
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)]
                #[reflect(Resource)]
                #[::bevy_auto_plugin::modes::module::prelude::auto_register_type()]
                #[::bevy_auto_plugin::modes::module::prelude::auto_init_resource()]
            }
                .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_flat_file() -> syn::Result<()> {
        let attr: Attribute =
            parse_quote! { #[auto_resource(derive, reflect, register, init)] };
        let args = ResourceAttributeArgs::from_meta_ext(&attr.meta)?;
        assert_eq!(
            args.expand_attrs(&Mode::FlatFile).to_token_stream().to_string(),
            quote! {
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
                
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource)]
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)]
                #[reflect(Resource)]
                #[::bevy_auto_plugin::modes::flat_file::prelude::auto_register_type()]
                #[::bevy_auto_plugin::modes::flat_file::prelude::auto_init_resource()]
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
        assert_eq!(
            args.inner
                .expand_attrs(&Mode::Global {
                    plugin: args.plugin
                })
                .to_token_stream()
                .to_string(),
            quote! {
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
                
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource)]
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)]
                #[reflect(Resource)]
                #[::bevy_auto_plugin::modes::global::prelude::auto_register_type(plugin = Test)]
                #[::bevy_auto_plugin::modes::global::prelude::auto_init_resource(plugin = Test)]
            }
                .to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_global_generics() -> syn::Result<()> {
        let attr: Attribute = parse_quote! { #[auto_resource(plugin = Test, generics(u8, bool), generics(u32, f32), derive, reflect, register, init)] };
        let args = GlobalArgs::<ResourceAttributeArgs>::from_meta_ext(&attr.meta)?;
        assert_eq!(
            args.inner
                .expand_attrs(&Mode::Global {
                    plugin: args.plugin
                })
                .to_token_stream()
                .to_string(),
            quote! {
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
                
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource)]
                #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)]
                #[reflect(Resource)]
                #[::bevy_auto_plugin::modes::global::prelude::auto_register_type(plugin = Test, generics(u8, bool), generics(u32, f32))]
                #[::bevy_auto_plugin::modes::global::prelude::auto_init_resource(plugin = Test, generics(u8, bool), generics(u32, f32))]
            }
                .to_string()
        );
        Ok(())
    }
}
