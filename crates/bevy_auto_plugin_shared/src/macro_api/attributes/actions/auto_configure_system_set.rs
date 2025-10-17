use crate::codegen::tokens::ArgsBackToTokens;
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::prelude::GenericsArgs;
use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::macro_api::derives::VariantData;
use crate::macro_api::schedule_config::{ScheduleConfigArgs, ScheduleWithScheduleConfigArgs};
use crate::macro_api::with_plugin::WithPlugin;
use crate::syntax::analysis::item::{IdentFromItemResult, resolve_ident_from_struct_or_enum};
use crate::syntax::ast::flag::Flag;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use darling::{FromMeta, FromVariant};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{Item, parse2};

const CONFIG_ATTR_NAME: &str = "auto_configure_system_set_config";
const CHAIN_CONFLICT_ERR: &str = "`chain` and `chain_ignore_deferred` are mutually exclusive";

#[derive(FromMeta, Default, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ConfigureSystemSetArgsInnerEntry {
    /// allows individual groups to be configured
    pub group: Option<Ident>,
    pub exclude: Flag,
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
    #[darling(multiple, default)]
    pub generics: Vec<TypeList>,
    /// allows individual groups to be configured
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

impl ItemAttributeArgs for ConfigureSystemSetArgs {
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_> {
        resolve_ident_from_struct_or_enum(item)
    }
}

impl GenericsArgs for ConfigureSystemSetArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokensWithConcreteTargetPath for ConfigureSystemSetArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    ) {
        let schedule = &self.schedule_config.schedule;
        let config_tokens = self.schedule_config.config.to_token_stream();
        if let Some(inner) = &self.inner {
            // enum
            let chained = if self.chain.is_present() {
                quote! { .chain() }
            } else if self.chain_ignore_deferred.is_present() {
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
                    #target :: #ident #chained #config_tokens
                });
            }
            if !entries.is_empty() {
                tokens.extend(quote! {
                     .configure_sets(#schedule, (#(#entries),*) #chained #config_tokens)
                });
            }
        } else {
            // struct
            if target.generics.is_empty() {
                tokens.extend(quote! {
                    .configure_sets(#schedule, #target #config_tokens)
                });
            } else {
                tokens.extend(quote! {
                    .configure_sets(#schedule, #target::default() #config_tokens)
                });
            }
        }
    }
}

