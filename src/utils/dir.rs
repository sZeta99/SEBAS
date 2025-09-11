use std::{env, path::PathBuf};

pub fn find_sebas_dir() -> Option<PathBuf> {
        let mut current_dir = env::current_dir().ok()?;
        loop {
            let candidate = current_dir.join(".sebas");
            if candidate.is_dir() {
                return Some(candidate);
            }
            if !current_dir.pop() {
                break;
            }
        }
        None
    }

pub fn get_all_sebas_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        let mut current_dir = env::current_dir().unwrap();
        
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
