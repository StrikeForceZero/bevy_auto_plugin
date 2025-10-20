use crate::macro_api::attributes::ItemAttribute;
use crate::macro_api::attributes::prelude::AddSystemArgs;
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::mixins::generics::with::WithGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

/// for codegen attaching to bevy app
struct Q<'a, T> {
    args: &'a T,
    context: &'a Context,
    input_item: &'a InputItem,
}

impl ToTokens for Q<'_, ItemAttribute<Composed<AddSystemArgs, WithPlugin, WithGenerics>>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schedule = &self.args.args.base.schedule_config.schedule;
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! { |app| {
                app.add_system(#schedule, #concrete_path);
            }});
        }
    }
}
