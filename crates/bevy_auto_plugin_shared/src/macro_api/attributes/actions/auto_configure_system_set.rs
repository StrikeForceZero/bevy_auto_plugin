use crate::{
    codegen::emit::{
        EmitBuilder,
        EmitErrOnlyResult,
        EmitResult,
        EmitResultExt,
        WithTs,
    },
    macro_api::{
        prelude::*,
        schedule_config::{
            ScheduleConfigArgs,
            ScheduleWithScheduleConfigArgs,
        },
    },
    syntax::{
        ast::flag::Flag,
        extensions::item::ItemAttrsExt,
        parse::{
            item::item_has_attr,
            scrub_helpers::{
                AttrSite,
                ScrubOutcome,
                scrub_helpers_and_ident_with_filter,
            },
        },
    },
};
use darling::FromMeta;
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};
use std::collections::HashMap;
use syn::{
    Attribute,
    Item,
    Path,
    parse_quote,
    spanned::Spanned,
};

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
pub type ConfigureSystemSetAppMutEmitter = AppMutationEmitter<IaConfigureSystemSet>;
pub type ConfigureSystemSetAttrEmitter = AttrEmitter<IaConfigureSystemSet>;

fn output(
    args: &ConfigureSystemSetArgs,
    app_param: &Ident,
    concrete_path: &Path,
    has_generics: bool,
) -> TokenStream {
    let mut tokens = TokenStream::new();
    let schedule = &args.schedule_config.schedule;
    let config_tokens = args.schedule_config.config.to_token_stream();
    if let Some(inner) = &args.inner {
        // enum
        let chained = if args.chain.is_present() {
            quote! { .chain() }
        } else if args.chain_ignore_deferred.is_present() {
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
        if has_generics {
            // TODO: generics are kind of silly here
            //  but if someone does use them we'll assume its just a marker type
            //  that can be initialized via `Default::default()`
            tokens.extend(quote! {
                #app_param.configure_sets(#schedule, #concrete_path::default() #config_tokens);
            });
        } else {
            tokens.extend(quote! {
                #app_param.configure_sets(#schedule, #concrete_path #config_tokens);
            });
        }
    }
    tokens
}

impl EmitAppMutationTokens for ConfigureSystemSetAppMutEmitter {
    fn item_post_process(&mut self) -> Result<(), (TokenStream, syn::Error)> {
        let input_item = &mut self.args.input_item;
        let args = &mut self.args.args.base;
        let input = input_item.to_token_stream();
        let (input, item) = input_item.ensure_ast().with_ts(input)?;
        check_strip_helpers(item, args).with_ts(input.clone())?;
        if args.inner.is_none() {
            let (input, inflated_args) = inflate_args_from_input(args.clone(), input)?;
            *args = inflated_args;
            *input_item = InputItem::Tokens(input);
        }
        Ok(())
    }
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let args = self.args.args.base.clone();
        // checks if we need to inflate args
        let inflated_args = if args.inner.is_none() {
            let args = args.clone();
            let input = self.args.input_item.to_token_stream();
            match inflate_args_from_input(args, input) {
                Ok((_, args)) => args,
                Err((_, err)) => {
                    tokens.extend(err.to_compile_error());
                    return;
                }
            }
        } else {
            args
        };
        let generics = self.args.args.generics();
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(output(&inflated_args, app_param, &concrete_path, !generics.is_empty()));
        }
    }
}

impl ToTokens for ConfigureSystemSetAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut args = self.args.args.extra_args();
        args.extend(self.args.args.base.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
        todo!("not implemented");
        // TODO: would need to modify item to inject helper attributes
    }
}

/// HACK - if the item doesn't have anymore `auto_configure_system_set` - set a flag to strip out the helper attributes
fn check_strip_helpers(item: &Item, args: &mut ConfigureSystemSetArgs) -> syn::Result<()> {
    if !item_has_attr(item, &parse_quote!(auto_configure_system_set))? {
        args._strip_helpers = true;
    }
    Ok(())
}

#[cfg(test)]
pub fn args_from_attr_input(
    attr: TokenStream,
    input: TokenStream,
) -> EmitResult<ConfigureSystemSetArgs, syn::Error> {
    let (input, mut args) = syn::parse2::<ConfigureSystemSetArgs>(attr).with_ts(input.clone())?;
    let mut input_item = InputItem::Tokens(input.clone());
    let (input, item) = input_item.ensure_ast().with_ts(input)?;
    check_strip_helpers(item, &mut args).with_ts(input)?;
    inflate_args_from_input(args, input_item.to_token_stream())
}

/// Type alias for the per-variant configuration data structure
type VariantConfigMap = HashMap<Ident, PerVariant>;

