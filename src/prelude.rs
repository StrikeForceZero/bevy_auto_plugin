#[cfg(feature = "mode_module")]
pub mod module {
    pub mod prelude {
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_add_event]
        ///     #[derive(Event, Reflect)]
        ///     struct FooEvent;
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.add_event::<FooEvent>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_add_event(generics(bool))]
        ///     #[derive(Event, Reflect)]
        ///     struct FooEventWithGeneric<T>(T);
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {
        ///     //     app.add_event::<FooEventWithGeneric<bool>>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_add_event as auto_add_event;
        pub use bevy_auto_plugin_proc_macros::module_auto_add_system as auto_add_system;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_init_resource]
        ///     #[derive(Resource, Default, Reflect)]
        ///     #[reflect(Resource)]
        ///     struct FooResource;
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.init_resource::<FooResource>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_init_resource(generics(bool))]
        ///     #[derive(Resource, Default, Reflect)]
        ///     #[reflect(Resource)]
        ///     struct FooResourceWithGeneric<T>(T);
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.init_resource::<FooResourceWithGeneric<bool>>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_init_resource as auto_init_resource;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_init_state]
        ///     #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
        ///     enum Foo {
        ///         #[default]
        ///         A,
        ///     }
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.init_state::<FooResource>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_init_state as auto_init_state;
        pub use bevy_auto_plugin_proc_macros::module_auto_insert_resource as auto_insert_resource;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_type]
        ///     #[derive(Component, Reflect)]
        ///     #[reflect(Component)]
        ///     #[auto_name]
        ///     struct FooComponent;
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {
        ///     //     app.register_type::<FooComponent>();
        ///     //     app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_type(generics(bool))]
        ///     #[auto_register_type(generics(u32))]
        ///     #[derive(Component, Reflect)]
        ///     #[reflect(Component)]
        ///     #[auto_name(generics(bool))]
        ///     struct FooComponentWithGeneric<T>(T);
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.register_type::<FooComponentWithGeneric<bool>>();
        ///     //     app.register_type::<FooComponentWithGeneric<u32>>();
        ///     //     app.register_required_components_with::<FooComponentWithGeneric<boo>, Name>(|| Name::new("FooComponentWithGeneric<boo>"));
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_name as auto_name;
        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_type]
        ///     #[derive(Component, Reflect)]
        ///     #[reflect(Component)]
        ///     pub struct MyComponent;
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.register_type::<MyComponent>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_plugin as auto_plugin;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_state_type]
        ///     #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
        ///     enum Foo {
        ///         #[default]
        ///         A,
        ///     }
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.register_type::<State<Foo>>();
        ///     //     app.register_type::<NextState<Foo>>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_register_state_type as auto_register_state_type;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_type]
        ///     #[derive(Component, Reflect)]
        ///     #[reflect(Component)]
        ///     struct FooComponent;
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.register_type::<FooComponent>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::module::prelude::*;
        ///
        /// #[auto_plugin(init_name=init)]
        /// pub mod my_plugin {
        ///     use bevy::prelude::*;
        ///     use bevy_auto_plugin::module::prelude::*;
        ///
        ///     #[auto_register_type(generics(bool))]
        ///     #[auto_register_type(generics(u32))]
        ///     #[derive(Component, Reflect)]
        ///     #[reflect(Component)]
        ///     struct FooComponentWithGeneric<T>(T);
        ///
        ///     /* code gen */
        ///     // pub(super) fn init(app: &mut App) {  
        ///     //     app.register_type::<FooComponentWithGeneric<bool>>();
        ///     //     app.register_type::<FooComponentWithGeneric<u32>>();
        ///     // }
        /// }
        ///
        /// fn plugin(app: &mut App) {
        ///     app.add_plugins(my_plugin::init);
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::module_auto_register_type as auto_register_type;
    }
}

