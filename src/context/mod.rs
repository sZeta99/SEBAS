use std::{collections::HashMap, path::PathBuf};
pub mod file_system;
pub mod format;
use crate::{context::format::Format, group::Group};

pub struct ContextConfig {
    pub deafult_group: PathBuf,
    pub black_list: Vec<String>,
    pub format: Format,
}

pub struct Context {
    path: PathBuf,
    name: String,
    groups: HashMap<String, Group>,
    config: ContextConfig,
}
impl Context {
    /// Creates a new SebasContext instance
    pub fn new(name: String, path: PathBuf, config: ContextConfig) -> Self {
        Self {
            name,
            path,
            groups: HashMap::new(),
            config,
        }
    }

    /// Adds a new group to the context
    pub fn add_group(&mut self, group: Group) {
        self.groups.insert(group.get_name().clone(), group);
    }

    /// Returns the Name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Retrieves a specific group by name
    pub fn get_group(&self, group_name: &str) -> Option<&Group> {
        self.groups.get(group_name)
    }

    /// Lists all group names in the context
    pub fn list_groups(&self) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }
    /// Returns the path of the contex
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    /// Returns the path of the config
    pub fn get_config(&self) -> &ContextConfig {
        &self.config
    }

    pub fn remove_group(&mut self, name: &str) -> Option<Group> {
        self.groups.remove(name)
    }

    pub fn get_group_mut(&mut self, name: &str) -> Option<&mut Group> {
        self.groups.get_mut(name)
    }
}
