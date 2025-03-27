use anyhow::Result;
use std::path::PathBuf;

pub struct StorageManager {
    base_path: PathBuf,
}

impl StorageManager {
    pub fn new(custom_path: Option<PathBuf>) -> Result<Self> {
        let base_path = custom_path.unwrap_or_else(|| {
            home::home_dir()
                .expect("Could not find home directory")
                .join(".sebas-rs")
        });

        // Ensure storage directory exists
        std::fs::create_dir_all(&base_path)?;

        Ok(StorageManager { base_path })
    }

    pub fn get_base_path(&self) -> &PathBuf {
        &self.base_path
    }

    // Additional storage-related methods can be added here
}
