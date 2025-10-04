use crate::__private::attribute_args::attributes::prelude::{
    AutoNameAttributeArgs, RegisterTypeAttributeArgs,
};
use crate::__private::attribute_args::attributes::shorthand::tokens::ArgsBackToTokens;
use crate::__private::attribute_args::attributes::shorthand::{
    AutoPluginShortHandAttribute, ExpandAttrs, ShortHandAttribute, tokens,
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
pub struct ComponentAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub auto_name: bool,
}

impl GenericsArgs for ComponentAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for ComponentAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::Component
    }
}

impl<'a> From<&'a ComponentAttributeArgs> for RegisterTypeAttributeArgs {
    fn from(value: &'a ComponentAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a ComponentAttributeArgs> for AutoNameAttributeArgs {
    fn from(value: &'a ComponentAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl ArgsBackToTokens for ComponentAttributeArgs {
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
        if self.auto_name {
            items.push(quote!(auto_name));
        }
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for ComponentAttributeArgs {
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
            expanded_attrs
                .attrs
                .push(tokens::derive_component(&self.derive.items));
        }
        if self.reflect.present {
            if self.derive.present {
                expanded_attrs.attrs.push(tokens::derive_reflect());
            }
            let component_ident: Ident = parse_quote!(Component);
            let items = std::iter::once(&component_ident).chain(self.reflect.items.iter());
            expanded_attrs.append(tokens::reflect(items))
        }
        if self.register {
            expanded_attrs
                .attrs
                .push(tokens::auto_register_type(plugin.clone(), self.into()));
        }
        if self.auto_name {
            expanded_attrs
                .attrs
                .push(tokens::auto_name(plugin.clone(), self.into()));
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::util::combo::combos_one_per_group_or_skip;
    use crate::{assert_vec_args_expand, plugin};
    use darling::ast::NestedMeta;
    use internal_test_util::{extract_punctuated_paths, vec_spread};
    use quote::ToTokens;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_expand_back_into_args() -> syn::Result<()> {
        for args in combos_one_per_group_or_skip(&[
            vec![quote!(derive), quote!(derive(Debug, Default))],
            vec![quote!(reflect), quote!(reflect(Debug, Default))],
            vec![quote!(register)],
            vec![quote!(auto_name)],
        ]) {
            println!("checking args: {}", quote! { #(#args),*});
            assert_vec_args_expand!(plugin!(parse_quote!(Test)), ComponentAttributeArgs, args);
        }
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let extras = extract_punctuated_paths(parse_quote!(Debug, Default))
            .into_iter()
            .map(NonEmptyPath::try_from)
            .collect::<syn::Result<Vec<_>>>()?;
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
            auto_name,
        )};
        let args = GlobalArgs::<ComponentAttributeArgs>::from_nested_meta(&args)?;
        let derive_args = vec_spread![tokens::derive_component_path(), ..extras.clone(),];
        let derive_reflect_path = tokens::derive_reflect_path();
        let reflect_args = vec_spread![parse_quote!(Component), ..extras,];
        let reflect_attr = tokens::reflect(reflect_args.iter().map(NonEmptyPath::last_ident));
        assert_eq!(
            args.inner
                .expand_attrs(&args.plugin())
                .to_token_stream()
                .to_string(),
            ExpandAttrs {
                use_items: reflect_attr.use_items,
                attrs: vec![
                    quote! { #[derive(#(#derive_args),*)] },
                    // TODO: merge these derives
                    quote! { #[derive(#derive_reflect_path)] },
                    quote! { #[reflect(#(#reflect_args),*)] },
                    tokens::auto_register_type(args.plugin(), (&args.inner).into()),
                    tokens::auto_name(args.plugin(), (&args.inner).into()),
                ]
            }
            .to_token_stream()
            .to_string()
        );
        Ok(())
    }
}
