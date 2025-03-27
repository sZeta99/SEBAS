use anyhow::{Result, Context, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use std::env;
use colored::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandEntry {
    command: String,
    comment: Option<String>,
    aliases: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupEntry {
    group: String,
    commands: Vec<CommandEntry>,
}

pub fn get_storage_path(location: Option<PathBuf>) -> PathBuf {
    location.unwrap_or_else(|| {
        env::current_dir().expect("Could not get current directory")
            .join(".sebas")
    })
}

fn sanitize_filename(input: &str) -> String {
    input.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect::<String>()
        .to_lowercase()
}

pub fn add_command(
    command: String, 
    group: Option<String>, 
    comment: Option<String>, 
    aliases: Option<String>, 
    location: Option<PathBuf>
) -> Result<()> {
    let storage_path = get_storage_path(location);
    fs::create_dir_all(&storage_path)?;

    // Use 'Ungrouped' if no group is specified
    let group_name = group.unwrap_or_else(|| "Ungrouped".to_string());
    let sanitized_group = sanitize_filename(&group_name);
    let file_path = storage_path.join(format!("{}.yaml", sanitized_group));

    // Parse aliases
    let parsed_aliases = aliases.map(|a| a.split(',').map(String::from).collect());

    let new_command = CommandEntry {
        command,
        comment,
        aliases: parsed_aliases,
    };

    // Read existing group entries or create new
    let mut group_entry = if file_path.exists() {
        let content = fs::read_to_string(&file_path)?;
        serde_yaml::from_str(&content)?
    } else {
        GroupEntry {
            group: group_name.clone(),
            commands: Vec::new(),
        }
    };

    // Add new command
    group_entry.commands.push(new_command);

    // Write back to file
    let yaml = serde_yaml::to_string(&group_entry)?;
    fs::write(file_path, yaml)
        .context("Failed to write command entry")?;

    println!("Command added to group '{}' successfully!", group_name.green());
    Ok(())
}

pub fn remove_command(identifier: &str, location: Option<PathBuf>) -> Result<()> {
    let storage_path = get_storage_path(location);

    // Search through all YAML files in the storage directory
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Only process YAML files
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = fs::read_to_string(&path)?;
            let mut group_entry: GroupEntry = serde_yaml::from_str(&content)?;
            
            // Find and remove matching command
            if let Some(pos) = group_entry.commands.iter()
                .position(|cmd| 
                    cmd.command.contains(identifier) || 
                    cmd.aliases.as_ref().map_or(false, |aliases| 
                        aliases.iter().any(|alias| alias.contains(identifier))
                    )
                ) {
                let removed_cmd = group_entry.commands.remove(pos);
                
                // If no commands left, remove the group file
                if group_entry.commands.is_empty() {
                    fs::remove_file(&path)?;
                    println!("Removed group: {}", path.file_stem().unwrap().to_string_lossy().green());
                } else {
                    // Update the file with remaining commands
                    let yaml = serde_yaml::to_string(&group_entry)?;
                    fs::write(&path, yaml)?;
                    println!("Removed command: {}", removed_cmd.command.green());
                }
                
                return Ok(());
            }
        }
    }

    Err(anyhow!("No command found matching: {}", identifier))
}

pub fn list_commands(location: Option<PathBuf>) -> Result<()> {
    let storage_path = get_storage_path(location);

    // Check if any commands exist
    let entries: Vec<_> = fs::read_dir(&storage_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
        .collect();

    if entries.is_empty() {
        println!("{}", "No commands stored yet.".yellow());
        return Ok(());
    }

    // Print header
    println!("{}", "Stored Commands:".bold().blue());
    println!("{}", "===============".blue());

    let mut total_commands = 0;

    // Sort group files
    let mut sorted_entries = entries;
    sorted_entries.sort_by_key(|entry| entry.file_name());

    // Iterate through sorted group files
    for entry in sorted_entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)?;
        let group_entry: GroupEntry = serde_yaml::from_str(&content)?;
        
        // Print group header
        println!("\n{}: ", group_entry.group.bold().green());
        
        // Sort commands within the group
        let mut sorted_commands = group_entry.commands;
        sorted_commands.sort_by(|a, b| a.command.cmp(&b.command));

        // Print each command in the group
        for (index, command) in sorted_commands.iter().enumerate() {
            // Command with index
            print!("{:2}. {}", index + 1, command.command.cyan());
            
            // Aliases (if any)
            if let Some(aliases) = &command.aliases {
                print!(" {}", format!("[{}]", aliases.join(", ")).dimmed());
            }
            
            // Comment (if any)
            if let Some(comment) = &command.comment {
                print!(" - {}", comment.italic().dimmed());
            }
            
            println!(); // New line
        }

        total_commands += sorted_commands.len();
    }

    // Footer with total commands
    println!("\n{} Total Commands", total_commands.to_string().bold().blue());

    Ok(())
}

pub fn retrieve_command(query: &str, location: Option<PathBuf>) -> Result<String> {
    let storage_path = get_storage_path(location);

    // Search through all YAML files in the storage directory
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Only process YAML files
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = fs::read_to_string(&path)?;
            let group_entry: GroupEntry = serde_yaml::from_str(&content)?;
            
            // Check if query matches any command in the group
            for command_entry in &group_entry.commands {
                if command_entry.command.contains(query) || 
                   command_entry.aliases.as_ref().map_or(false, |aliases| 
                       aliases.iter().any(|alias| alias.contains(query))) {
                    return Ok(command_entry.command.clone());
                }
            }
        }
    }

    Err(anyhow!("No command found matching: {}", query))
}




pub fn edit_command(command: &str, location: Option<PathBuf>) -> Result<()> {
    // TODO: Implement more robust edit functionality
    // For now, just find and print the command
    let existing_command = retrieve_command(command, location)?;
    println!("Found command to edit: {}", existing_command);
    
    // In a real implementation, this would open an editor
    // or prompt for new command details
    Ok(())
}
pub fn run_command(query: &str, location: Option<PathBuf>) -> Result<()> {
    let command = retrieve_command(query, location)?;
    
    // Simply print the command for the user to copy or for shell integration
    println!("Command: {}", command);
    
    Ok(())
}
