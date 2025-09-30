Automatically adds an observer to the Bevy `App`.

This attribute marks a function as an observer that will be automatically registered with the Bevy App when the module is initialized.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin]
mod my_module {
    use super::*;
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;
    
    #[derive(Component)]
    struct Foo;
    
    #[auto_add_observer]
    fn foo_observer(add: On<Add, Foo>, mut commands: Commands) {
        // ...
    }
}
```