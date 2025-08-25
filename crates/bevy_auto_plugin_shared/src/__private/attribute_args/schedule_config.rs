use darling::FromMeta;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::Path;

#[derive(FromMeta, Clone, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct ScheduleWithScheduleConfigArgs {
    pub schedule: Path,
    #[darling(default)]
    pub config: ScheduleConfigArgs,
}

impl ScheduleWithScheduleConfigArgs {
    pub fn to_inner_arg_tokens_vec(&self) -> Vec<MacroStream> {
        let mut tokens = vec![];
        let schedule = &self.schedule;
        tokens.push(quote! { schedule = #schedule });
        let config = self.config.to_inner_arg_tokens_vec();
        if !config.is_empty() {
            tokens.push(quote! { config( #(#config),* )});
        }
        tokens
    }
}

#[derive(FromMeta, Clone, Debug, Default, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct ScheduleConfigArgs {
    pub in_set: Option<Path>,
    pub before: Option<Path>,
    pub after: Option<Path>,
    pub run_if: Option<Path>,
    pub distributive_run_if: Option<Path>,
    pub ambiguous_with: Option<Path>,
    pub ambiguous_with_all: Option<bool>,
    pub after_ignore_deferred: Option<Path>,
    pub before_ignore_deferred: Option<Path>,
}

impl ScheduleConfigArgs {
    pub fn to_inner_arg_tokens_vec(&self) -> Vec<MacroStream> {
        let mut tokens = vec![];
        if let Some(in_set) = &self.in_set {
            tokens.push(quote! {
                in_set = #in_set
            });
        }
        if let Some(before) = &self.before {
            tokens.push(quote! {
                before = #before
            });
        }
        if let Some(after) = &self.after {
            tokens.push(quote! {
                after = #after
            });
        }
        if let Some(run_if) = &self.run_if {
            tokens.push(quote! {
                run_if = #run_if
            });
        }
        if let Some(distributive_run_if) = &self.distributive_run_if {
            tokens.push(quote! {
                distributive_run_if = #distributive_run_if
            });
        }
        if let Some(ambiguous_with) = &self.ambiguous_with {
            tokens.push(quote! {
                ambiguous_with = #ambiguous_with
            });
        }
        if let Some(true) = self.ambiguous_with_all {
            tokens.push(quote! {
                .ambiguous_with_all()
            });
        }
        if let Some(before_ignore_deferred) = &self.before_ignore_deferred {
            tokens.push(quote! {
                before_ignore_deferred = #before_ignore_deferred
            });
        }
        if let Some(after_ignore_deferred) = &self.after_ignore_deferred {
            tokens.push(quote! {
                after_ignore_deferred = #after_ignore_deferred
            });
        }
        tokens
    }
}

impl ToTokens for ScheduleConfigArgs {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        if let Some(in_set) = &self.in_set {
            tokens.extend(quote! {
                .in_set(#in_set)
            });
        }
        if let Some(before) = &self.before {
            tokens.extend(quote! {
                .before(#before)
            });
        }
        if let Some(after) = &self.after {
            tokens.extend(quote! {
                .after(#after)
            });
        }
        if let Some(run_if) = &self.run_if {
            tokens.extend(quote! {
                .run_if(#run_if)
            });
        }
        if let Some(distributive_run_if) = &self.distributive_run_if {
            tokens.extend(quote! {
                .distributive_run_if(#distributive_run_if)
            });
        }
        if let Some(ambiguous_with) = &self.ambiguous_with {
            tokens.extend(quote! {
                .ambiguous_with(#ambiguous_with)
            });
        }
        if let Some(true) = self.ambiguous_with_all {
            tokens.extend(quote! {
                .ambiguous_with_all()
            });
        }
        if let Some(before_ignore_deferred) = &self.before_ignore_deferred {
            tokens.extend(quote! {
                .before_ignore_deferred(#before_ignore_deferred)
            });
        }
        if let Some(after_ignore_deferred) = &self.after_ignore_deferred {
            tokens.extend(quote! {
                .after_ignore_deferred(#after_ignore_deferred)
            });
        }
    }
}
