pub mod prelude {
    #[doc(inline)]
    /// Automatically registers an event to be added to the app in module mode.
    ///
    /// # Parameters
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
    ///   When provided, the event will be registered with these specific generic parameters.
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

    #[doc(inline)]
    /// Automatically registers a system to be added to the app in module mode.
    ///
    /// # Parameters
    /// - `schedule = ScheduleName` - Required. Specifies which schedule to add the system to.
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
    /// - `in_set = SetName` - Optional. Specifies which system set to add the system to.
    ///
    /// # Example (basic)
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_auto_plugin::module::prelude::*;
    ///
    /// #[auto_plugin(init_name=init)]
    /// pub mod my_plugin {
    ///     use bevy::prelude::*;
    ///     use bevy_auto_plugin::module::prelude::*;
    ///
    ///     #[derive(Resource, Debug, Default)]
    ///     pub(super) struct TestResource(pub i32);
    ///
    ///     #[auto_add_system(schedule = Update)]
    ///     pub(super) fn test_system(mut res: ResMut<TestResource>) {
    ///         res.0 += 1;
    ///     }
    /// }
    ///
    /// fn plugin(app: &mut App) {
    ///     app.init_resource::<my_plugin::TestResource>();
    ///     app.add_plugins(my_plugin::init);
    /// }
    /// ```
    ///
    /// # Example (with system set)
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_auto_plugin::module::prelude::*;
    ///
    /// #[auto_plugin(init_name=init)]
    /// pub mod my_plugin {
    ///     use bevy::prelude::*;
    ///     use bevy_auto_plugin::module::prelude::*;
    ///
    ///     #[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    ///     pub(super) enum TestSet {
    ///         First,
    ///     }
    ///
    ///     #[derive(Resource, Debug, Default)]
    ///     pub(super) struct TestResource(pub i32);
    ///
    ///     #[auto_add_system(schedule = Update, in_set = TestSet::First)]
    ///     pub(super) fn test_system(mut res: ResMut<TestResource>) {
    ///         res.0 += 1;
    ///     }
    /// }
    ///
    /// fn plugin(app: &mut App) {
    ///     app.init_resource::<my_plugin::TestResource>();
    ///     app.configure_sets(Update, my_plugin::TestSet::First);
    ///     app.add_plugins(my_plugin::init);
    /// }
    /// ```
    pub use bevy_auto_plugin_proc_macros::module_auto_add_system as auto_add_system;

    #[doc(inline)]
    /// Automatically registers a resource to be initialized in the app in module mode.
    ///
    /// # Parameters
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
    ///   When provided, the resource will be initialized with these specific generic parameters.
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
    /// Automatically initializes a state in the app in module mode.
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

    #[doc(inline)]
    /// Automatically inserts a resource with a specific value into the app in module mode.
    ///
    /// # Parameters
    /// - `resource(Value)` - Required. Specifies the resource value to insert.
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
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
    ///     #[auto_insert_resource(resource(Test(1)))]
    ///     #[derive(Resource, Default, Debug, PartialEq)]
    ///     pub struct Test(pub usize);
    ///
    ///     /* code gen */
    ///     // pub(super) fn init(app: &mut App) {  
    ///     //     app.init_resource::<Test>();
    ///     //     app.insert_resource(Test(1));
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
    ///     #[auto_insert_resource(resource(TestWithGeneric(1)), generics(usize))]
    ///     #[derive(Resource, Default, Debug, PartialEq)]
    ///     pub struct TestWithGeneric<T>(pub T);
    ///
    ///     /* code gen */
    ///     // pub(super) fn init(app: &mut App) {  
    ///     //     app.insert_resource(TestWithGeneric::<usize>(1));
    ///     // }
    /// }
    ///
    /// fn plugin(app: &mut App) {
    ///     app.add_plugins(my_plugin::init);
    /// }
    /// ```
    pub use bevy_auto_plugin_proc_macros::module_auto_insert_resource as auto_insert_resource;

    #[doc(inline)]
    /// Automatically adds a Name component to entities with this component in module mode.
    ///
    /// # Parameters
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
    ///   When provided, the Name component will be added to entities with this component
    ///   using the specified generic parameters.
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
    /// The main attribute for module mode that processes all auto attributes in the module.
    ///
    /// # Parameters
    /// - `init_name=identifier` - Optional. Specifies the name of the generated function that initializes the plugin. (defaults to init)
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
    /// Automatically registers `State<T>` and `NextState<T>` types with the app in module mode.
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
    /// Automatically registers a type with the app's type registry in module mode.
    ///
    /// # Parameters
    /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
    ///   When provided, the type will be registered with these specific generic parameters.
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
