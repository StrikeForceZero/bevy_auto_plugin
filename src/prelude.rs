#[cfg(feature = "mode_module")]
pub mod module {
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
        /// Automatically registers State<T> and NextState<T> types with the app in module mode.
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
}

#[cfg(feature = "mode_flat_file")]
pub mod flat_file {
    pub mod prelude {
        #[doc(inline)]
        /// Automatically registers an event to be added to the app.
        ///
        /// # Parameters
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the event will be registered with these specific generic parameters.
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

        #[doc(inline)]
        /// Automatically registers a system to be added to the app.
        ///
        /// # Parameters
        /// - `schedule = ScheduleName` - Required. Specifies which schedule to add the system to.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        /// - `in_set = SetName` - Optional. Specifies which system set to add the system to.
        ///
        /// # Example (basic)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[derive(Resource, Debug, Default)]
        /// struct TestResource(i32);
        ///
        /// #[auto_add_system(schedule = Update)]
        /// fn test_system(mut res: ResMut<TestResource>) {
        ///     res.0 += 1;
        /// }
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     app.init_resource::<TestResource>();
        ///     /* generated code */
        ///     // app.add_systems(Update, test_system);
        /// }
        /// ```
        ///
        /// # Example (with system set)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
        /// enum TestSet { First, Second }
        ///
        /// #[derive(Resource, Debug, Default)]
        /// struct TestResource(i32);
        ///
        /// #[auto_add_system(schedule = Update, in_set = TestSet::First)]
        /// fn test_system(mut res: ResMut<TestResource>) {
        ///     res.0 += 1;
        /// }
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     app.init_resource::<TestResource>();
        ///     app.configure_sets(Update, TestSet::First);
        ///     /* generated code */
        ///     // app.add_systems(Update, test_system.in_set(TestSet::First));
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_system as auto_add_system;

        #[doc(inline)]
        /// Automatically registers a resource to be initialized in the app.
        ///
        /// # Parameters
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the resource will be initialized with these specific generic parameters.
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
        /// Automatically initializes a state in the app.
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

        #[doc(inline)]
        /// Automatically inserts a resource with a specific value into the app.
        ///
        /// # Parameters
        /// - `resource(Value)` - Required. Specifies the resource value to insert.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///
        /// # Example (without generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[derive(Resource, Debug, Default, PartialEq, Reflect)]
        /// #[reflect(Resource)]
        /// #[auto_insert_resource(resource(FooResource(42)))]
        /// struct FooResource(usize);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.insert_resource(FooResource(42));
        /// }
        /// ```
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::flat_file::prelude::*;
        ///
        /// #[derive(Resource, Debug, Default, PartialEq, Reflect)]
        /// #[reflect(Resource)]
        /// #[auto_insert_resource(resource(FooResourceWithGeneric(42)), generics(usize))]
        /// struct FooResourceWithGeneric<T>(T);
        ///
        /// #[auto_plugin(app=app)]
        /// fn plugin(app: &mut App) {
        ///     /* generated code */
        ///     // app.insert_resource(FooResourceWithGeneric::<usize>(42));
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_insert_resource as auto_insert_resource;

        #[doc(inline)]
        /// Automatically adds a Name component to entities with this component.
        ///
        /// # Parameters
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the Name component will be added to entities with this component
        ///   using the specified generic parameters.
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
        ///     // app.register_required_components_with::<FooComponentWithGeneric<bool>, Name>(|| Name::new("FooComponentWithGeneric<bool>"));
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::flat_file_auto_name as auto_name;

        #[doc(inline)]
        /// The main attribute for flat file mode that processes all auto attributes in the file.
        ///
        /// # Parameters
        /// - `app=identifier` - Optional. Specifies the app variable name that code will be injected into. (defaults to app)
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
        /// Automatically registers State<T> and NextState<T> types with the app.
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
        /// Automatically registers a type with the app's type registry.
        ///
        /// # Parameters
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the type will be registered with these specific generic parameters.
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
        /// A derive macro that implements Plugin for a struct and collects registered components,
        /// events, resources, and systems.
        ///
        /// # Parameters
        /// - `impl_plugin_trait` - Optional. When present, automatically implements the Plugin trait.
        /// - `impl_generic_plugin_trait` - Optional. When present, automatically implements the Plugin trait universally across all generics.
        /// - `impl_generic_auto_plugin_trait` - Optional. When present, automatically implements the AutoPlugin trait universally across all generics.
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
        /// Automatically registers an event to be added to the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should register this event.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the event will be registered with these specific generic parameters.
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
        ///
        /// # Example (with generics)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Event, Debug, Default, PartialEq, Reflect)]
        /// #[auto_register_type(plugin = MyPlugin, generics(usize))]
        /// #[auto_add_event(plugin = MyPlugin, generics(usize))]
        /// struct FooEventWithGeneric<T>(T);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_add_event as auto_add_event;

