use bevy_app::App;
use bevy_internal::MinimalPlugins;
use std::any::TypeId;

pub fn create_minimal_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

pub fn type_id_of<T>() -> TypeId
where
    T: 'static,
{
    TypeId::of::<T>()
}
