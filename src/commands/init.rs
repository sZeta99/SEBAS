use std::{env, fs, path::PathBuf};

use crate::SebasApp;

impl SebasApp {

    pub fn init_folder(path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let target_path = path.unwrap_or_else(|| env::current_dir().unwrap());
        let sebas_dir = target_path.join(".sebas");
        
        if sebas_dir.exists() {
            println!("SEBAS folder already exists at: {}", sebas_dir.display());
            return Ok(());
        }

        fs::create_dir_all(&sebas_dir)?;
        println!("SEBAS folder initialized at: {}", sebas_dir.display());
        Ok(())
    }
}

