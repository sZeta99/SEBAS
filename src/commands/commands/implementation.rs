use crate::{ commands::core::definition::SavedCommand, utils::preseed::preseed, SebasApp};
use std::process::Command as ProcessCommand;

impl SebasApp {
    pub fn add_command(&self, command_text: Option<String>, group: Option<String>, comment: Option<String>, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        let command = if let Some(cmd) = command_text {
            cmd
        } else if let Some(cmd) = Self::get_command_from_stdin() {
            cmd
        } else if let Some(cmd) = Self::get_last_shell_command() {
            if !yes && !Self::confirm(&format!("Save last command: '{}'?", cmd)) {
                println!("Command not saved.");
                return Ok(());
            }
            cmd
        } else {
            return Err("No command provided. Use stdin, provide as argument, or ensure shell history is available.".into());
        };

        let group_name = group.unwrap_or_else(|| "Miscellaneous".to_string());
        let mut group_data = self.load_group(&group_name)?;
        
        // Check if group file exists
        let group_file = self.sebas_dir.join(format!("{}.yaml", group_name));
        if !group_file.exists() && !yes {
            if !Self::confirm(&format!("Group '{}' does not exist. Create it?", group_name)) {
                println!("Command not added.");
                return Ok(());
            }
        }

        let hash = Self::generate_hash(&command);
        let saved_command = SavedCommand {
            command: command.clone(),
            comment,
            hash: hash.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        group_data.commands.push(saved_command);
        self.save_group(&group_name, &group_data)?;
        
        println!("Command added to group '{}' with hash {}", group_name, hash);
        Ok(())
    }

    pub fn list_commands(&self, group_filter: Option<String>, verbose: bool, plain: bool) -> Result<(), Box<dyn std::error::Error>> {
        let resolved = self.resolve_all_commands();
        
        if resolved.is_empty() {
            println!("No commands saved.");
            return Ok(());
        }

        for cmd in resolved {
            if let Some(ref filter) = group_filter {
                if cmd.group != *filter {
                    continue;
                }
            }

            if plain {
                println!("{}", cmd.command.command);
            } else if verbose {
                println!("[{}] {} ({}/{}) - ID: {}", 
                    cmd.index,
                    cmd.command.command,
                    cmd.folder_path.parent().unwrap_or(&cmd.folder_path).display(),
                    cmd.group,
                    cmd.command.hash
                );
                if let Some(ref comment) = cmd.command.comment {
                    println!("    Comment: {}", comment);
                }
            } else {
                println!("[{}] {} ({}) - ID: {}", 
                    cmd.index,
                    cmd.command.command,
                    cmd.group,
                    cmd.command.hash
                );
            }
        }
        
        Ok(())
    }

    pub fn edit_command(&self, identifier: &str, new_command: Option<String>, new_group: Option<String>, new_comment: Option<String>, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        let resolved_cmd = self.find_command_by_identifier(identifier)
            .ok_or("Command not found")?;
        
        if !yes {
            println!("Editing command: {}", resolved_cmd.command.command);
            if !Self::confirm("Continue with edit?") {
                println!("Edit cancelled.");
                return Ok(());
            }
        }

        // If changing group, we need to move the command
        let target_group = new_group.as_ref().unwrap_or(&resolved_cmd.group);
        
        // Load current group and remove the command
        let app = SebasApp { sebas_dir: resolved_cmd.folder_path.clone() };
        let mut current_group = app.load_group(&resolved_cmd.group)?;
        current_group.commands.retain(|cmd| cmd.hash != resolved_cmd.command.hash);
        
        // Update command
        let mut updated_command = resolved_cmd.command.clone();
        if let Some(cmd) = new_command {
            updated_command.command = cmd;
            updated_command.hash = Self::generate_hash(&updated_command.command);
        }
        if let Some(comment) = new_comment {
            updated_command.comment = Some(comment);
        }

        // Save to target group
        let mut target_group_data = app.load_group(target_group)?;
        target_group_data.commands.push(updated_command);
        
        app.save_group(&resolved_cmd.group, &current_group)?;
        app.save_group(target_group, &target_group_data)?;
        
        println!("Command updated successfully.");
        Ok(())
    }

    pub fn remove_command(&self, identifier: &str, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
        let resolved_cmd = self.find_command_by_identifier(identifier)
            .ok_or("Command not found")?;
        
        if !yes {
            if !Self::confirm(&format!("Delete command: '{}'?", resolved_cmd.command.command)) {
                println!("Deletion cancelled.");
                return Ok(());
            }
        }

        let app = SebasApp { sebas_dir: resolved_cmd.folder_path.clone() };
        let mut group = app.load_group(&resolved_cmd.group)?;
        group.commands.retain(|cmd| cmd.hash != resolved_cmd.command.hash);
        app.save_group(&resolved_cmd.group, &group)?;
        
        println!("Command deleted successfully.");
        Ok(())
    }

    pub fn search_commands(&self, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let resolved = self.resolve_all_commands();
        let query_lower = query.to_lowercase();
        let mut found = false;

        for cmd in resolved {
            let command_lower = cmd.command.command.to_lowercase();
            let comment_matches = cmd.command.comment
                .as_ref()
                .map(|c| c.to_lowercase().contains(&query_lower))
                .unwrap_or(false);
            
            if command_lower.contains(&query_lower) || comment_matches {
                println!("[{}] {} ({}) - ID: {}", 
                    cmd.index,
                    cmd.command.command,
                    cmd.group,
                    cmd.command.hash
                );
                if let Some(ref comment) = cmd.command.comment {
                    println!("    Comment: {}", comment);
                }
                found = true;
            }
        }

        if !found {
            println!("No commands found matching '{}'.", query);
        }
        
        Ok(())
    }

    pub fn get_command(&self, identifier: &str) -> Result<(), Box<dyn std::error::Error>> {
        let resolved_cmd = self.find_command_by_identifier(identifier)
            .ok_or("Command not found")?;
        
        let _ = preseed(&resolved_cmd.command.command);
        Ok(())
    }

    pub fn list_groups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let groups = self.get_all_groups()?;
        
        if groups.is_empty() {
            println!("No groups found.");
            return Ok(());
        }

        for group in groups {
            let group_data = self.load_group(&group)?;
            let folder_name = self.sebas_dir
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            println!("{}/{} ({} commands)", folder_name, group, group_data.commands.len());
        }
        
        Ok(())
    }
}

