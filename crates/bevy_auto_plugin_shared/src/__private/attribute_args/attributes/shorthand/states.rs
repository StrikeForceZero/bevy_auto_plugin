use crate::__private::attribute_args::attributes::prelude::{
    InitStateAttributeArgs, RegisterTypeAttributeArgs,
};
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

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StatesAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub derive: FlagOrList<NonEmptyPath>,
    pub reflect: FlagOrList<Ident>,
    pub register: bool,
    pub init: bool,
}

impl GenericsArgs for StatesAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl AutoPluginAttributeKind for StatesAttributeArgs {
    type Attribute = AutoPluginShortHandAttribute;
    fn attribute() -> Self::Attribute {
        Self::Attribute::States
    }
}

impl<'a> From<&'a StatesAttributeArgs> for RegisterTypeAttributeArgs {
    fn from(value: &'a StatesAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl<'a> From<&'a StatesAttributeArgs> for InitStateAttributeArgs {
    fn from(value: &'a StatesAttributeArgs) -> Self {
        Self {
            generics: value.generics.clone(),
        }
    }
}

impl ArgsBackToTokens for StatesAttributeArgs {
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
        if self.init {
            items.push(quote!(init));
        }
        tokens.extend(quote! { #(#items),* });
    }
}

impl ShortHandAttribute for StatesAttributeArgs {
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
            expanded_attrs.append(tokens::derive_states(&self.derive.items));
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
        if self.init {
            expanded_attrs
                .attrs
                .push(tokens::auto_init_states(mode.clone(), self.into()));
        }
        expanded_attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::attribute_args::attributes::shorthand::Mode;
    use crate::__private::util::combo::combos_one_per_group_or_skip;
    use crate::assert_vec_args_expand;
    use darling::ast::NestedMeta;
    use internal_test_util::{extract_punctuated_paths, vec_spread};
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
            for args in combos_one_per_group_or_skip(&[
                vec![quote!(derive), quote!(derive(Debug, Default))],
                vec![quote!(reflect), quote!(reflect(Debug, Default))],
                vec![quote!(register)],
                vec![quote!(init)],
            ]) {
                println!(
                    "checking mode: {}, args: {}",
                    mode.as_str(),
                    quote! { #(#args),*}
                );
                assert_vec_args_expand!(mode, StatesAttributeArgs, args);
            }
        }
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_expand_attrs_global() -> syn::Result<()> {
        let extras = extract_punctuated_paths(parse_quote!(A, B))
            .into_iter()
            .map(NonEmptyPath::try_from)
            .collect::<syn::Result<Vec<_>>>()?;
        let args: NestedMeta = parse_quote! {_(
            plugin = Test,
            derive(#(#extras),*),
            reflect(#(#extras),*),
            register,
            init,
        )};
        let args = GlobalArgs::<StatesAttributeArgs>::from_nested_meta(&args)?;
        let mode = Mode::Global {
            plugin: args.plugin.clone(),
        };
        let derive_attr = tokens::derive_states(&extras);
        let derive_reflect_path = tokens::derive_reflect_path();
        let reflect_args = vec_spread![..extras,];
        let reflect_attr = tokens::reflect(reflect_args.iter().map(NonEmptyPath::last_ident));
        println!("{}", args.inner.expand_attrs(&mode).to_token_stream());
        assert_eq!(
            args.inner.expand_attrs(&mode).to_token_stream().to_string(),
            ExpandAttrs {
                use_items: [derive_attr.use_items, reflect_attr.use_items].concat(),
                attrs: vec_spread![
                    ..derive_attr.attrs,
                    // TODO: merge these derives
                    quote! { #[derive(#derive_reflect_path)] },
                    quote! { #[reflect(#(#reflect_args),*)] },
                    tokens::auto_register_type(mode.clone(), (&args.inner).into()),
                    tokens::auto_init_states(mode.clone(), (&args.inner).into()),
                ]
            }
            .to_token_stream()
            .to_string()
        );
        Ok(())
    }
}
