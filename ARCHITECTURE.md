# Architecture

This document explains how the `bevy_auto_plugin` ecosystem is structured and how macro expansion flows through the crates.

## High-Level Flow

1. **User-facing macros** are re-exported from `bevy_auto_plugin` (in [`src/lib.rs`](src/lib.rs) under the `prelude`).
2. When a user applies:
    - `#[derive(AutoPlugin)]` → expansion handled by  
      [`bevy_auto_plugin_shared/src/__private/expand/derive/auto_plugin.rs`](crates/bevy_auto_plugin_shared/src/__private/expand/derive/auto_plugin.rs)
    - `#[auto_plugin]` (attribute form) → expansion handled by  
      [`bevy_auto_plugin_shared/src/__private/expand/attr/auto_plugin.rs`](crates/bevy_auto_plugin_shared/src/__private/expand/attr/auto_plugin.rs)
    - `#[auto_*( ... )]` attributes → routed to  
      [`bevy_auto_plugin_shared/src/__private/expand/attr/`](crates/bevy_auto_plugin_shared/src/__private/expand/attr/)

---

## Attribute Types

`auto_*` attributes fall into two categories:

### Actions

- Mutate the input token stream **and/or** append code that registers static slices used during plugin initialization.
- Parsing types are defined in:  
  [`bevy_auto_plugin_shared/src/macro_api/attributes/actions`](crates/bevy_auto_plugin_shared/src/macro_api/attributes/actions)

Each action attribute is modeled as: `ItemAttribute<Composed<T, P, G>, R>`

Where:

| Generic | Meaning |
|---------|----------|
| `T` | The [`darling`](https://crates.io/crates/darling) parameter struct for the attribute |
| `P` | Plugin context: `WithPlugin` |
| `G` | Generics handling: `WithZeroGenerics`, `WithZeroOrSingleGenerics`, or `WithZeroOrManyGenerics` |
| `R` | Resolver restricting usage: `AllowAny`, `AllowFn`, or `AllowStructOrEnum` |

Actions can implement traits for one or both of the following wrapper types:

#### [`AppMutationEmitter<T>`](crates/bevy_auto_plugin_shared/src/macro_api/emitters/app_mutation.rs)

- Generates tokens for static slice registration and plugin mutation.
- **May also mutate the input tokens** (rare).  
  Example use case: allowing helper attributes on enum variants.  
  See:  
  [`bevy_auto_plugin_shared/src/macro_api/attributes/actions/auto_configure_system_set.rs`](crates/bevy_auto_plugin_shared/src/macro_api/attributes/actions/auto_configure_system_set.rs)

#### [`AttrEmitter<T>`](crates/bevy_auto_plugin_shared/src/macro_api/emitters/attr.rs)

- Responsible for serializing a parsed parameter struct back into attribute tokens (useful for rewrite expansions).

---

### Rewrites

- Expand into one or more other attributes.
- Only modify the **item’s attributes**, not the item body.
- Uses wrapper type [`AttrExpansionEmitter<T>`](crates/bevy_auto_plugin_shared/src/macro_api/emitters/attr_expansion.rs)

---

## Supporting Infrastructure

### Custom Parsing

Located in:  
[`bevy_auto_plugin_shared/src/syntax/ast`](crates/bevy_auto_plugin_shared/src/syntax/ast)  
Includes custom AST fragments, parsing helpers, and type definitions used across attributes.

### Codegen Tokens

All hygienic Bevy paths are centralized in:  
[`bevy_auto_plugin_shared/src/codegen/tokens.rs`](crates/bevy_auto_plugin_shared/src/codegen/tokens.rs)

- Crate alias resolution is handled via [`proc-macro-crate`](https://crates.io/crates/proc-macro-crate).

### Token Preservation

Macro expansions are designed to **preserve input tokens on errors**.  
This ensures IDEs (like rust-analyzer) maintain valid syntax trees and prevent code “going red” on partially-written macro usage.
