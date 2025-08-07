use crate::expr_value::ExprValue;
use crate::item_with_attr_match::ItemWithAttributeMatch;
use crate::type_list::TypeList;
use crate::util::extensions::path::PathExt;
use crate::util::path_fmt::path_to_string_with_spaces;
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::{ToTokens, quote};
use syn::parse::Parse;
use syn::{Attribute, Expr, Generics, Path, Type, Visibility, parse_quote, parse_str};

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
    pub plugin: Option<Ident>,
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(default)]
    pub app_param: Option<Ident>,
}

#[derive(FromMeta, Clone, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalAddSystemArgs {
    pub plugin: Path,
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleConfigArgs,
}

impl GlobalAddSystemArgs {
    pub fn into_add_system_with_target_args(self, target: Path) -> AddSystemWithTargetArgs {
        AddSystemWithTargetArgs {
            schedule_config: ScheduleConfigWithSystemArgs::from_macro_attr(
                target,
                self.schedule_config,
            ),
        }
    }
}

impl From<GlobalAddSystemArgs> for AddSystemArgs {
    fn from(args: GlobalAddSystemArgs) -> Self {
        Self {
            generics: args.generics,
            schedule_config: args.schedule_config,
        }
    }
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
}

#[derive(FromMeta, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalInsertResourceAttributeArgs {
    pub plugin: Path,
    #[darling(flatten)]
    pub inner: InsertResourceArgs,
}

impl GlobalInsertResourceAttributeArgs {
    pub fn has_generics(&self) -> bool {
        self.inner
            .generics
            .as_ref()
            .map(|generics| !generics.is_empty())
            .unwrap_or(false)
    }
}

