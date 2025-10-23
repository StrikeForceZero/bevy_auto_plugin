use crate::macro_api::prelude::*;
use crate::macro_api::schedule_config::{ScheduleConfigArgs, ScheduleWithScheduleConfigArgs};
use crate::syntax::analysis::item::resolve_ident_from_struct_or_enum;
use crate::syntax::ast::flag::Flag;
use crate::syntax::parse::item::ts_item_has_attr;
use crate::syntax::parse::scrub_helpers::{AttrSite, scrub_helpers_and_ident_with_filter};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Attribute, Item, parse_quote};

const CONFIG_ATTR_NAME: &str = "auto_configure_system_set_config";
const CHAIN_CONFLICT_ERR: &str = "`chain` and `chain_ignore_deferred` are mutually exclusive";

fn is_config_helper(attr: &Attribute) -> bool {
    attr.path().is_ident(CONFIG_ATTR_NAME)
}

#[derive(FromMeta, Default, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ConfigureSystemSetArgsInnerEntry {
    /// allows per schedule entry/variants to be configured
    pub group: Option<Ident>,
    /// order in [`ConfigureSystemSetArgsInner::entries`]
    pub order: Option<usize>,
    /// .chain()
    pub chain: Flag,
    /// .chain_ignore_deferred()
    pub chain_ignore_deferred: Flag,
    #[darling(default)]
    pub config: ScheduleConfigArgs,
}

impl ConfigureSystemSetArgsInnerEntry {
    fn validate(self) -> darling::Result<Self> {
        if self.chain.is_present() && self.chain_ignore_deferred.is_present() {
            let err = darling::Error::custom(
                "`chain` and `chain_ignore_deferred` are mutually exclusive",
            )
            .with_span(&self.chain_ignore_deferred.span());
            Err(err)
        } else {
            Ok(self)
        }
    }
}

pub type ConfigureSystemSetArgsInnerEntries = Vec<(Ident, ConfigureSystemSetArgsInnerEntry)>;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
/// for enums only
pub struct ConfigureSystemSetArgsInner {
    #[darling(skip)]
    pub entries: Vec<(Ident, ConfigureSystemSetArgsInnerEntry)>,
}

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ConfigureSystemSetArgs {
    /// allows per schedule entry/variants to be configured
    pub group: Option<Ident>,
    #[darling(flatten)]
    pub schedule_config: ScheduleWithScheduleConfigArgs,
    /// .chain()
    pub chain: Flag,
    /// .chain_ignore_deferred()
    pub chain_ignore_deferred: Flag,
    #[darling(skip)]
    /// Some when enum, None when struct
    pub inner: Option<ConfigureSystemSetArgsInner>,
    #[darling(skip, default)]
    /// internal - used to track if this is the last attribute in the item to strip helpers
    pub _strip_helpers: bool,
}

impl ConfigureSystemSetArgs {
    fn validate(self) -> darling::Result<Self> {
        if self.chain.is_present() && self.chain_ignore_deferred.is_present() {
            let err = darling::Error::custom(CHAIN_CONFLICT_ERR)
                .with_span(&self.chain_ignore_deferred.span());
            Err(err)
        } else {
            Ok(self)
        }
    }
}

impl AttributeIdent for ConfigureSystemSetArgs {
    const IDENT: &'static str = "auto_configure_system_set";
}

pub type IaConfigureSystemSet = ItemAttribute<
    Composed<ConfigureSystemSetArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QConfigureSystemSet<'a> = Q<'a, IaConfigureSystemSet>;
pub type QQConfigureSystemSet<'a> = QQ<'a, IaConfigureSystemSet>;

