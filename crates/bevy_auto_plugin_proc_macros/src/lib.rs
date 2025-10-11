use bevy_auto_plugin_shared::__private::expand;
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

#[allow(dead_code)]
/// thin adapter converting between the compiler-level and proc_macro2 streams
fn handle_attribute<F: Fn(MacroStream, MacroStream) -> MacroStream>(
    handler: F,
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handler(attr.into(), input.into()).into()
}

/// Derives `AutoPlugin` which generates the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("../docs/proc_attributes/derive_auto_plugin.md")]
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
pub fn derive_auto_plugin(input: CompilerStream) -> CompilerStream {
    expand::derive::auto_plugin::expand_derive_auto_plugin(input.into()).into()
}

/// Attaches to a fn and injects a call to the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("../docs/proc_attributes/auto_plugin.md")]
#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
pub fn auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin::expand_auto_plugin, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_register_type.md")]
#[proc_macro_attribute]
pub fn auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_type, attr, input)
}

/// Automatically adds a message type to the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_add_message.md")]
#[proc_macro_attribute]
pub fn auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_message, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_init_resource.md")]
#[proc_macro_attribute]
pub fn auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_resource, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_insert_resource.md")]
#[proc_macro_attribute]
pub fn auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_insert_resource, attr, input)
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_init_state.md")]
#[proc_macro_attribute]
pub fn auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_state, attr, input)
}

/// Automatically registers a required component `Name` with a value using the concrete name of the item.
#[doc = include_str!("../docs/proc_attributes/auto_name.md")]
#[proc_macro_attribute]
pub fn auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_name, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_register_state_type.md")]
#[proc_macro_attribute]
pub fn auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_state_type, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_add_system.md")]
#[proc_macro_attribute]
pub fn auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_system, attr, input)
}

/// Automatically adds the fn as a proc_attributes observer to bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_add_observer.md")]
#[proc_macro_attribute]
pub fn auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_observer, attr, input)
}

/// Automatically registers item as Component for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_component.md")]
#[proc_macro_attribute]
pub fn auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_component, attr, input)
}

/// Automatically registers item as Resource for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_resource.md")]
#[proc_macro_attribute]
pub fn auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_resource, attr, input)
}

/// Automatically registers item as Event for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_event.md")]
#[proc_macro_attribute]
pub fn auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_event, attr, input)
}

/// Automatically registers item as Message for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_message.md")]
#[proc_macro_attribute]
pub fn auto_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_message, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_states.md")]
#[proc_macro_attribute]
pub fn auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_states, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_system.md")]
#[proc_macro_attribute]
pub fn auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_system, attr, input)
}

/// Automatically adds proc_attributes observer to bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_observer.md")]
#[proc_macro_attribute]
pub fn auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_observer, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[doc = include_str!("../docs/proc_attributes/auto_run_on_build.md")]
#[proc_macro_attribute]
pub fn auto_run_on_build(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_run_on_build, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[doc = include_str!("../docs/proc_attributes/auto_bind_plugin.md")]
#[proc_macro_attribute]
pub fn auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        expand::attr::auto_bind_plugin::auto_bind_plugin_outer,
        attr,
        input,
    )
}

// TODO: use the one in shared

#[derive(Debug)]
enum TakeAndPutAttrsError {
    ItemDoesNotHaveAttrs,
}

trait ItemAttrsExt {
    fn attrs_mut(&mut self) -> Result<&mut Vec<syn::Attribute>, TakeAndPutAttrsError>;
    fn take_attrs(&mut self) -> Result<Vec<syn::Attribute>, TakeAndPutAttrsError>;
    fn put_attrs(
        &mut self,
        attrs: Vec<syn::Attribute>,
    ) -> Result<Vec<syn::Attribute>, TakeAndPutAttrsError>;
}

impl ItemAttrsExt for syn::Item {
    fn attrs_mut(&mut self) -> Result<&mut Vec<syn::Attribute>, TakeAndPutAttrsError> {
        Ok(match self {
            syn::Item::Const(i) => &mut i.attrs,
            syn::Item::Enum(i) => &mut i.attrs,
            syn::Item::ExternCrate(i) => &mut i.attrs,
            syn::Item::Fn(i) => &mut i.attrs,
            syn::Item::ForeignMod(i) => &mut i.attrs,
            syn::Item::Impl(i) => &mut i.attrs,
            syn::Item::Macro(i) => &mut i.attrs,
            syn::Item::Mod(i) => &mut i.attrs,
            syn::Item::Static(i) => &mut i.attrs,
            syn::Item::Struct(i) => &mut i.attrs,
            syn::Item::Trait(i) => &mut i.attrs,
            syn::Item::TraitAlias(i) => &mut i.attrs,
            syn::Item::Type(i) => &mut i.attrs,
            syn::Item::Union(i) => &mut i.attrs,
            syn::Item::Use(i) => &mut i.attrs,
            syn::Item::Verbatim(_) => return Err(TakeAndPutAttrsError::ItemDoesNotHaveAttrs),
            _ => return Err(TakeAndPutAttrsError::ItemDoesNotHaveAttrs),
        })
    }
    fn take_attrs(&mut self) -> Result<Vec<syn::Attribute>, TakeAndPutAttrsError> {
        Ok(std::mem::take(self.attrs_mut()?))
    }
    fn put_attrs(
        &mut self,
        attrs: Vec<syn::Attribute>,
    ) -> Result<Vec<syn::Attribute>, TakeAndPutAttrsError> {
        Ok(std::mem::replace(self.attrs_mut()?, attrs))
    }
}

