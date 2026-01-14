mod inflate;

use crate::{
    macro_api::{
        prelude::*,
        schedule_config::{
            ScheduleConfigArgs,
            ScheduleWithScheduleConfigArgs,
        },
    },
    syntax::ast::flag::Flag,
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
use syn::Attribute;

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

impl EmitAppMutationTokens for ConfigureSystemSetAppMutEmitter {
    fn post_process_inner_item(&mut self) -> Result<(), (InputItem, syn::Error)> {
        let input_item = &mut self.args.input_item;
        let args = &mut self.args.args.base;
        if args.inner.is_none() {
            let (maybe_scrubbed_input_item, inflated_args) =
                inflate::inflate_args_from_input_item(args.clone(), input_item)?;
            *args = inflated_args;
            *input_item = maybe_scrubbed_input_item;
        }
        Ok(())
    }
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let args = self.args.args.base.clone();
        // checks if we need to inflate args
        let inflated_args = if args.inner.is_none() {
            let args = args.clone();
            let input_item = &self.args.input_item;
            match inflate::inflate_args_from_input_item(args, input_item) {
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
        let concrete_paths = match self.args.concrete_paths() {
            Ok(paths) => paths,
            Err(err) => {
                tokens.extend(err.to_compile_error());
                return;
            }
        };
        for concrete_path in concrete_paths {
            tokens.extend(inflate::output(
                &inflated_args,
                app_param,
                &concrete_path,
                !generics.is_empty(),
            ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        codegen::emit::EmitResultExt,
        macro_api::attributes::actions::auto_configure_system_set::inflate::args_from_attr_input,
    };
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
        let (_, inflated_args) = args_from_attr_input(attr, input.clone()).strip_err_context()?;
        Ok((ident, inflated_args))
    }

    mod test_struct {
        use super::*;
        use crate::macro_api::attributes::actions::auto_configure_system_set::inflate::output;
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
        use crate::macro_api::attributes::actions::auto_configure_system_set::inflate::{
            check_strip_helpers,
            output,
        };
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
            .strip_err_context()?;
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
            let (scrubbed_input, _) = args_from_attr_input(attr, input).strip_err_context()?;
            assert_ts_eq!(
                scrubbed_input,
                quote! {
                    enum Foo {
                        A,
                        B,
                    }
                }
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
            let mut input_item =
                InputItem::from_ts_validated(input.clone()).expect("should be valid item");
            let item = input_item.ensure_ast().expect("should be valid item ast");
            assert!(!check_strip_helpers(item));
            let (tokens, _) =
                args_from_attr_input(attr, input.clone()).map_err_context(token_string)?;

            assert_ts_eq!(tokens, input);

            Ok(())
        }
    }
}
