use crate::type_list::TypeList;
use crate::util::{PathExt, path_to_string_with_spaces};
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::{Ident, TokenStream as MacroStream, TokenStream};
use quote::quote;
use syn::{Attribute, Generics, Path, Type, Visibility, parse2};

#[allow(dead_code)]
#[derive(Debug, FromField)]
pub struct FieldData {
    ident: Option<Ident>,
    ty: Type,
}

#[allow(dead_code)]
#[derive(Debug, FromVariant)]
pub struct VariantData {
    ident: Ident,
    fields: darling::ast::Fields<FieldData>,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(auto_plugin), forward_attrs, supports(struct_any, enum_any))]
pub struct GlobalAutoPluginDeriveArgs {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: darling::ast::Data<VariantData, FieldData>,
    pub attrs: Vec<Attribute>,
    #[darling(flatten)]
    pub auto_plugin: GlobalAutoPluginStructOrEnumAttributeArgs,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginStructOrEnumAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub impl_plugin_trait: bool,
    pub impl_generic_auto_plugin_trait: bool,
    pub impl_generic_plugin_trait: bool,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginFnAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

#[derive(FromMeta, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalStructOrEnumAttributeArgs {
    pub plugin: Path,
    #[darling(flatten, default)]
    pub inner: StructOrEnumAttributeArgs,
}

impl GlobalStructOrEnumAttributeArgs {
    pub fn has_generics(&self) -> bool {
        self.inner.has_generics()
    }
    fn concat_ident_hash(&self, ident: &Ident) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn get_unique_ident_string(&self, prefix: &'static str, ident: &Ident) -> String {
        let hash = self.concat_ident_hash(ident);
        format!("{prefix}_{hash}")
    }

    pub fn get_unique_ident(&self, prefix: &'static str, ident: &Ident) -> Ident {
        let ident_string = self.get_unique_ident_string(prefix, ident);
        Ident::new(&ident_string, ident.span())
    }
}

#[derive(FromMeta, Debug, Default, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StructOrEnumAttributeArgs {
    // TODO: #[darling(multiple)]
    //     pub generics: Vec<TypeList>,
    pub generics: Option<TypeList>,
}

impl StructOrEnumAttributeArgs {
    pub fn has_generics(&self) -> bool {
        self.generics
            .as_ref()
            .map(|types| !types.0.is_empty())
            .unwrap_or(false)
    }
}

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct ScheduleConfigArgs {
    pub schedule: Path,
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

#[derive(FromMeta, Debug)]
#[darling(derive_syn_parse)]
pub struct AddSystemArgs {
    pub generics: Option<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleConfigArgs,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ScheduleConfigSerializedArgs {
    pub schedule_path_string: String,
    pub scheduled_item_path_string: String,
    pub in_set_path_string: Option<String>,
    pub before_path_string: Option<String>,
    pub after_path_string: Option<String>,
    pub run_if_path_string: Option<String>,
    pub distributive_run_if_path_string: Option<String>,
    pub ambiguous_with_path_string: Option<String>,
    pub ambiguous_with_all_flag: Option<bool>,
    pub after_ignore_deferred_path_string: Option<String>,
    pub before_ignore_deferred_path_string: Option<String>,
}

impl ScheduleConfigSerializedArgs {
    pub fn from_macro_attr(system: &Path, attr: &ScheduleConfigArgs) -> Self {
        Self {
            schedule_path_string: path_to_string_with_spaces(&attr.schedule),
            scheduled_item_path_string: path_to_string_with_spaces(system),
            in_set_path_string: attr.in_set.as_ref().map(path_to_string_with_spaces),
            before_path_string: attr.before.as_ref().map(path_to_string_with_spaces),
            after_path_string: attr.after.as_ref().map(path_to_string_with_spaces),
            run_if_path_string: attr.run_if.as_ref().map(path_to_string_with_spaces),
            distributive_run_if_path_string: attr
                .distributive_run_if
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_path_string: attr
                .ambiguous_with
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_all_flag: attr.ambiguous_with_all,
            before_ignore_deferred_path_string: attr
                .before_ignore_deferred
                .as_ref()
                .map(path_to_string_with_spaces),
            after_ignore_deferred_path_string: attr
                .after_ignore_deferred
                .as_ref()
                .map(path_to_string_with_spaces),
        }
    }
    pub fn to_tokens(&self) -> syn::Result<MacroStream> {
        let mut output = quote! {};
        if let Some(in_set) = &self.in_set_path_string {
            let in_set = syn::parse_str::<Path>(in_set)?;
            output = quote! {
                #output
                    .in_set(#in_set)
            };
        }
        if let Some(before) = &self.before_path_string {
            let before = syn::parse_str::<Path>(before)?;
            output = quote! {
                #output
                    .before(#before)
            };
        }
        if let Some(after) = &self.after_path_string {
            let after = syn::parse_str::<Path>(after)?;
            output = quote! {
                #output
                    .after(#after)
            }
        }
        if let Some(run_if) = &self.run_if_path_string {
            let run_if = syn::parse_str::<Path>(run_if)?;
            output = quote! {
                #output
                    .run_if(#run_if)
            }
        }
        if let Some(distributive_run_if) = &self.distributive_run_if_path_string {
            let distributive_run_if = syn::parse_str::<Path>(distributive_run_if)?;
            output = quote! {
                #output
                    .distributive_run_if(#distributive_run_if)
            }
        }
        if let Some(ambiguous_with) = &self.ambiguous_with_path_string {
            let ambiguous_with = syn::parse_str::<Path>(ambiguous_with)?;
            output = quote! {
                #output
                    .ambiguous_with(#ambiguous_with)
            }
        }
        if let Some(true) = self.ambiguous_with_all_flag {
            output = quote! {
                #output
                    .ambiguous_with_all()
            }
        }
        if let Some(before_ignore_deferred) = &self.before_ignore_deferred_path_string {
            let before_ignore_deferred = syn::parse_str::<Path>(before_ignore_deferred)?;
            output = quote! {
                #output
                    .before_ignore_deferred(#before_ignore_deferred)
            }
        }
        if let Some(after_ignore_deferred) = &self.after_ignore_deferred_path_string {
            let after_ignore_deferred = syn::parse_str::<Path>(after_ignore_deferred)?;
            output = quote! {
                #output
                    .after_ignore_deferred(#after_ignore_deferred)
            }
        }
        Ok(output)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AddSystemSerializedArgs {
    pub schedule_config: ScheduleConfigSerializedArgs,
}

impl AddSystemSerializedArgs {
    pub fn from_macro_attr(system: &Path, attr: &AddSystemArgs) -> Self {
        let system_path_tokens = if system.has_generics() {
            quote! { #system }
        } else {
            let generics = attr
                .generics
                .as_ref()
                .map(TokenStream::from)
                .unwrap_or_default();
            quote! { #system::<#generics> }
        };
        // TODO: return result
        let system = parse2::<Path>(system_path_tokens).expect("failed to parse system path");

        Self {
            schedule_config: ScheduleConfigSerializedArgs::from_macro_attr(
                &system,
                &attr.schedule_config,
            ),
        }
    }
    pub fn to_tokens(&self, app_ident: &Ident) -> syn::Result<MacroStream> {
        let config_tokens = self.schedule_config.to_tokens()?;
        let schedule = syn::parse_str::<Path>(&self.schedule_config.schedule_path_string)?;
        let system = syn::parse_str::<Path>(&self.schedule_config.scheduled_item_path_string)?;
        Ok(quote! {
            #app_ident.add_systems(#schedule, #system #config_tokens);
        })
    }
}
