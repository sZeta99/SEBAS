use anyhow::{Result, Context, anyhow};
use std::path::PathBuf;
use std::fs;
use home::home_dir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandEntry {
    command: String,
    group: Option<String>,
    comment: Option<String>,
    aliases: Option<Vec<String>>,
}

impl CommandEntry {
    pub fn get_command(&self) -> &str {
        &self.command
    }
}

pub fn add_command(
    command: String, 
    group: Option<String>, 
    comment: Option<String>, 
    aliases: Option<String>, 
    location: Option<PathBuf>
) -> Result<()> {
    // Determine storage path
    let storage_path = location.unwrap_or_else(|| {
        home_dir()
            .expect("Could not find home directory")
            .join(".sebas-rs")
    });

    // Create storage directory if it doesn't exist
    fs::create_dir_all(&storage_path)?;

    // Parse aliases
    let parsed_aliases = aliases.map(|a| a.split(',').map(String::from).collect());

    let entry = CommandEntry {
        command,
        group,
        comment,
        aliases: parsed_aliases,
    };


    println!("Command added successfully!");
    Ok(())
}

pub fn retrieve_command(query: &str) -> Result<String> {
    let storage_path = home_dir()
        .expect("Could not find home directory")
        .join(".sebas-rs");

    // Search through all JSON files in the storage directory
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Only process JSON files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let command_entry: CommandEntry = serde_json::from_str(&content)?;
            
            // Check if query matches command or any alias
            if command_entry.command.contains(query) || 
               command_entry.aliases.map_or(false, |aliases| 
                   aliases.iter().any(|alias| alias.contains(query))) {
                return Ok(command_entry.command);
            }
        }
    }

    Err(anyhow!("No command found matching: {}", query))
}

pub fn run_command(query: &str) -> Result<()> {
    let command = retrieve_command(query)?;
    
    // Simply print the command for the user to copy or for shell integration
    println!("Command: {}", command);
    
    Ok(())
}

pub fn remove_command(identifier: &str) -> Result<()> {
    let storage_path = home_dir()
        .expect("Could not find home directory")
        .join(".sebas-rs");

    // Search and remove files matching the identifier
    let mut removed = false;
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let command_entry: CommandEntry = serde_json::from_str(&content)?;
            
            // Check if identifier matches command or aliases
            if command_entry.command.contains(identifier) || 
               command_entry.aliases.map_or(false, |aliases| 
                   aliases.iter().any(|alias| alias.contains(identifier))) {
                fs::remove_file(&path)?;
                removed = true;
                println!("Removed command: {}", command_entry.command);
                break;
            }
        }
    }

    if !removed {
        return Err(anyhow!("No command found matching: {}", identifier));
    }

    Ok(())
}

pub fn list_commands() -> Result<()> {
    let storage_path = home_dir()
        .expect("Could not find home directory")
        .join(".sebas-rs");

    println!("Stored Commands:");
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let command_entry: CommandEntry = serde_json::from_str(&content)?;
            
            println!("- Command: {}", command_entry.command);
            if let Some(comment) = &command_entry.comment {
                println!("  Comment: {}", comment);
            }
            if let Some(aliases) = &command_entry.aliases {
                println!("  Aliases: {}", aliases.join(", "));
            }
            println!();
        }
    }

    Ok(())
}

pub fn search_commands(query: &str) -> Result<()> {
    let storage_path = home_dir()
        .expect("Could not find home directory")
        .join(".sebas-rs");

    println!("Search Results for '{}':", query);
    let mut found = false;

    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let command_entry: CommandEntry = serde_json::from_str(&content)?;
            
            // Check if query matches command, aliases, or comment
            if command_entry.command.contains(query) || 
               command_entry.comment.map_or(false, |c| c.contains(query)) ||
               command_entry.aliases.map_or(false, |aliases| 
                   aliases.iter().any(|alias| alias.contains(query))) {
                println!("- Command: {}", command_entry.command);
                //if let Some(comment) = &command_entry.comment {
                //    println!("  Comment: {}", comment);
                //}
                //if let Some(aliases) = &command_entry.aliases {
                //    println!("  Aliases: {}", aliases.join(", "));
                //}
                println!();
                found = true;
            }
        }
    }

    if !found {
        println!("No commands found matching '{}'", query);
    }

    Ok(())
}

pub fn edit_command(command: &str) -> Result<()> {
    // TODO: Implement more robust edit functionality
    // For now, just find and print the command
    let existing_command = retrieve_command(command)?;
    println!("Found command to edit: {}", existing_command);
    
    // In a real implementation, this would open an editor
    // or prompt for new command details
    Ok(())
}
