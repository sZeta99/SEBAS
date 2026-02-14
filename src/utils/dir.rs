use std::{collections::HashMap, path::PathBuf};

pub fn get_first_reachable_sebas_dir(current_dir: &PathBuf) -> Option<PathBuf> {
    let mut current_dir = current_dir.clone();
    loop {
        let candidate = current_dir.join(".sebas");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !current_dir.pop() {
            break;
        }
    }
    //The use of iteretor ancestor may be preferable
    None
}

pub fn get_all_reachable_sebas_dir(current_dir: &PathBuf) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut current_dir = current_dir.clone();

    loop {
        let candidate = current_dir.join(".sebas");
        if candidate.is_dir() {
            dirs.push(candidate);
        }
        if !current_dir.pop() {
            break;
        }
    }
    dirs
}
/// List all the group in a dir
:TODO
pub fn get_all_groups_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
    Vec::new()
}
/// List all group with the path, allow to have muliple path for group
pub fn get_all_groups_in_vec(dirs: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    HashMap::new()
}