/// Type alias for tracking observed order of variants
type ObservedOrderMap = HashMap<Ident, usize>;

/// Holds configuration data for a specific enum variant
#[derive(Default)]
struct PerVariant {
    /// Default configuration (when group == None)
    default: Option<ConfigureSystemSetArgsInnerEntry>,
    /// Group-specific configurations (when group == Some(g))
    per_group: HashMap<Ident, ConfigureSystemSetArgsInnerEntry>,
}

/// Processes an input TokenStream and constructs ConfigureSystemSetArgs
pub fn inflate_args_from_input(
    mut args: ConfigureSystemSetArgs,
    input: TokenStream,
) -> EmitResult<ConfigureSystemSetArgs, syn::Error> {
    let mut builder = EmitBuilder::from_checkpoint(input);

    // Process the input and scrub helper attributes
    let scrubbed_item = process_helper_attributes(&mut builder, args._strip_helpers)?;

    // Handle based on item type
    match &scrubbed_item.item {
        Item::Struct(_) => {
            return builder.into_ok(args);
        }
        Item::Enum(item_enum) => {
            // Process enum variants to populate args
            process_enum_variants(&mut builder, &mut args, item_enum, &scrubbed_item)?;
        }
        _ => {
            let err = syn::Error::new(scrubbed_item.item.span(), "Only struct or enum supported");
            return builder.into_err(err);
        }
    }

    Ok(args).with_ts(builder.into_tokens())
}

/// Scrubs helper attributes from input and prepares for processing
fn process_helper_attributes(
    builder: &mut EmitBuilder,
    strip_helpers: bool,
) -> Result<ScrubOutcome, (TokenStream, syn::Error)> {
    fn resolve_ident(item: &Item) -> syn::Result<&Ident> {
        // TODO: remove and use ident from higher level?
        item.ident()
    }

    fn is_allowed_helper(site: &AttrSite, attr: &Attribute) -> bool {
        is_config_helper(attr) && matches!(site, AttrSite::Variant { .. })
    }

    // TODO: maybe we need a code path that doesn't strip - only analyze?
    let scrub = scrub_helpers_and_ident_with_filter(
        builder.to_token_stream(),
        is_allowed_helper,
        is_config_helper,
        resolve_ident,
    )
    // TODO: this feels bad - see `ScrubOutcome#ident` todo
    .strip_ok_tokens()?;

    builder.try_unit(|b| {
        if strip_helpers {
            // Always write back scrubbed item to prevent helpers from re-triggering
            scrub.replace_token_stream(b)
        } else {
            // Only prepend errors if any, otherwise keep helpers for next attribute
            scrub.prepend_errors(b)
        }
    })?;

    Ok(scrub)
}

/// Processes enum variants to extract and organize configuration entries
fn process_enum_variants(
    builder: &mut EmitBuilder,
    args: &mut ConfigureSystemSetArgs,
    item_enum: &syn::ItemEnum,
    scrubbed_item: &ScrubOutcome,
) -> EmitErrOnlyResult<(), syn::Error> {
    // Parse and collect configuration data from variant attributes
    let (variant_configs, observed_order) = collect_variant_configs(builder, scrubbed_item)?;

    // Create entries based on variant configs and apply fallback rules
    let entries = create_variant_entries(item_enum, &variant_configs, &observed_order, &args.group);

    // Store processed entries in args
    args.inner = Some(ConfigureSystemSetArgsInner { entries });
    Ok(())
}

/// Collects configuration data from variant attributes
fn collect_variant_configs(
    builder: &mut EmitBuilder,
    scrubbed_item: &ScrubOutcome,
) -> EmitErrOnlyResult<(VariantConfigMap, ObservedOrderMap), syn::Error> {
    let mut variants_cfg: VariantConfigMap = HashMap::new();
    let mut observed_order_by_variant: ObservedOrderMap = HashMap::new();

    builder.try_unit(|_| {
        for (observed_index, site) in scrubbed_item.all_with_removed_attrs().into_iter().enumerate()
        {
            if let AttrSite::Variant { variant } = &site.site {
                observed_order_by_variant.entry(variant.clone()).or_insert(observed_index);
                process_variant_attributes(
                    variant,
                    &site.attrs,
                    observed_index,
                    &mut variants_cfg,
                )?;
            }
        }
        Ok(())
    })?;

    Ok((variants_cfg, observed_order_by_variant))
}