impl RequiredUseQTokens for QConfigureSystemSet<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let args = &self.args.args;
        let generics = args.generics();
        let base = &self.args.args.base;
        let schedule = &args.base.schedule_config.schedule;
        let config_tokens = args.base.schedule_config.config.to_token_stream();
        for concrete_path in self.args.concrete_paths() {
            if let Some(inner) = &base.inner {
                // enum
                let chained = if base.chain.is_present() {
                    quote! { .chain() }
                } else if base.chain_ignore_deferred.is_present() {
                    quote! { .chain_ignore_deferred() }
                } else {
                    quote! {}
                };
                let mut entries = vec![];
                for (ident, entry) in inner.entries.iter() {
                    let chained = if entry.chain.is_present() {
                        quote! { .chain() }
                    } else if entry.chain_ignore_deferred.is_present() {
                        quote! { .chain_ignore_deferred() }
                    } else {
                        quote! {}
                    };
                    let config_tokens = entry.config.to_token_stream();
                    entries.push(quote! {
                        #concrete_path :: #ident #chained #config_tokens
                    });
                }
                if !entries.is_empty() {
                    tokens.extend(quote! {
                         #app_param.configure_sets(#schedule, (#(#entries),*) #chained #config_tokens);
                    });
                }
            } else {
                // struct
                if generics.is_empty() {
                    tokens.extend(quote! {
                        #app_param.configure_sets(#schedule, #concrete_path #config_tokens);
                    });
                } else {
                    // TODO: generics are kind of silly here
                    //  but if someone does use them we'll assume its just a marker type
                    //  that can be initialized via `Default::default()`
                    tokens.extend(quote! {
                        #app_param.configure_sets(#schedule, #concrete_path::default() #config_tokens);
                    });
                }
            }
        }
    }
}

impl ToTokens for QQConfigureSystemSet<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        // TODO: cleanup
        args.extend(
            self.args
                .args
                .base
                .schedule_config
                .to_inner_arg_tokens_vec(),
        );
        tokens.extend(quote! {
            #(#args),*
        });
    }
}

/// HACK - if the item doesn't have anymore `auto_configure_system_set` - set a flag to strip out the helper attributes
fn check_strip_helpers(input: TokenStream, args: &mut ConfigureSystemSetArgs) -> syn::Result<()> {
    if !ts_item_has_attr(input, &parse_quote!(auto_configure_system_set))? {
        args._strip_helpers = true;
    }
    Ok(())
}

#[cfg(test)]
pub fn args_from_attr_input(
    attr: TokenStream,
    // this is the only way we can strip out non-derive based attribute helpers
    input: &mut TokenStream,
) -> syn::Result<ConfigureSystemSetArgs> {
    let mut args = syn::parse2::<ConfigureSystemSetArgs>(attr)?;
    check_strip_helpers(input.clone(), &mut args)?;
    args_with_plugin_from_args_input(&mut args, input)?;
    Ok(args)
}