#[proc_macro_attribute]
pub fn inheritable(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use quote::{ToTokens, format_ident, quote};
    use syn::*;
    #[derive(darling::FromMeta, Default)]
    #[darling(default, derive_syn_parse)]
    struct Inheritable {
        suffix: Option<Ident>,
        merge: darling::util::Flag,
        blacklist: darling::util::Flag,
        filter: darling::util::PathList,
    }

    impl ToTokens for Inheritable {
        fn to_tokens(&self, tokens: &mut MacroStream) {
            let mut items = vec![];
            // if let Some(suffix) = &self.suffix {
            //     items.push(quote! { suffix = #suffix });
            // }
            if self.blacklist.is_present() {
                items.push(quote! { blacklist });
            }
            if !self.filter.is_empty() {
                for filter_path in self.filter.iter() {
                    items.push(quote! { filter(#filter_path) })
                }
            }
            tokens.extend(quote! { #(#items),* });
        }
    }

    let inheritable_meta = parse_macro_input!(attr as Inheritable);

    let cloned_input = input.clone();
    let input: MacroStream = input.into();
    let mut item = parse_macro_input!(cloned_input as Item);

    let macro_rules_ident = {
        let suffix = {
            if let Some(suffix) = &inheritable_meta.suffix {
                suffix
            } else {
                match &item {
                    Item::Fn(item) => &item.sig.ident,
                    Item::Enum(e) => &e.ident,
                    Item::Struct(s) => &s.ident,
                    _ => panic!("`inheritable` can only be used on functions, structs, and enums."),
                }
            }
        };
        format_ident!("inherit_{}", suffix)
    };

    let item_attrs = item.take_attrs().unwrap();
    item.put_attrs(item_attrs.clone()).unwrap();

    let mut item_attrs = item_attrs;

    if inheritable_meta.blacklist.is_present() {
        item_attrs.retain(|attr| !inheritable_meta.filter.contains(attr.path()));
    } else if !inheritable_meta.filter.is_empty() {
        item_attrs.retain(|attr| inheritable_meta.filter.contains(attr.path()));
    }

    let target_idents = item_attrs
        .iter()
        .map(|attr| attr.path())
        .collect::<Vec<_>>();

    let mut inherit_or_merge_args = vec![];
    if inheritable_meta.merge.is_present() {
        inherit_or_merge_args.push(quote! { merge });
    }

    if !target_idents.is_empty() {
        inherit_or_merge_args.push(quote! { target(#(#target_idents),*) });
    }

    let test_macro_rules_ident = format_ident!("test_{}", macro_rules_ident);

    let macro_rules_tokens = quote! {
        #[macro_export]
        #[allow(non_snake_case)]
        macro_rules! #macro_rules_ident {
            ($($tokens:tt)*) => {
                #[::bevy_auto_plugin::__private::_inherit_or_merge(#(#inherit_or_merge_args),*)]
                #(#item_attrs)*
                #[::bevy_auto_plugin::__private::_end_inherit_or_merge]
                $($tokens)*
            }
        }
        #[cfg(test)]
        macro_rules! #test_macro_rules_ident {
            ($($tokens:tt)*) => { ::quote::quote! {
                #[::bevy_auto_plugin::__private::_inherit_or_merge(#(#inherit_or_merge_args),*)]
                #(#item_attrs)*
                #[::bevy_auto_plugin::__private::_end_inherit_or_merge]
                $($tokens)*
            } }
        }
    };

    CompilerStream::from(quote! {
        #input

        #macro_rules_tokens
    })
}

#[proc_macro_attribute]
pub fn _inherit_or_merge(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use quote::quote;
    use syn::*;

    #[derive(darling::FromMeta, Default)]
    #[darling(default, derive_syn_parse)]
    struct InheritOrMerge {
        merge: darling::util::Flag,
        target: darling::util::PathList,
    }

    let inherit_or_merge_meta = parse_macro_input!(attr as InheritOrMerge);
    let mut item = parse_macro_input!(input as Item);

    let item_attrs = item.take_attrs().unwrap();

    enum AttrTarget {
        Target(Attribute),
        Skip(Attribute),
    }

    let mut source_attrs: Vec<Attribute> = vec![];
    let mut target_attrs: Vec<AttrTarget> = vec![];

    let mut is_filling_targets = false;

    for attr in item_attrs {
        let Some(last) = attr.path().segments.last() else {
            unreachable!();
        };
        if last.ident == "_inherit_or_merge" && source_attrs.is_empty() {
            continue;
        }
        if last.ident == "_end_inherit_or_merge" {
            is_filling_targets = true;
            continue;
        }
        if is_filling_targets {
            if !inherit_or_merge_meta
                .target
                .contains(&Path::from(last.ident.clone()))
            {
                target_attrs.push(AttrTarget::Skip(attr))
            } else {
                target_attrs.push(AttrTarget::Target(attr))
            }
        } else {
            source_attrs.push(attr);
        }
    }

    let mut final_attrs: Vec<Attribute> = vec![];

    for source_attr in source_attrs {
        if inherit_or_merge_meta.merge.is_present() {
            for target_attr in target_attrs.iter_mut() {
                match target_attr {
                    AttrTarget::Target(inner) => {
                        if inner.path().segments.last().unwrap().ident
                            == source_attr.path().segments.last().unwrap().ident
                        {
                            *inner = source_attr.clone();
                            todo!("not implemented");
                        } else {
                            continue;
                        }
                    }
                    AttrTarget::Skip(_) => continue,
                }
            }
        } else {
            final_attrs.push(source_attr);
        }
    }

    for target_attr in target_attrs {
        final_attrs.push(match target_attr {
            AttrTarget::Target(inner) => inner,
            AttrTarget::Skip(inner) => inner,
        });
    }

    CompilerStream::from(quote! {
        #(#final_attrs)*
        #item
    })
}

#[proc_macro_attribute]
pub fn _end_inherit_or_merge(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // passthrough
    input
}
