use crate::__private::expr_path_or_call::ExprPathOrCall;
use darling::FromMeta;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Clone, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct ScheduleWithScheduleConfigArgs {
    pub schedule: ExprPathOrCall,
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
    #[darling(multiple)]
    pub in_set: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub before: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub after: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub run_if: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub distributive_run_if: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub ambiguous_with: Vec<ExprPathOrCall>,
    pub ambiguous_with_all: Option<bool>,
    #[darling(multiple)]
    pub after_ignore_deferred: Vec<ExprPathOrCall>,
    #[darling(multiple)]
    pub before_ignore_deferred: Vec<ExprPathOrCall>,
}

impl ScheduleConfigArgs {
    pub fn to_inner_arg_tokens_vec(&self) -> Vec<MacroStream> {
        let mut tokens = vec![];
        for in_set in &self.in_set {
            tokens.push(quote! {
                in_set = #in_set
            });
        }
        for before in &self.before {
            tokens.push(quote! {
                before = #before
            });
        }
        for after in &self.after {
            tokens.push(quote! {
                after = #after
            });
        }
        for run_if in &self.run_if {
            tokens.push(quote! {
                run_if = #run_if
            });
        }
        for distributive_run_if in &self.distributive_run_if {
            tokens.push(quote! {
                distributive_run_if = #distributive_run_if
            });
        }
        for ambiguous_with in &self.ambiguous_with {
            tokens.push(quote! {
                ambiguous_with = #ambiguous_with
            });
        }
        if let Some(true) = self.ambiguous_with_all {
            tokens.push(quote! {
                .ambiguous_with_all()
            });
        }
        for before_ignore_deferred in &self.before_ignore_deferred {
            tokens.push(quote! {
                before_ignore_deferred = #before_ignore_deferred
            });
        }
        for after_ignore_deferred in &self.after_ignore_deferred {
            tokens.push(quote! {
                after_ignore_deferred = #after_ignore_deferred
            });
        }
        tokens
    }
}

impl ToTokens for ScheduleConfigArgs {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        for in_set in &self.in_set {
            tokens.extend(quote! {
                .in_set(#in_set)
            });
        }
        for before in &self.before {
            tokens.extend(quote! {
                .before(#before)
            });
        }
        for after in &self.after {
            tokens.extend(quote! {
                .after(#after)
            });
        }
        for run_if in &self.run_if {
            tokens.extend(quote! {
                .run_if(#run_if)
            });
        }
        for distributive_run_if in &self.distributive_run_if {
            tokens.extend(quote! {
                .distributive_run_if(#distributive_run_if)
            });
        }
        for ambiguous_with in &self.ambiguous_with {
            tokens.extend(quote! {
                .ambiguous_with(#ambiguous_with)
            });
        }
        if let Some(true) = self.ambiguous_with_all {
            tokens.extend(quote! {
                .ambiguous_with_all()
            });
        }
        for before_ignore_deferred in &self.before_ignore_deferred {
            tokens.extend(quote! {
                .before_ignore_deferred(#before_ignore_deferred)
            });
        }
        for after_ignore_deferred in &self.after_ignore_deferred {
            tokens.extend(quote! {
                .after_ignore_deferred(#after_ignore_deferred)
            });
        }
    }
}