pub trait GlobalMacroArgs: Parse + std::hash::Hash {
    type Input;
    type ToTokensFn: Fn(&Self, Self::Input) -> syn::Result<MacroStream>;
    fn target_paths_with_ident(&self, ident: &Ident) -> Vec<Path> {
        let output = self
            .generics()
            .iter()
            .map(|generics| parse_quote!( #ident::<#generics> ))
            .collect::<Vec<_>>();

        // failsafe - we need at least one generated
        // TODO: this made it more complex...
        if output.is_empty() {
            vec![parse_quote!( #ident )]
        } else {
            output
        }
    }
    fn generics(&self) -> &[TypeList];
    fn plugin(&self) -> &Path;
    fn to_input(self, ident: &Ident) -> syn::Result<impl Iterator<Item = Self::Input>>;

    fn _concat_ident_hash(&self, ident: &Ident) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn _get_unique_ident_string(&self, prefix: &'static str, ident: &Ident) -> String {
        let hash = self._concat_ident_hash(ident);
        format!("{prefix}_{hash}")
    }

    fn get_unique_ident(&self, prefix: &'static str, ident: &Ident) -> Ident {
        let ident_string = self._get_unique_ident_string(prefix, ident);
        Ident::new(&ident_string, ident.span())
    }
}

impl GlobalMacroArgs for GlobalStructOrEnumAttributeArgs {
    type Input = Path;
    type ToTokensFn = fn(&Self, Self::Input) -> syn::Result<MacroStream>;
    fn generics(&self) -> &[TypeList] {
        &self.inner.generics
    }
    fn plugin(&self) -> &Path {
        &self.plugin
    }
    fn to_input(self, ident: &Ident) -> syn::Result<impl Iterator<Item = Self::Input>> {
        Ok(self.target_paths_with_ident(ident).into_iter())
    }
}

impl GlobalMacroArgs for GlobalInsertResourceAttributeArgs {
    type Input = InsertResourceArgsWithPath;
    type ToTokensFn = fn(&Self, Self::Input) -> syn::Result<MacroStream>;
    fn generics(&self) -> &[TypeList] {
        if let Some(g) = &self.inner.generics {
            std::slice::from_ref(g)
        } else {
            &[]
        }
    }
    fn plugin(&self) -> &Path {
        &self.plugin
    }

    fn to_input(self, ident: &Ident) -> syn::Result<impl Iterator<Item = Self::Input>> {
        let target_paths = self.target_paths_with_ident(ident);
        let mut result_vec = Vec::with_capacity(target_paths.len());

        for target in target_paths {
            let item = InsertResourceArgsWithPath {
                path: target,
                resource_args: self.inner.clone(),
            };
            result_vec.push(item);
        }

        Ok(result_vec.into_iter())
    }
}

impl GlobalMacroArgs for GlobalAddSystemArgs {
    type Input = AddSystemWithTargetArgs;
    type ToTokensFn = fn(&Self, Self::Input) -> syn::Result<MacroStream>;
    fn generics(&self) -> &[TypeList] {
        &self.generics
    }
    fn plugin(&self) -> &Path {
        &self.plugin
    }
    fn to_input(self, ident: &Ident) -> syn::Result<impl Iterator<Item = Self::Input>> {
        let target_paths = self.target_paths_with_ident(ident);
        let mut result_vec = Vec::with_capacity(target_paths.len());

        for target in target_paths {
            let add_system_args = AddSystemArgs::from(self.clone());
            let items = AddSystemWithTargetArgs::try_from_macro_attr(target, add_system_args)?;
            result_vec.extend(items);
        }

        Ok(result_vec.into_iter())
    }
}

#[derive(FromMeta, Debug, Default, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StructOrEnumAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

impl From<TypeList> for StructOrEnumAttributeArgs {
    fn from(value: TypeList) -> Self {
        Self {
            generics: vec![value],
        }
    }
}

impl StructOrEnumAttributeArgs {
    pub fn has_generics(&self) -> bool {
        !self.generics.is_empty()
    }
}

#[derive(FromMeta, Clone, Debug, PartialEq, Hash)]
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
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    #[darling(flatten)]
    pub schedule_config: ScheduleConfigArgs,
}

impl AddSystemArgs {
    pub fn has_generics(&self) -> bool {
        !self.generics.is_empty()
    }
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
    pub fn to_tokens(self) -> syn::Result<MacroStream> {
        ScheduleConfigWithSystemArgs::try_from(self)?.to_tokens()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleConfigWithSystemArgs {
    pub schedule_path: Path,
    pub scheduled_item_path: Path,
    pub in_set_path: Option<Path>,
    pub before_path: Option<Path>,
    pub after_path: Option<Path>,
    pub run_if_path: Option<Path>,
    pub distributive_run_if_path: Option<Path>,
    pub ambiguous_with_path: Option<Path>,
    pub ambiguous_with_all_flag: Option<bool>,
    pub after_ignore_deferred_path: Option<Path>,
    pub before_ignore_deferred_path: Option<Path>,
}

impl ScheduleConfigWithSystemArgs {
    pub fn from_macro_attr(system: Path, attr: ScheduleConfigArgs) -> Self {
        Self {
            schedule_path: attr.schedule,
            scheduled_item_path: system,
            in_set_path: attr.in_set,
            before_path: attr.before,
            after_path: attr.after,
            run_if_path: attr.run_if,
            distributive_run_if_path: attr.distributive_run_if,
            ambiguous_with_path: attr.ambiguous_with,
            ambiguous_with_all_flag: attr.ambiguous_with_all,
            before_ignore_deferred_path: attr.before_ignore_deferred,
            after_ignore_deferred_path: attr.after_ignore_deferred,
        }
    }
    pub fn to_tokens(&self) -> syn::Result<MacroStream> {
        let mut output = quote! {};
        if let Some(in_set) = &self.in_set_path {
            output.extend(quote! {
                .in_set(#in_set)
            });
        }
        if let Some(before) = &self.before_path {
            output.extend(quote! {
                .before(#before)
            });
        }
        if let Some(after) = &self.after_path {
            output.extend(quote! {
                .after(#after)
            });
        }
        if let Some(run_if) = &self.run_if_path {
            output.extend(quote! {
                .run_if(#run_if)
            });
        }
        if let Some(distributive_run_if) = &self.distributive_run_if_path {
            output.extend(quote! {
                .distributive_run_if(#distributive_run_if)
            });
        }
        if let Some(ambiguous_with) = &self.ambiguous_with_path {
            output.extend(quote! {
                .ambiguous_with(#ambiguous_with)
            });
        }
        if let Some(true) = self.ambiguous_with_all_flag {
            output.extend(quote! {
                .ambiguous_with_all()
            });
        }
        if let Some(before_ignore_deferred) = &self.before_ignore_deferred_path {
            output.extend(quote! {
                .before_ignore_deferred(#before_ignore_deferred)
            });
        }
        if let Some(after_ignore_deferred) = &self.after_ignore_deferred_path {
            output.extend(quote! {
                .after_ignore_deferred(#after_ignore_deferred)
            });
        }
        Ok(output)
    }
}

impl TryFrom<ScheduleConfigSerializedArgs> for ScheduleConfigWithSystemArgs {
    type Error = syn::Error;
    fn try_from(value: ScheduleConfigSerializedArgs) -> Result<Self, Self::Error> {
        fn parse(value_str: String) -> syn::Result<Path> {
            parse_str::<Path>(&value_str)
        }
        fn parse_opt(value_str: Option<String>) -> syn::Result<Option<Path>> {
            let Some(value_str) = value_str else {
                return Ok(None);
            };
            Ok(Some(parse(value_str)?))
        }
        Ok(Self {
            schedule_path: parse(value.schedule_path_string)?,
            scheduled_item_path: parse(value.scheduled_item_path_string)?,
            in_set_path: parse_opt(value.in_set_path_string)?,
            before_path: parse_opt(value.before_path_string)?,
            after_path: parse_opt(value.after_path_string)?,
            run_if_path: parse_opt(value.run_if_path_string)?,
            distributive_run_if_path: parse_opt(value.distributive_run_if_path_string)?,
            ambiguous_with_path: parse_opt(value.ambiguous_with_path_string)?,
            ambiguous_with_all_flag: value.ambiguous_with_all_flag,
            after_ignore_deferred_path: parse_opt(value.after_ignore_deferred_path_string)?,
            before_ignore_deferred_path: parse_opt(value.before_ignore_deferred_path_string)?,
        })
    }
}

impl From<ScheduleConfigWithSystemArgs> for ScheduleConfigSerializedArgs {
    fn from(value: ScheduleConfigWithSystemArgs) -> Self {
        Self {
            schedule_path_string: path_to_string_with_spaces(&value.schedule_path),
            scheduled_item_path_string: path_to_string_with_spaces(&value.scheduled_item_path),
            in_set_path_string: value.in_set_path.as_ref().map(path_to_string_with_spaces),
            before_path_string: value.before_path.as_ref().map(path_to_string_with_spaces),
            after_path_string: value.after_path.as_ref().map(path_to_string_with_spaces),
            run_if_path_string: value.run_if_path.as_ref().map(path_to_string_with_spaces),
            distributive_run_if_path_string: value
                .distributive_run_if_path
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_path_string: value
                .ambiguous_with_path
                .as_ref()
                .map(path_to_string_with_spaces),
            ambiguous_with_all_flag: value.ambiguous_with_all_flag,
            after_ignore_deferred_path_string: value
                .after_ignore_deferred_path
                .as_ref()
                .map(path_to_string_with_spaces),
            before_ignore_deferred_path_string: value
                .before_ignore_deferred_path
                .as_ref()
                .map(path_to_string_with_spaces),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AddSystemSerializedArgs {
    pub schedule_config: ScheduleConfigSerializedArgs,
}

impl AddSystemSerializedArgs {
    pub fn try_from_macro_attr(
        system: Path,
        attr: AddSystemArgs,
    ) -> syn::Result<impl Iterator<Item = Self>> {
        Ok(AddSystemWithTargetArgs::try_from_macro_attr(system, attr)?.map(Self::from))
    }
    pub fn to_tokens(self, app_ident: &Ident) -> syn::Result<MacroStream> {
        AddSystemWithTargetArgs::try_from(self)?.to_tokens(app_ident)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddSystemWithTargetArgs {
    pub schedule_config: ScheduleConfigWithSystemArgs,
}

impl AddSystemWithTargetArgs {
    pub fn try_from_macro_attr(
        system: Path,
        attr: AddSystemArgs,
    ) -> syn::Result<impl Iterator<Item = Self>> {
        let systems: Vec<Path> = if system.has_generics()? || !attr.has_generics() {
            vec![parse_quote! { #system }]
        } else {
            attr.generics
                .into_iter()
                .map(|generics| {
                    parse_quote! { #system::<#generics> }
                })
                .collect()
        };

        Ok(systems.into_iter().map(move |system| Self {
            schedule_config: ScheduleConfigWithSystemArgs::from_macro_attr(
                system,
                attr.schedule_config.clone(),
            ),
        }))
    }
    pub fn to_tokens(self, app_ident: &Ident) -> syn::Result<MacroStream> {
        let config_tokens = self.schedule_config.to_tokens()?;
        let schedule = self.schedule_config.schedule_path;
        let system = self.schedule_config.scheduled_item_path;
        Ok(quote! {
            #app_ident.add_systems(#schedule, #system #config_tokens);
        })
    }
}

impl TryFrom<AddSystemSerializedArgs> for AddSystemWithTargetArgs {
    type Error = syn::Error;
    fn try_from(value: AddSystemSerializedArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            schedule_config: ScheduleConfigWithSystemArgs::try_from(value.schedule_config)?,
        })
    }
}

impl From<AddSystemWithTargetArgs> for AddSystemSerializedArgs {
    fn from(value: AddSystemWithTargetArgs) -> Self {
        Self {
            schedule_config: value.schedule_config.into(),
        }
    }
}

#[derive(FromMeta, Debug, PartialEq, Hash, Clone)]
#[darling(derive_syn_parse)]
pub struct InsertResourceArgs {
    // only allow single
    #[darling(default)]
    pub generics: Option<TypeList>,
    pub resource: ExprValue,
}

impl InsertResourceArgs {
    pub fn validate_resource(&self) -> syn::Result<()> {
        if !matches!(
            self.resource.0,
            Expr::Call(_) // Foo(_)  or Foo::Bar(_)
            | Expr::Path(_) // Foo or Foo::Bar
            | Expr::Struct(_) // Foo { .. } or Foo::Bar { .. }
        ) {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a struct or enum value",
            ));
        }
        Ok(())
    }
}

impl TryFrom<InsertResourceSerializedArgsWithPath> for InsertResourceArgs {
    type Error = syn::Error;

    fn try_from(value: InsertResourceSerializedArgsWithPath) -> Result<Self, Self::Error> {
        Ok(Self {
            generics: value
                .generics_string
                .map(|s| {
                    let type_list = syn::parse_str::<TypeList>(&s)?;
                    syn::Result::Ok(Some(type_list))
                })
                .unwrap_or(Ok(None))?,
            resource: parse_str::<ExprValue>(&value.resource_expr_string)?,
        })
    }
}

impl TryFrom<ItemWithAttributeMatch> for InsertResourceArgs {
    type Error = syn::Error;
    fn try_from(value: ItemWithAttributeMatch) -> Result<Self, Self::Error> {
        Ok(InsertResourceArgs::from_meta(
            &value.matched_attribute.meta,
        )?)
    }
}

#[derive(Debug)]
pub struct InsertResourceArgsWithPath {
    pub path: Path,
    pub resource_args: InsertResourceArgs,
}
impl InsertResourceArgsWithPath {
    pub fn to_tokens(self, app_ident: &Ident) -> syn::Result<MacroStream> {
        let expected_path = &self.path;
        let resource = &self.resource_args.resource;

        Ok(quote! {
            #app_ident.insert_resource::<#expected_path>(#resource);
        })
    }
}

impl TryFrom<InsertResourceSerializedArgsWithPath> for InsertResourceArgsWithPath {
    type Error = syn::Error;
    fn try_from(value: InsertResourceSerializedArgsWithPath) -> Result<Self, Self::Error> {
        Ok(Self {
            path: parse_str(&value.path_string)?,
            resource_args: InsertResourceArgs::try_from(value)?,
        })
    }
}

impl TryFrom<ItemWithAttributeMatch> for InsertResourceArgsWithPath {
    type Error = syn::Error;

    fn try_from(value: ItemWithAttributeMatch) -> Result<Self, Self::Error> {
        Ok(Self {
            path: value.path.clone(),
            resource_args: InsertResourceArgs::try_from(value)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InsertResourceSerializedArgsWithPath {
    pub path_string: String,
    pub generics_string: Option<String>,
    pub resource_expr_string: String,
}

impl From<InsertResourceArgsWithPath> for InsertResourceSerializedArgsWithPath {
    fn from(value: InsertResourceArgsWithPath) -> Self {
        Self {
            path_string: path_to_string_with_spaces(&value.path),
            generics_string: value
                .resource_args
                .generics
                .as_ref()
                .map(|generics| generics.to_token_stream().to_string()),
            resource_expr_string: value.resource_args.resource.to_token_stream().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from_item_with_attribute_match_for_insert_resource_args() {
        let attr: Attribute = parse_quote! { #[foo(generics(usize), resource(Foo(1)))] };
        println!("{:?}", attr);
    }
}

pub fn default_app_ident() -> Ident {
    Ident::new("app", Span::call_site())
}
