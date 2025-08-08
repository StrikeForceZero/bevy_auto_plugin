Automatically inserts a resource with a specific value into the app in module mode.

# Parameters
- `resource(Value)` - Required. Specifies the resource value to insert.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_init_resource]
    #[auto_insert_resource(resource(Test(1)))]
    #[derive(Resource, Default, Debug, PartialEq)]
    pub struct Test(pub usize);

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.init_resource::<Test>();
    //     app.insert_resource(Test(1));
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_insert_resource(resource(TestWithGeneric(1)), generics(usize))]
    #[derive(Resource, Default, Debug, PartialEq)]
    pub struct TestWithGeneric<T>(pub T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.insert_resource(TestWithGeneric::<usize>(1));
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```