/// Processes attributes for a specific variant
fn process_variant_attributes(
    variant: &Ident,
    attrs: &[Attribute],
    observed_index: usize,
    variants_cfg: &mut VariantConfigMap,
) -> syn::Result<()> {
    for attr in attrs {
        // Skip non-config helpers
        if !is_config_helper(attr) {
            continue;
        }

        // Parse entry from attribute metadata
        let mut entry = ConfigureSystemSetArgsInnerEntry::from_meta(&attr.meta)?;

        // Set default order if not specified
        if entry.order.is_none() {
            entry.order = Some(observed_index);
        }

        let bucket = variants_cfg.entry(variant.clone()).or_default();

        // Store entry based on group
        match &entry.group {
            Some(g) => {
                if bucket.per_group.contains_key(g) {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("duplicate helper for variant `{variant}` and group `{g}`"),
                    ));
                }
                bucket.per_group.insert(g.clone(), entry);
            }
            None => {
                if bucket.default.is_some() {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("duplicate default (no-group) helper for variant `{variant}`"),
                    ));
                }
                bucket.default = Some(entry);
            }
        }
    }

    Ok(())
}

/// Creates entries for each variant based on configs and fallback rules
fn create_variant_entries(
    item_enum: &syn::ItemEnum,
    variants_cfg: &VariantConfigMap,
    observed_order: &ObservedOrderMap,
    outer_group: &Option<Ident>,
) -> ConfigureSystemSetArgsInnerEntries {
    let mut entries = Vec::with_capacity(item_enum.variants.len());
    let mut next_fallback_index = observed_order.len();

    for variant in &item_enum.variants {
        let variant_ident = variant.ident.clone();

        // Find or create observed order for this variant
        let observed_index = observed_order.get(&variant_ident).copied().unwrap_or_else(|| {
            let idx = next_fallback_index;
            next_fallback_index += 1;
            idx
        });

        // Apply fallback rules to select entry
        let entry =
            select_entry_with_fallback(&variant_ident, variants_cfg, observed_index, outer_group);

        entries.push((variant_ident, entry));
    }

    // Sort by order and filter by group
    sort_and_filter_entries(entries, outer_group)
}

/// Selects the appropriate entry for a variant based on fallback rules
fn select_entry_with_fallback(
    variant_ident: &Ident,
    variants_cfg: &VariantConfigMap,
    observed_index: usize,
    outer_group: &Option<Ident>,
) -> ConfigureSystemSetArgsInnerEntry {
    let bucket = variants_cfg.get(variant_ident);

    // First try: explicit group match
    if let (Some(g), Some(b)) = (outer_group, bucket) {
        if let Some(found) = b.per_group.get(g) {
            return found.clone();
        }
    }

    // Second try: default entry with group override
    if let Some(b) = bucket {
        if let Some(mut default_entry) = b.default.clone() {
            default_entry.group = outer_group.clone();
            if default_entry.order.is_none() {
                default_entry.order = Some(observed_index);
            }
            return default_entry;
        }
    }

    // Fallback: synthesize default entry
    ConfigureSystemSetArgsInnerEntry {
        group: outer_group.clone(),
        order: Some(observed_index),
        ..Default::default()
    }
}

