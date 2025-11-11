use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::context::error::CRUDContextError;
use crate::context::Context;

// TODO: THIS IS a placeholder implementation
pub trait FileSystem {
    fn get_sebas_dir(&self) -> Result<PathBuf, CRUDContextError>;
    fn list_contexts(&self) -> Result<Vec<String>, CRUDContextError>
    where
        Self: Sized;
    fn ensure_sebas_directory(sebas_dir: &Path) -> Result<(), CRUDContextError>;
    fn cleanup_sebas_directory(base_path: &Path) -> Result<(), CRUDContextError>;
}

impl FileSystem for Context {
    fn get_sebas_dir(&self) -> Result<PathBuf, CRUDContextError> {
        if !self.path.exists() {
            return Err(CRUDContextError::InvalidPath(format!("{:?}", self.path)));
        }
        Ok(self.path.clone())
    }

    fn list_contexts(&self) -> Result<Vec<String>, CRUDContextError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let mut contexts = Vec::new();
        for entry in fs::read_dir(self.path.clone())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml") {
                if let Some(stem) = path.file_stem() {
                    if let Some(name) = stem.to_str() {
                        contexts.push(name.to_string());
                    :}
                }
            }
        }
        Ok(contexts)
    }

    fn ensure_sebas_directory(sebas_dir: &Path) -> Result<(), CRUDContextError> {
        if !sebas_dir.exists() {
            fs::create_dir_all(sebas_dir)?;
        }
        Ok(())
    }

    fn cleanup_sebas_directory(base_path: &Path) -> Result<(), CRUDContextError> {
        let sebas_dir = base_path.join(".sebas");
        if sebas_dir.exists() {
            fs::remove_dir_all(sebas_dir)?;
        }
        Ok(())
    }
}
