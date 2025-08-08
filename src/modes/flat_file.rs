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
    /// Automatically registers `State<T>` and `NextState<T>` types with the app.
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
