use std::{env, fs, path::PathBuf};

use crate::SebasApp;

impl SebasApp {
    /// Initialization Function
    ///
    /// # Arguments
    ///
    /// * `Option: path` - Path of the folder where to init the sebas
    ///
    pub fn init_folder(
        path: Option<PathBuf>,
        current_path: PathBuf,
        extention_string: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let target_path = path.unwrap_or_else(|| current_path);
        let sebas_dir = target_path.join(extention_string);

        if sebas_dir.exists() {
            println!("SEBAS folder already exists at: {}", sebas_dir.display());
            return Ok(());
        }

        fs::create_dir_all(&sebas_dir)?;
        println!("SEBAS folder initialized at: {}", sebas_dir.display());
        Ok(())
    }
}
