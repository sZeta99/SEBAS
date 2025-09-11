use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ResolvedCommand {
    pub(crate) command: SavedCommand,
    pub(crate) group: String,
    pub(crate) folder_path: PathBuf,
   pub(crate) index: usize,
}
#[derive(Serialize, Deserialize,
    Debug, Clone)]
pub struct SavedCommand {
    pub(crate) command: String,
    pub(crate)comment: Option<String>,
    pub(crate) hash: String,
    pub(crate) created_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandGroup {
    pub(crate) commands: Vec<SavedCommand>,
}