pub fn args_with_plugin_from_args_input(
    args: &mut ConfigureSystemSetArgs,
    // this is the only way we can strip out non-derive based attribute helpers
    input: &mut TokenStream,
) -> syn::Result<()> {
    fn resolve_ident(item: &Item) -> syn::Result<&Ident> {
        resolve_ident_from_struct_or_enum(item)
            .map_err(|err| syn::Error::new(item.span(), format!("failed to resolve ident: {err}")))
    }

    fn is_allowed_helper(site: &AttrSite, attr: &Attribute) -> bool {
        is_config_helper(attr) && matches!(site, AttrSite::Variant { .. })
    }

    // 2) Scrub helpers from the item and resolve ident
    let scrub = scrub_helpers_and_ident_with_filter(
        input.clone(),
        is_allowed_helper,
        is_config_helper,
        resolve_ident,
    )?;

    // 3)
    if args._strip_helpers {
        // Always write back the scrubbed item to *input* so helpers never re-trigger and IDE has something to work with
        scrub.write_back(input)?;
    } else {
        // Check if we have errors to print and if so, strip helpers from the item
        // Otherwise, maintain helpers for the next attribute to process
        scrub.write_if_errors_with_scrubbed_item(input)?;
    }

    // 4) If it's a struct, there are no entries to compute
    let data_enum = match scrub.item {
        Item::Struct(_) => {
            return Ok(());
        }
        Item::Enum(ref en) => en,
        _ => unreachable!("resolve_ident_from_struct_or_enum guarantees struct|enum"),
    };

    // 5) Collect per-variant configs:
    //    - per-group configs: HashMap<Ident, Entry>
    //    - default (no-group) config: Option<Entry>
    use std::collections::HashMap;

    #[derive(Default)]
    struct PerVariant {
        /// group == None
        default: Option<ConfigureSystemSetArgsInnerEntry>,
        /// group == Some(g)
        per_group: HashMap<Ident, ConfigureSystemSetArgsInnerEntry>,
    }

    let mut variants_cfg: HashMap<Ident, PerVariant> = HashMap::new();

    // Require an observed order per variant, based on site enumeration position.
    // Track the FIRST observed index we see for that variant.
    let mut observed_order_by_variant: HashMap<Ident, usize> = HashMap::new();

    // Parse all removed helper attrs (last-one-wins per key), but error on duplicates for the SAME key.
    // Treat a second helper on the same (variant, group or None) as a hard error.
    for (observed_index, site) in scrub.all_with_removed_attrs().into_iter().enumerate() {
        if let AttrSite::Variant { variant } = &site.site {
            observed_order_by_variant
                .entry(variant.clone())
                .or_insert(observed_index);

            for attr in &site.attrs {
                // Only care about our helper
                if !is_config_helper(attr) {
                    continue;
                }

                let mut entry = ConfigureSystemSetArgsInnerEntry::from_meta(&attr.meta)?;

                // If order wasn't provided on the helper, set it to the first observed index for this variant
                if entry.order.is_none() {
                    entry.order = Some(
                        *observed_order_by_variant
                            .get(variant)
                            .unwrap_or(&observed_index),
                    );
                }

                let bucket = variants_cfg.entry(variant.clone()).or_default();
                match &entry.group {
                    Some(g) => {
                        if bucket.per_group.contains_key(g) {
                            return Err(syn::Error::new(
                                attr.span(),
                                format!("duplicate helper for variant `{variant}` and group `{g}`",),
                            ));
                        }
                        bucket.per_group.insert(g.clone(), entry);
                    }
                    None => {
                        if bucket.default.is_some() {
                            return Err(syn::Error::new(
                                attr.span(),
                                format!(
                                    "duplicate default (no-group) helper for variant `{variant}`",
                                ),
                            ));
                        }
                        bucket.default = Some(entry);
                    }
                }
            }
        }
    }

    // 6) Walk the enum variants and assemble entries using fallback rules:
    //    chosen = per_group[outer_group]
    //           || (default with group overwritten to outer_group)
    //           || synthesized default (group = outer_group, order = observed)
    let outer_group = args.group.clone();
    let mut entries: ConfigureSystemSetArgsInnerEntries =
        Vec::with_capacity(data_enum.variants.len());

    for v in &data_enum.variants {
        let v_ident = v.ident.clone();

        let prev_observed_len = observed_order_by_variant.len();
        // Find observed order for this variant (if we never saw the site, use sequential fallback)
        let observed = *observed_order_by_variant
            .entry(v_ident.clone())
            .or_insert_with(|| prev_observed_len);

        let chosen_entry = (|| {
            let bucket = variants_cfg.get(&v_ident);

            // prefer explicit group match
            if let (Some(g), Some(b)) = (&outer_group, bucket)
                && let Some(found) = b.per_group.get(g)
            {
                return Some(found.clone());
            }

            // else use the default helper but override its group to the outer group
            if let Some(b) = bucket
                && let Some(mut def) = b.default.clone()
            {
                def.group = outer_group.clone();
                if def.order.is_none() {
                    def.order = Some(observed);
                }
                return Some(def);
            }

            // else synthesize a default entry
            Some(ConfigureSystemSetArgsInnerEntry {
                group: outer_group.clone(),
                order: Some(observed),
                ..Default::default()
            })
        })()
        .expect("infallible");

        entries.push((v_ident, chosen_entry));
    }

    // 7) Sort & filter
    entries.sort_by_key(|(_, e)| e.order.unwrap_or_default());
    entries.retain(|(_, e)| {
        // same group as outer group
        match (&e.group, &outer_group) {
            (Some(g), Some(og)) => g == og,
            // If either side is None, keep it (acts as "applies to any")
            _ => true,
        }
    });

    // 8) Store into args and return
    args.inner = Some(ConfigureSystemSetArgsInner { entries });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use internal_test_proc_macro::xtest;
    use syn::{Path, parse_quote, parse2};

    fn ident_and_args_from_attr_input(
        attr: TokenStream,
        mut input: TokenStream,
    ) -> Result<(Ident, ConfigureSystemSetArgs), syn::Error> {
        let item = parse2::<Item>(input.clone())?;
        let ident = resolve_ident_from_struct_or_enum(&item).map_err(|err| {
            syn::Error::new(item.span(), format!("failed to resolve ident: {err}"))
        })?;
        args_from_attr_input(attr, &mut input).and_then(|args| Ok((ident.clone(), args)))
    }

    fn ident_and_args_from_attr_mut_input(
        attr: TokenStream,
        input: &mut TokenStream,
    ) -> Result<(Ident, ConfigureSystemSetArgs), syn::Error> {
        let item = parse2::<Item>(input.clone())?;
        let ident = resolve_ident_from_struct_or_enum(&item).map_err(|err| {
            syn::Error::new(item.span(), format!("failed to resolve ident: {err}"))
        })?;
        args_from_attr_input(attr, input).and_then(|args| Ok((ident.clone(), args)))
    }

    mod test_struct {
        use super::*;
        use quote::quote;
        #[xtest]
        fn test_to_tokens_no_generics() -> syn::Result<()> {
            let args = parse2::<ConfigureSystemSetArgs>(quote!(schedule = Update))?;
            let path: Path = parse_quote!(FooTarget);
            let args_with_target = WithTargetPath::try_from((path, args))?;
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , FooTarget)
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_single() -> syn::Result<()> {
            let args =
                parse2::<ConfigureSystemSetArgs>(quote!(schedule = Update, generics(u8, bool)))?;
            let path: Path = parse_quote!(FooTarget);
            let args_with_target = WithTargetPath::try_from((path, args))?;
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , FooTarget :: <u8, bool > ::default() )
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_multiple() -> syn::Result<()> {
            let args = parse2::<ConfigureSystemSetArgs>(quote!(
                schedule = Update,
                generics(u8, bool),
                generics(bool, bool)
            ))?;
            let path: Path = parse_quote!(FooTarget);
            let args_with_target = WithTargetPath::try_from((path, args))?;
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , FooTarget :: <u8, bool >::default() )
                }
                .to_string()
            );
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , FooTarget :: <bool, bool >::default() )
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }
    }

    mod test_enum {
        use super::*;
        use crate::syntax::validated::path_without_generics::PathWithoutGenerics;
        use quote::quote;

        #[xtest]
        fn test_to_tokens_no_generics() -> syn::Result<()> {
            let (ident, args) = ident_and_args_from_attr_input(
                quote!(schedule = Update),
                quote! {
                    enum Foo {
                        A,
                        B,
                    }
                },
            )?;
            let args_with_target =
                WithTargetPath::try_from((PathWithoutGenerics::from(ident), args)).unwrap(); // infallible
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , ( Foo::A , Foo::B ))
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_single() -> syn::Result<()> {
            let (ident, args) = ident_and_args_from_attr_input(
                quote!(schedule = Update),
                quote! {
                    enum Foo {
                        A,
                        B,
                    }
                },
            )?;
            let args_with_target =
                WithTargetPath::try_from((PathWithoutGenerics::from(ident), args)).unwrap(); // infallible
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , ( Foo::A , Foo::B ))
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_multiple() -> syn::Result<()> {
            let (ident, args) = ident_and_args_from_attr_input(
                quote!(schedule = Update),
                quote! {
                    enum Foo {
                        #[auto_configure_system_set_config(group = A)]
                        A,
                        #[auto_configure_system_set_config(group = B)]
                        B,
                    }
                },
            )?;
            let args_with_target =
                WithTargetPath::try_from((PathWithoutGenerics::from(ident), args)).unwrap(); // infallible
            let mut token_iter = args_with_target.to_tokens_iter();
            assert_eq!(
                token_iter.next().expect("token_iter").to_string(),
                quote! {
                    . configure_sets (Update , ( Foo::A , Foo::B ))
                }
                .to_string()
            );
            assert!(token_iter.next().is_none());
            Ok(())
        }

        // TODO: more tests

        #[xtest]
        fn test_helper() -> syn::Result<()> {
            let (_ident, args) = ident_and_args_from_attr_input(
                quote! {
                    group = A,
                    schedule = Update,
                },
                quote! {
                    enum Foo {
                        #[auto_configure_system_set_config(group = A)]
                        A,
                        #[auto_configure_system_set_config(group = A)]
                        B,
                    }
                },
            )?;
            assert_eq!(
                args,
                ConfigureSystemSetArgs {
                    schedule_config: ScheduleWithScheduleConfigArgs {
                        schedule: parse_quote!(Update),
                        config: ScheduleConfigArgs::default(),
                    },
                    inner: Some(ConfigureSystemSetArgsInner {
                        entries: vec![
                            (
                                parse_quote!(A),
                                ConfigureSystemSetArgsInnerEntry {
                                    group: parse_quote!(A),
                                    order: Some(1),
                                    ..Default::default()
                                }
                            ),
                            (
                                parse_quote!(B),
                                ConfigureSystemSetArgsInnerEntry {
                                    group: parse_quote!(A),
                                    order: Some(2),
                                    ..Default::default()
                                }
                            )
                        ]
                    }),
                    group: Some(parse_quote!(A)),
                    generics: vec![],
                    chain: Flag::from(false),
                    chain_ignore_deferred: Flag::from(false),
                    _strip_helpers: true,
                }
            );
            Ok(())
        }

        #[xtest]
        fn test_helper_removed_from_ts() {
            let mut input = quote! {
                enum Foo {
                    #[auto_configure_system_set_config(group = A)]
                    A,
                    #[auto_configure_system_set_config(group = A)]
                    B,
                }
            };
            let _ = ident_and_args_from_attr_mut_input(
                quote! {
                    group = A,
                    schedule = Update,
                },
                &mut input,
            );
            assert_eq!(
                input.to_string(),
                quote! {
                    enum Foo {
                        A,
                        B,
                    }
                }
                .to_string()
            );
        }

        #[xtest]
        fn test_conflict_outer() {
            let mut input = quote! {
                enum Foo {
                    A,
                }
            };
            let res = ident_and_args_from_attr_mut_input(
                quote! {
                    schedule = Update,
                    chain, chain_ignore_deferred
                },
                &mut input,
            )
            .map_err(|e| e.to_string());

            assert_eq!(res, Err(CHAIN_CONFLICT_ERR.into()));
        }

        #[xtest]
        fn test_conflict_entries() {
            let mut input = quote! {
                enum Foo {
                    #[auto_configure_system_set_config(chain, chain_ignore_deferred)]
                    A,
                }
            };
            let res = ident_and_args_from_attr_mut_input(
                quote! {
                    schedule = Update,
                },
                &mut input,
            )
            .map_err(|e| e.to_string());

            assert_eq!(res, Err(CHAIN_CONFLICT_ERR.into()));
        }
    }
}