#[cfg(feature = "mode_flat_file")]
pub mod flat_file {
    pub mod prelude {
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_add_event]
        /// #[derive(Event, Reflect)]
        /// struct FooEvent;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.add_event::<FooEvent>();
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_add_event(generics(bool))]
        /// #[derive(Event, Reflect)]
        /// struct FooEventWithGeneric<T>(T);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.add_event::<FooEventWithGeneric<bool>>();
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_event as auto_add_event;
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_system as auto_add_system;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_init_resource]
        /// #[derive(Resource, Default, Reflect)]
        /// #[reflect(Resource)]
        /// struct FooResource;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.init_resource::<FooResource>();
        /// }
        /// ```
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_init_resource(generics(bool))]
        /// #[derive(Resource, Default, Reflect)]
        /// #[reflect(Resource)]
        /// struct FooResourceWithGeneric<T>(T);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.init_resource::<FooResourceWithGeneric<bool>>();
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_init_resource as auto_init_resource;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_init_state]
        /// #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
        /// struct Foo;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.init_state::<Foo>();
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_init_state as auto_init_state;
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_insert_resource as auto_insert_resource;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_register_type]
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// #[auto_name]
        /// struct FooComponent;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.register_type::<FooComponent>();
        ///     // app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_register_type(generics(bool))]
        /// #[auto_register_type(generics(u32))]
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// #[auto_name(generics(bool))]
        /// struct FooComponentWithGeneric<T>(T);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.register_type::<FooComponentWithGeneric<bool>>();
        ///     // app.register_type::<FooComponentWithGeneric<u32>>();
        ///     // app.register_required_components_with::<FooComponentWithGeneric<boo>, Name>(|| Name::new("FooComponentWithGeneric<boo>"));
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_name as auto_name;
        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// // Example attributes or declarations for components, events, or resources
        /// // #[auto_register_type]
        /// // #[derive(Component, Reflect)]
        /// // #[reflect(Component)]
        /// // struct MyComponent;
        ///
        /// // ^ auto macro attributes must be declared above #[auto_plugin]
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     // Code generated by the macro is injected here.
        ///     // For example:
        ///     // app.register_type::<MyComponent>();
        ///
        ///     // Your custom logic comes here.
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_plugin as auto_plugin;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_register_state_type]
        /// #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
        /// struct Foo;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.register_type::<State<Foo>>();
        ///     // app.register_type::<NextState<Foo>>();
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_register_state_type as auto_register_state_type;
        #[doc(inline)]
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_register_type]
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// struct FooComponent;
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.register_type::<FooComponent>();
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[auto_register_type(generics(bool))]
        /// #[auto_register_type(generics(u32))]
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// struct FooComponentWithGeneric<T>(T);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.register_type::<FooComponentWithGeneric<bool>>();
        ///     // app.register_type::<FooComponentWithGeneric<u32>>();
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_register_type as auto_register_type;
    }
}

#[cfg(feature = "mode_global")]
pub mod global {
    pub mod prelude {
        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// // Plugin will automatically implement the Plugin trait
        /// // and include all registered components, events, resources, etc.
        /// ```
        pub use bevy_auto_plugin_proc_macros::AutoPlugin;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Event, Debug, Default, PartialEq, Reflect)]
        /// #[auto_register_type(plugin = MyPlugin)]
        /// #[auto_add_event(plugin = MyPlugin)]
        /// struct FooEvent(usize);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_add_event as auto_add_event;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Resource, Debug, Default)]
        /// struct FooResource(usize);
        ///
        /// #[auto_add_system(plugin = MyPlugin, schedule = Update)]
        /// fn foo_system(mut foo_res: ResMut<FooResource>) {
        ///     foo_res.0 += 1;
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_add_system as auto_add_system;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Resource, Debug, Default, PartialEq, Reflect)]
        /// #[reflect(Resource)]
        /// #[auto_register_type(plugin = MyPlugin)]
        /// #[auto_init_resource(plugin = MyPlugin)]
        /// struct FooResource(usize);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_init_resource as auto_init_resource;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
        /// #[auto_init_state(plugin = MyPlugin)]
        /// #[auto_register_state_type(plugin = MyPlugin)]
        /// enum FooState {
        ///     #[default]
        ///     Start,
        ///     End,
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_init_state as auto_init_state;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Resource, Debug, Default, PartialEq, Reflect)]
        /// #[reflect(Resource)]
        /// #[auto_register_type(plugin = MyPlugin)]
        /// #[auto_insert_resource(plugin = MyPlugin, resource(FooResource(42)))]
        /// struct FooResource(usize);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_insert_resource as auto_insert_resource;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// #[auto_register_type(plugin = MyPlugin)]
        /// #[auto_name(plugin = MyPlugin)]
        /// struct FooComponent;
        ///
        /// // This will automatically add a Name component to any entity with FooComponent
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_name as auto_name;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// // This macro enables the struct to act as a plugin that automatically
        /// // registers all types, resources, events, etc. marked with appropriate macros
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_plugin;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
        /// #[auto_init_state(plugin = MyPlugin)]
        /// #[auto_register_state_type(plugin = MyPlugin)]
        /// enum FooState {
        ///     #[default]
        ///     Start,
        ///     End,
        /// }
        ///
        /// // This will register both State<FooState> and NextState<FooState> with the type registry
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_register_state_type as auto_register_state_type;

        #[doc(inline)]
        ///
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Component, Reflect)]
        /// #[reflect(Component)]
        /// #[auto_register_type(plugin = MyPlugin)]
        /// struct FooComponent;
        ///
        /// // This will register FooComponent with the type registry
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_register_type as auto_register_type;
    }
}
