use crate::attribute_args::{AddSystemSerializedArgs, InsertResourceSerializedArgsWithPath};
use std::collections::HashSet;

#[derive(Default)]
pub struct AutoPluginContext {
    pub register_types: HashSet<String>,
    pub register_state_types: HashSet<String>,
    pub add_events: HashSet<String>,
    pub init_resources: HashSet<String>,
    pub insert_resources: HashSet<InsertResourceSerializedArgsWithPath>,
    pub init_states: HashSet<String>,
    pub auto_names: HashSet<String>,
    pub add_systems: HashSet<AddSystemSerializedArgs>,
    pub add_observers: HashSet<String>,
}
