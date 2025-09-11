use crate::{utils::dir::get_all_sebas_dirs, SebasApp};
impl SebasApp {
    pub fn sync_folders() -> Result<(), Box<dyn std::error::Error>> {
        println!("Syncing commands from nested .sebas folders...");
        
        let dirs = get_all_sebas_dirs();
        println!("Found {} .sebas folders to sync.", dirs.len());
        
        for dir in &dirs {
            let folder_name = dir.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            println!("  - {}", folder_name);
        }
        
        println!("Sync completed. Use 'sebas ls -v' to see all available commands.");
        Ok(())
    }
}