/// Sorts entries by order and filters by group
fn sort_and_filter_entries(
    mut entries: ConfigureSystemSetArgsInnerEntries,
    outer_group: &Option<Ident>,
) -> ConfigureSystemSetArgsInnerEntries {
    // Sort by order
    entries.sort_by_key(|(_, entry)| entry.order.unwrap_or_default());

    // Filter by group
    entries.retain(|(_, entry)| {
        match (&entry.group, outer_group) {
            (Some(g), Some(og)) => g == og,
            // If either side is None, keep it (acts as "applies to any")
            _ => true,
        }
    });

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::emit::EmitResultExt;
    use internal_test_proc_macro::xtest;
    use syn::{
        Path,
        parse_quote,
        parse2,
    };

    fn ident_and_args_from_attr_input(
        attr: TokenStream,
        input: TokenStream,
    ) -> Result<(Ident, ConfigureSystemSetArgs), syn::Error> {
        let mut input_item = InputItem::Tokens(input.clone());
        let ident = input_item.ident()?.clone();
        let (_, inflated_args) = args_from_attr_input(attr, input.clone()).strip_err_tokens()?;
        Ok((ident, inflated_args))
    }

    mod test_struct {
        use super::*;
        use quote::quote;
        #[xtest]
        fn test_to_tokens_no_generics() -> syn::Result<()> {
            let args = parse2::<ConfigureSystemSetArgs>(quote!(schedule = Update))?;
            let path: Path = parse_quote!(FooTarget);
            let app_param = parse_quote!(app);
            let tokens = output(&args, &app_param, &path, false);
            assert_eq!(
                tokens.to_string(),
                quote! {
                   #app_param . configure_sets (Update , FooTarget);
                }
                .to_string()
            );
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_single() -> syn::Result<()> {
            let args = parse2::<ConfigureSystemSetArgs>(quote!(schedule = Update))?;
            let app_param = parse_quote!(app);
            let tokens = output(&args, &app_param, &parse_quote!(FooTarget::<u8, bool>), true);
            assert_eq!(
                tokens.to_string(),
                quote! {
                    #app_param . configure_sets (Update , FooTarget :: <u8, bool > ::default() );
                }
                .to_string()
            );
            Ok(())
        }

        #[xtest]
        fn test_to_tokens_multiple() -> syn::Result<()> {
            let args = parse2::<ConfigureSystemSetArgs>(quote!(schedule = Update))?;
            let app_param = parse_quote!(app);
            let tokens = output(&args, &app_param, &parse_quote!(FooTarget::<u8, bool>), true);
            assert_eq!(
                tokens.to_string(),
                quote! {
                    #app_param . configure_sets (Update , FooTarget :: <u8, bool >::default() );
                }
                .to_string()
            );
            let tokens = output(&args, &app_param, &parse_quote!(FooTarget::<bool, bool>), true);
            assert_eq!(
                tokens.to_string(),
                quote! {
                    #app_param . configure_sets (Update , FooTarget :: <bool, bool >::default() );
                }
                .to_string()
            );
            Ok(())
        }
    }

    mod test_enum {
        use super::*;
        use internal_test_util::{
            assert_ts_eq,
            token_stream::token_string,
        };
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
            let app_param = parse_quote!(app);
            let output = output(&args, &app_param, &(ident.into()), false);
            assert_eq!(
                output.to_string(),
                quote! {
                    #app_param . configure_sets (Update , ( Foo::A , Foo::B ));
                }
                .to_string()
            );
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
            let app_param = parse_quote!(app);
            let tokens = output(&args, &app_param, &(ident.into()), false);
            assert_eq!(
                tokens.to_string(),
                quote! {
                    #app_param . configure_sets (Update , ( Foo::A , Foo::B ));
                }
                .to_string()
            );
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
            let app_param = parse_quote!(app);
            let tokens = output(&args, &app_param, &(ident.into()), false);
            assert_eq!(
                tokens.to_string(),
                quote! {
                    #app_param . configure_sets (Update , ( Foo::A , Foo::B ));
                }
                .to_string()
            );
            Ok(())
        }

        // TODO: more tests

        #[xtest]
        fn test_helper() -> syn::Result<()> {
            let (_, inflated_args) = args_from_attr_input(
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
            )
            .strip_err_tokens()?;
            assert_eq!(
                inflated_args,
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
                    chain: Flag::from(false),
                    chain_ignore_deferred: Flag::from(false),
                    _strip_helpers: true,
                }
            );
            Ok(())
        }

        #[xtest]
        fn test_helper_removed_from_ts() -> syn::Result<()> {
            let attr = quote! {
                group = A,
                schedule = Update,
            };
            let input = quote! {
                enum Foo {
                    #[auto_configure_system_set_config(group = A)]
                    A,
                    #[auto_configure_system_set_config(group = A)]
                    B,
                }
            };
            let (scrubbed_input, _) = args_from_attr_input(attr, input).strip_err_tokens()?;
            assert_eq!(
                scrubbed_input.to_string(),
                quote! {
                    enum Foo {
                        A,
                        B,
                    }
                }
                .to_string()
            );
            Ok(())
        }

        #[xtest]
        fn test_conflict_outer() {
            let res = ident_and_args_from_attr_input(
                quote! {
                    schedule = Update,
                    chain, chain_ignore_deferred
                },
                quote! {
                    enum Foo {
                        A,
                    }
                },
            )
            .map_err(|e| e.to_string());

            assert_eq!(res, Err(CHAIN_CONFLICT_ERR.into()));
        }

        #[xtest]
        fn test_conflict_entries() {
            let res = ident_and_args_from_attr_input(
                quote! {
                    schedule = Update,
                },
                quote! {
                    enum Foo {
                        #[auto_configure_system_set_config(chain, chain_ignore_deferred)]
                        A,
                    }
                },
            )
            .map_err(|e| e.to_string());

            assert_eq!(res, Err(CHAIN_CONFLICT_ERR.into()));
        }

        #[xtest]
        fn test_dont_strip_helpers_early() -> Result<(), (String, syn::Error)> {
            let attr = quote! { group = A, schedule = Update };
            let input = quote! {
                #[auto_configure_system_set(group = B, schedule = FixedUpdate)]
                enum Foo {
                    #[auto_configure_system_set_config(group = A, config(run_if = always))]
                    #[auto_configure_system_set_config(group = B, config(run_if = never))]
                    A,
                }
            };
            let (tokens, args) =
                args_from_attr_input(attr, input.clone()).map_err_tokens(token_string)?;

            assert!(!args._strip_helpers);
            assert_ts_eq!(tokens, input);

            Ok(())
        }
    }
}
