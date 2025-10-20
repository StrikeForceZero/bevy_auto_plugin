use crate::macro_api::attributes::ItemAttribute;
use crate::macro_api::attributes::prelude::AddSystemArgs;
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::macro_paths::MacroPathProvider;
use crate::macro_api::mixins::generics::with::WithGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

/// for codegen re-emitting macro args
struct QQ<'a, T> {
    args: &'a T,
    context: &'a Context,
    input_item: &'a mut InputItem,
}

impl<T> QQ<'_, T>
where
    T: MacroPathProvider,
    Self: ToTokens,
{
    fn inject_attribute_macro(&mut self) -> syn::Result<()> {
        let args = self.to_token_stream();
        self.input_item.map_ast(|item| {
            let macro_path = T::macro_path(self.context);
            // insert attribute tokens
            let mut attrs = item
                .take_attrs()
                .map_err(|err| syn::Error::new(item.span(), err))?;
            attrs.insert(0, parse_quote!(#[#macro_path(#args)]));
            item.put_attrs(attrs).unwrap(); // infallible
            Ok(())
        })
    }
}

impl ToTokens for QQ<'_, ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithGenerics>>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schedule = &self.args.args.core.schedule_config.schedule;
        let extra_args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#extra_args),*
            schedule = #schedule,
        });
    }
}