        #[doc(inline)]
        /// Automatically registers a system to be added to the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should register this system.
        /// - `schedule = ScheduleName` - Required. Specifies which schedule to add the system to.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        /// - `in_set = SetName` - Optional. Specifies which system set to add the system to.
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
        ///
        /// # Example (with system set)
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_auto_plugin::global::prelude::*;
        ///
        /// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
        /// enum TestSet { First, Second }
        ///
        /// #[derive(AutoPlugin)]
        /// #[auto_plugin(impl_plugin_trait)]
        /// struct MyPlugin;
        ///
        /// #[derive(Resource, Debug, Default)]
        /// struct FooResource(usize);
        ///
        /// #[auto_add_system(plugin = MyPlugin, schedule = Update, in_set = TestSet::First)]
        /// fn foo_system(mut foo_res: ResMut<FooResource>) {
        ///     foo_res.0 += 1;
        /// }
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_add_system as auto_add_system;

        #[doc(inline)]
        /// Automatically registers a resource to be initialized in the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should initialize this resource.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the resource will be initialized with these specific generic parameters.
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
        ///
        /// # Example (with generics)
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
        /// #[auto_register_type(plugin = MyPlugin, generics(usize))]
        /// #[auto_init_resource(plugin = MyPlugin, generics(usize))]
        /// struct FooResourceWithGeneric<T>(T);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_init_resource as auto_init_resource;

        #[doc(inline)]
        /// Automatically initializes a state in the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should initialize this state.
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
        /// Automatically inserts a resource with a specific value into the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should insert this resource.
        /// - `resource(Value)` - Required. Specifies the resource value to insert.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the resource will be inserted with these specific generic parameters.
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
        ///
        /// # Example (with generics)
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
        /// #[auto_register_type(plugin = MyPlugin, generics(usize))]
        /// #[auto_insert_resource(plugin = MyPlugin, resource(FooResourceWithGeneric(42)), generics(usize))]
        /// struct FooResourceWithGeneric<T>(T);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_insert_resource as auto_insert_resource;

        #[doc(inline)]
        /// Automatically adds a Name component to entities with this component in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should register this name.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the Name component will be added to entities with this component
        ///   using the specified generic parameters.
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
        ///
        /// # Example (with generics)
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
        /// #[auto_register_type(plugin = MyPlugin, generics(bool))]
        /// #[auto_name(plugin = MyPlugin, generics(bool))]
        /// struct FooComponentWithGeneric<T>(T);
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_name as auto_name;

        #[doc(inline)]
        /// Main attribute for global mode used on structs that will become plugins.
        ///
        /// # Parameters
        /// - `impl_plugin_trait` - Optional. When present, automatically implements the Plugin trait.
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
        /// Automatically registers State<T> and NextState<T> types with the app in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should register these state types.
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
        /// Automatically registers a type with the app's type registry in global mode.
        ///
        /// # Parameters
        /// - `plugin = PluginType` - Required. Specifies which plugin should register this type.
        /// - `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
        ///   When provided, the type will be registered with these specific generic parameters.
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
        ///
        /// # Example (with generics)
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
        /// #[auto_register_type(plugin = MyPlugin, generics(bool))]
        /// #[auto_register_type(plugin = MyPlugin, generics(u32))]
        /// struct FooComponentWithGeneric<T>(T);
        ///
        /// // This will register FooComponentWithGeneric<bool> and FooComponentWithGeneric<u32>
        /// // with the type registry
        /// ```
        pub use bevy_auto_plugin_proc_macros::global_auto_register_type as auto_register_type;
    }
}