impl ArgsBackToTokens for ConfigureSystemSetArgs {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream) {
        let mut args = vec![];
        if !self.generics().is_empty() {
            args.extend(self.generics().to_attribute_arg_vec_tokens());
        }
        args.extend(self.schedule_config.to_inner_arg_tokens_vec());
        tokens.extend(quote! { #(#args),* });
    }
}

pub fn args_with_plugin_from_attr_input(
    attr: TokenStream,
    // this is the only way we can strip out non-derive based attribute helpers
    input: &mut TokenStream,
) -> syn::Result<WithPlugin<ConfigureSystemSetArgs>> {
    let mut args = parse2::<WithPlugin<ConfigureSystemSetArgs>>(attr)?;
    if let Some(entries) = entries_from_args_input(&args.inner, input)? {
        args.inner.inner = Some(ConfigureSystemSetArgsInner { entries });
    }
    Ok(args)
}

#[cfg(test)]
fn ident_and_args_from_attr_input(
    attr: TokenStream,
    input: &mut TokenStream,
) -> syn::Result<(Ident, ConfigureSystemSetArgs)> {
    let mut args = parse2::<ConfigureSystemSetArgs>(attr)?;
    let (ident, entries) = ident_and_entries_from_args_input(&args, input)?;
    if let Some(entries) = entries {
        args.inner = Some(ConfigureSystemSetArgsInner { entries });
    }
    Ok((ident, args))
}

fn entries_from_args_input(
    args: &ConfigureSystemSetArgs,
    input: &mut TokenStream,
) -> syn::Result<Option<ConfigureSystemSetArgsInnerEntries>> {
    let (_, entries) = ident_and_entries_from_args_input(args, input)?;
    Ok(entries)
}

fn ident_and_entries_from_args_input(
    args: &ConfigureSystemSetArgs,
    input: &mut TokenStream,
) -> syn::Result<(Ident, Option<ConfigureSystemSetArgsInnerEntries>)> {
    let item = parse2::<Item>(input.clone())?;
    let ident = ConfigureSystemSetArgs::resolve_item_ident(&item)?.clone();
    let mut data_enum = match item {
        Item::Struct(_) => return Ok((ident, None)),
        Item::Enum(data_enum) => data_enum,
        _ => {
            unreachable!(
                "expected struct or enum - `ConfigureSystemSetArgs::resolve_item_ident` out of sync"
            );
        }
    };

    let mut variants = Vec::with_capacity(data_enum.variants.len());
    for v in &mut data_enum.variants {
        let attrs = std::mem::take(&mut v.attrs);
        let mut keep = vec![];
        let mut helpers = vec![];
        for attr in attrs {
            if attr.path().is_ident(CONFIG_ATTR_NAME) {
                helpers.push(attr);
            } else {
                keep.push(attr);
            }
        }
        let _ = std::mem::replace(&mut v.attrs, keep);
        variants.push((VariantData::from_variant(v), helpers));
    }
    let mut entries: Vec<(Ident, ConfigureSystemSetArgsInnerEntry)> = Vec::new();
    for (variant_data_res, helpers) in variants {
        // write changes back to input
        *input = quote! { #data_enum };
        let variant_attrs = helpers;
        let variant_data = variant_data_res?;
        let mut entry = ConfigureSystemSetArgsInnerEntry::default();
        if variant_attrs.is_empty() {
            entry.group = args.group.clone();
        } else {
            for variant_attr in variant_attrs {
                let entry_args = ConfigureSystemSetArgsInnerEntry::from_meta(&variant_attr.meta)?;
                entry = entry_args;
            }
        }
        entries.push((variant_data.ident, entry));
    }
    entries.sort_by_key(|(_, entry)| entry.order.unwrap_or_default());
    entries.retain(|(_, entry)| {
        !entry.exclude.is_present() && entry.group.is_none()
            || match (&entry.group, &args.group) {
                (Some(group), Some(args_group)) => {
                    group == args_group && !entry.exclude.is_present()
                }
                _ => true,
            }
    });
    Ok((ident, Some(entries)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::with_target_path::WithTargetPath;
    use internal_test_proc_macro::xtest;
    use syn::{Path, parse_quote, parse2};

    mod test_struct {
        use super::*;
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

        #[xtest]
        fn test_to_tokens_no_generics() -> syn::Result<()> {
            let (ident, args) = ident_and_args_from_attr_input(
                quote!(schedule = Update),
                &mut quote! {
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
                &mut quote! {
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
                &mut quote! {
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

        #[xtest]
        fn test_to_tokens_multiple_exclude() -> syn::Result<()> {
            let (ident, args) = ident_and_args_from_attr_input(
                quote!(schedule = Update),
                &mut quote! {
                    enum Foo {
                        #[auto_configure_system_set_config(group = A)]
                        #[auto_configure_system_set_config(group = B, exclude)]
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
                    . configure_sets (Update , ( Foo::A, Foo::B ))
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
                &mut quote! {
                    enum Foo {
                        #[auto_configure_system_set_config(group = A)]
                        A,
                        #[auto_configure_system_set_config(group = A, exclude)]
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
                        entries: vec![(
                            parse_quote!(A),
                            ConfigureSystemSetArgsInnerEntry {
                                group: parse_quote!(A),
                                ..Default::default()
                            }
                        )]
                    }),
                    group: Some(parse_quote!(A)),
                    generics: vec![],
                    chain: Flag::from(false),
                    chain_ignore_deferred: Flag::from(false),
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
                    #[auto_configure_system_set_config(group = A, exclude)]
                    B,
                }
            };
            let _ = ident_and_args_from_attr_input(
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
            let res = ident_and_args_from_attr_input(
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
            let res = ident_and_args_from_attr_input(
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
