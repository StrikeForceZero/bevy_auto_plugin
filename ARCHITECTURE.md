# Architecture

## Flow
- each `bevy_auto_plugin_proc_macros` re-exported by `bevy_auto_plugin` crate in `src/lib` `prelude`
- `#[derive(AutoPlugin)]`
  - calls into `crates/bevy_auto_plugin_shared/src/__private/expand/derive/auto_plugin.rs`
- `#[auto_plugin]`
  - calls into `crates/bevy_auto_plugin_shared/src/__private/expand/attr/auto_plugin.rs`
- `#[auto_*(...)]`
  - calls into `crates/bevy_auto_plugin_shared/src/__private/expand/attr`
    - actions are items that change the input token stream or append tokens to register the static slices during initialization
    - rewrites are attributes that expand into several other attributes. only modifies input item attrs.
  - params defined in `crates/bevy_auto_plugin_shared/src/macro_api/attributes/actions`
    - `ItemAttribute<Composed<T, P, G>, R>`
      - `T` = [darling](https://crates.io/crates/darling) param struct
      - `P` = plugin params: `WithPlugin`
      - `G` = generics: `WithZeroGenerics | WithZeroOrSingleGenerics | WithZeroOrManyGenerics` 
      - `R` = resolver: `AllowAny | AllowFn | AllowStructOrEnum`
    - Each action attribute may impl traits for `AppMutationEmitter<T>` or `AttrEmitter<T>`.
      - `AppMutationEmitter<T>` is responsible for the tokens generated for each respective static slice being constructed
        - bonus: it can also override the implementation to mutate the input tokens. for example, allowing helper attributes to exist on an enum variant.
          - see `crates/bevy_auto_plugin_shared/src/macro_api/attributes/actions/auto_configure_system_set.rs` for the only current use case.
      - `AttrEmitter<T>` is responsible for serializing a param sctruct back to tokens
  - there's several custom parsing types in `crates/bevy_auto_plugin_shared/src/syntax/ast`
  - all hygenic bevy paths are in `crates/bevy_auto_plugin_shared/src/codegen/tokens.rs`
    - crate aliases are resolved using `proc-macro-crate`
  - input tokens are expected to be preserved during errors. without this, IDEs can struggle when editing macros. 