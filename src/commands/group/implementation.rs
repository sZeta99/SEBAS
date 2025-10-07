use std::{clone, fs, path::PathBuf};

use crate::{commands::core::definition::CommandGroup, SebasApp};

impl SebasApp {
    pub fn load_group(&self, group_name: &str) -> Result<CommandGroup, Box<dyn std::error::Error>> {
        let group_file = self.sebas_dir.join(format!("{}.yaml", group_name));
        if !group_file.exists() {
            return Ok(CommandGroup::new());
        }
        
        let content = fs::read_to_string(&group_file)?;
        let group: CommandGroup = serde_yaml::from_str(&content)?;
        Ok(group)
    }

    pub fn save_group(&self, group_name: &str, group_file: PathBuf, group: &CommandGroup) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(group)?;
        fs::write(&group_file, content)?;
        Ok(())
    }

    pub fn get_all_groups(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut groups = Vec::new();
        for entry in fs::read_dir(&self.sebas_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    groups.push(name.to_string());
                }
            }
        }
        groups.sort();
        Ok(groups)
    }

    pub fn add_group(&self, name: &str, path: Option<PathBuf> ,yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        
        let group_file = if let Some(p) = path {
            p.join(format!("{}.yaml", name))

        } else {
            self.sebas_dir.join(format!("{}.yaml", name))
        };
               
        if group_file.exists() {
            println!("Group '{}' already exists.", name);
            return Ok(());
        }

        if !yes && !Self::confirm(&format!("Create group '{}'?", name)) {
            println!("Group creation cancelled.");
            return Ok(());
        }

        let group = CommandGroup::new();
        self.save_group(name, group_file, &group)?;
        println!("Group '{}' created successfully.", name);
        Ok(())
    }

    pub fn rename_group(&self, old_name: &str, new_name: &str, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        let old_file = self.sebas_dir.join(format!("{}.yaml", old_name));
        let new_file = self.sebas_dir.join(format!("{}.yaml", new_name));
        
        if !old_file.exists() {
            return Err(format!("Group '{}' not found.", old_name).into());
        }
        
        if new_file.exists() {
            return Err(format!("Group '{}' already exists.", new_name).into());
        }

        if !yes && !Self::confirm(&format!("Rename group '{}' to '{}'?", old_name, new_name)) {
            println!("Rename cancelled.");
            return Ok(());
        }

        fs::rename(&old_file, &new_file)?;
        println!("Group '{}' renamed to '{}'.", old_name, new_name);
        Ok(())
    }

    pub fn remove_group(&self, name: &str, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        let group_file = self.sebas_dir.join(format!("{}.yaml", name));
        
        if !group_file.exists() {
            return Err(format!("Group '{}' not found.", name).into());
        }

        let group = self.load_group(name)?;
        let command_count = group.commands.len();

        if !yes && !Self::confirm(&format!("Delete group '{}' and all its {} commands?", name, command_count)) {
            println!("Deletion cancelled.");
            return Ok(());
        }

        fs::remove_file(&group_file)?;
        println!("Group '{}' and {} commands deleted.", name, command_count);
        Ok(())
    }

}

