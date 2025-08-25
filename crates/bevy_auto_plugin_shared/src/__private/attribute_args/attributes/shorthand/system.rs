use crate::__private::attribute_args::attributes::prelude::{
    AddSystemAttributeArgs, RegisterTypeAttributeArgs,
};
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::attributes::shorthand::{
    AutoPluginShortHandAttribute, ExpandAttrs, Mode, ShortHandAttribute, tokens,
};
use crate::__private::attribute_args::schedule_config::ScheduleWithScheduleConfigArgs;
use crate::__private::attribute_args::{AutoPluginAttributeKind, GenericsArgs};
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::{TokenStream as MacroStream, TokenStream};
use quote::quote;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct SystemAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
}

impl GenericsArgs for SystemAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for SystemAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::System
    }
}

impl<'a> From<&'a SystemAttributeArgs> for RegisterTypeAttributeArgs {
    fn from(value: &'a SystemAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a SystemAttributeArgs> for AddSystemAttributeArgs {
    fn from(value: &'a SystemAttributeArgs) -> Self {
        AddSystemAttributeArgs {
            generics: value.generics.clone(),
            schedule_config: value.schedule_config.clone(),
        }
    }
}

impl ArgsBackToTokens for SystemAttributeArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut items = vec![];
        items.extend(self.generics().to_attribute_arg_vec_tokens());
        items.extend(self.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for SystemAttributeArgs {
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
        expanded_attrs
            .attrs
            .push(tokens::auto_add_systems(mode.clone(), self.into()));
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::attribute_args::attributes::shorthand::Mode;
    use crate::assert_vec_args_expand;
    use darling::ast::NestedMeta;
    use internal_test_util::vec_spread;
    use quote::ToTokens;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        for mode in [
            Mode::Module,
            Mode::FlatFile,
            Mode::Global {
                plugin: parse_quote!(Test),
            },
        ] {
            let args = vec![quote! { schedule = Update }];
            println!(
                "checking mode: {}, args: {}",
                mode.as_str(),
                quote! { #(#args),*}
            );
            assert_vec_args_expand!(mode, SystemAttributeArgs, args);
        }
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
            schedule = Update,
        )};
        let args = GlobalArgs::<SystemAttributeArgs>::from_nested_meta(&args)?;
        let mode = Mode::Global {
            plugin: args.plugin.clone(),
        };
        println!("{}", args.inner.expand_attrs(&mode).to_token_stream());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            ExpandAttrs {
                use_items: vec![],
                attrs: vec_spread![tokens::auto_add_systems(mode.clone(), (&args.inner).into()),]
            }
            .to_token_stream()
            .to_string()
        );
        Ok(())
    }
}
