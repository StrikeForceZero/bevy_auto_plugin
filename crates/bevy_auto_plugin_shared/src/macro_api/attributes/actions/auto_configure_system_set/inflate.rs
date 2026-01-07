use super::CONFIG_ATTR_NAME;
use crate::{
    codegen::emit::{
        Ctx,
        EmitResult,
        EmitResultExt,
    },
    macro_api::{
        attributes::actions::auto_configure_system_set,
        input_item::InputItem,
        prelude::{
            ConfigureSystemSetArgs,
            ConfigureSystemSetArgsInner,
            ConfigureSystemSetArgsInnerEntries,
            ConfigureSystemSetArgsInnerEntry,
        },
    },
    syntax::{
        analysis::item::item_has_attr,
        parse::scrub_helpers::{
            AttrSite,
            ScrubOutcome,
            scrub_helpers_with_filter,
        },
    },
};
use darling::FromMeta;
use proc_macro2::{
    Ident,
    Span,
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

pub fn output(
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

pub fn check_strip_helpers(item: &Item) -> bool {
    !item_has_attr(item, &parse_quote!(auto_configure_system_set))
}

#[cfg(test)]
pub fn args_from_attr_input(
    attr: TokenStream,
    input: TokenStream,
) -> EmitResult<InputItem, ConfigureSystemSetArgs, syn::Error> {
    let input_item =
        InputItem::from_ts_validated(input.clone()).map_err(|e| (InputItem::Tokens(input), e))?;
    let (input_item, args) =
        Ctx::start(input_item).and_then(|_, _| syn::parse2::<ConfigureSystemSetArgs>(attr))?;
    inflate_args_from_input_item(args, &input_item)
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
pub fn inflate_args_from_input_item(
    mut args: ConfigureSystemSetArgs,
    input_item: &InputItem,
) -> EmitResult<InputItem, ConfigureSystemSetArgs, syn::Error> {
    Ctx::start(input_item.clone())
        .and_then(|input_item, _| {
            // Process the input helper attributes
            process_helper_attributes(input_item)
        })
        .and_then_ctx_mut(|input_item, scrubbed_outcome| {
            let should_strip_helpers = check_strip_helpers(&scrubbed_outcome.original_item);
            let maybe_scrubbed_input_item_tokens = if should_strip_helpers {
                scrubbed_outcome.to_scrubbed_item_tokens()
            } else {
                scrubbed_outcome.to_original_item_tokens()
            };
            *input_item = InputItem::Tokens(maybe_scrubbed_input_item_tokens);
            // TODO: so this is kind of ugly. we check if we need to re-emit the scrubbed item.
            //  but in doing so we are required to include any errors which breaks syn parsing.
            //  and our context required `InputItem` instead of just a `TokenStream`.
            //  so we check if the scrubbed item has errors and if so break out early.
            match input_item.has_compiler_errors() {
                Ok(has_compiler_errors) => {
                    if has_compiler_errors {
                        return Err(
                            // TODO: we need a ui test to make sure the other errors are still emitted with their spans
                            syn::Error::new(Span::call_site(), format!("invalid {CONFIG_ATTR_NAME}s:")),
                        );
                    }
                }
                Err(err) => {
                    return Err(
                        syn::Error::new(
                            Span::call_site(),
                            format!("bevy_auto_plugin bug - please open an issue with a reproduction case: {err:?}"),
                        ),
                    );
                }
            }
            Ok(scrubbed_outcome)
        })
        .and_then_ctx(|_maybe_scrubbed_input_item, scrub_outcome| {
            // Handle based on item type
            match &scrub_outcome.original_item {
                Item::Enum(item_enum) => {
                    // Process enum variants for the specified group to populate args
                    let inner = process_enum_variants_for_group(
                        args.group.as_ref(),
                        item_enum,
                        &scrub_outcome,
                    )?;
                    args.inner = inner;
                    Ok(args)
                }
                Item::Struct(_) => Ok(args),
                _ => {
                    let err = syn::Error::new(Span::call_site(), "Only struct or enum supported");
                    Err(err)
                }
            }
        })
}

/// Scrubs helper attributes from input and prepares for processing
fn process_helper_attributes(
    input_item: impl AsRef<InputItem>,
) -> Result<ScrubOutcome, syn::Error> {
    fn is_allowed_helper(site: &AttrSite, attr: &Attribute) -> bool {
        auto_configure_system_set::is_config_helper(attr)
            && matches!(site, AttrSite::Variant { .. })
    }

    scrub_helpers_with_filter(
        input_item,
        is_allowed_helper,
        auto_configure_system_set::is_config_helper,
    )
}

/// Processes enum variants to extract and organize configuration entries
fn process_enum_variants_for_group(
    group: Option<&Ident>,
    item_enum: &syn::ItemEnum,
    scrubbed_item: &ScrubOutcome,
) -> Result<Option<ConfigureSystemSetArgsInner>, syn::Error> {
    // Parse and collect configuration data from variant attributes
    let (variant_configs, observed_order) = collect_variant_configs(scrubbed_item)?;

    // Create entries based on variant configs and apply fallback rules
    let entries = create_variant_entries(item_enum, &variant_configs, &observed_order, group);

    Ok(Some(ConfigureSystemSetArgsInner { entries }))
}

/// Collects configuration data from variant attributes
fn collect_variant_configs(
    scrubbed_item: &ScrubOutcome,
) -> Result<(VariantConfigMap, ObservedOrderMap), syn::Error> {
    let mut variants_cfg: VariantConfigMap = HashMap::new();
    let mut observed_order_by_variant: ObservedOrderMap = HashMap::new();

    for (observed_index, site) in scrubbed_item.all_with_removed_attrs().into_iter().enumerate() {
        if let AttrSite::Variant { variant } = &site.site {
            observed_order_by_variant.entry(variant.clone()).or_insert(observed_index);
            process_variant_attributes(variant, &site.attrs, observed_index, &mut variants_cfg)?;
        }
    }

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
        if !auto_configure_system_set::is_config_helper(attr) {
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
    outer_group: Option<&Ident>,
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
    outer_group: Option<&Ident>,
) -> ConfigureSystemSetArgsInnerEntry {
    let bucket = variants_cfg.get(variant_ident);

    // First try: explicit group match
    if let (Some(g), Some(b)) = (outer_group, bucket)
        && let Some(found) = b.per_group.get(g)
    {
        return found.clone();
    }

    // Second try: default entry with group override
    if let Some(b) = bucket
        && let Some(mut default_entry) = b.default.clone()
    {
        default_entry.group = outer_group.cloned();
        if default_entry.order.is_none() {
            default_entry.order = Some(observed_index);
        }
        return default_entry;
    }

    // Fallback: synthesize default entry
    ConfigureSystemSetArgsInnerEntry {
        group: outer_group.cloned(),
        order: Some(observed_index),
        ..Default::default()
    }
}

/// Sorts entries by order and filters by group
fn sort_and_filter_entries(
    mut entries: ConfigureSystemSetArgsInnerEntries,
    outer_group: Option<&Ident>,
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
