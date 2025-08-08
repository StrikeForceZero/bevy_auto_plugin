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
    /// - `in_set = SetName` - Optional. See [`bevy IntoScheduleConfigs in_set`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.in_set)
    /// - `before = SetName or system` - Optional. See [`bevy IntoScheduleConfigs before`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.before)
    /// - `after = SetName or system` - Optional. See [`bevy IntoScheduleConfigs after`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.after)
    /// - `run_if = Condition` - Optional. See [`bevy IntoScheduleConfigs run_if`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.run_if)
    /// - `distributive_run_if = Condition` - Optional. See [`bevy IntoScheduleConfigs run_if_inner`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.run_if_inner)
    /// - `ambiguous_with = System` - Optional. See [`bevy IntoScheduleConfigs ambiguous_with`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with)
    /// - `ambiguous_with_all = bool` - Optional. See [`bevy IntoScheduleConfigs ambiguous_with_all`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with_all)
    /// - `after_ignore_deferred = SetName or system` - Optional. See [`bevy IntoScheduleConfigs after_ignore_deferred`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.after_ignore_deferred)
    /// - `before_ignore_deferred = SetName or system` - Optional. See [`bevy IntoScheduleConfigs before_ignore_deferred`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.before_ignore_deferred)
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
    /// Attribute to mark the build function for the plugin, or impl Plugin trait build method for injection
    ///
    /// # Parameters
    /// - `plugin = PluginType` - **Required for bare functions only.** Specifies the plugin this build function belongs to.  
    ///   **Not allowed on `impl Plugin` methods**, since the plugin type is already known.
    /// - `app_param = identifier` - *(Optional)* Specifies the name of the `App` parameter that code will be injected into.  
    ///   Defaults to `app` if omitted.
    ///
    /// # Example - impl Plugin
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_auto_plugin::global::prelude::*;
    ///
    /// #[derive(AutoPlugin)]
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     #[global_auto_plugin(app_param=non_default_app_param_name)]
    ///     fn build(&self, non_default_app_param_name: &mut App) {
    ///         // code injected here
    ///
    ///         // your code
    ///     }
    /// }
    /// ```
    ///
    /// # Example - bare fn
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_auto_plugin::global::prelude::*;
    ///
    /// #[derive(AutoPlugin)]
    /// struct MyPlugin;
    ///
    /// #[global_auto_plugin(plugin = MyPlugin, app_param=non_default_app_param_name)]
    /// fn build(non_default_app_param_name: &mut App) {
    ///     // code injected here
    ///
    ///     // your code
    /// }
    /// ```
    pub use bevy_auto_plugin_proc_macros::global_auto_plugin;

    #[doc(inline)]
    /// Automatically registers `State<T>` and `NextState<T>` types with the app in global mode.
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
