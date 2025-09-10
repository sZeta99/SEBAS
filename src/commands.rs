use anyhow::{Result, Context, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use std::env;
use colored::*;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use std::io::{self, stdout, Write};
use std::process::Command;

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

// New function to load all commands
fn load_all_commands(location: Option<PathBuf>) -> Result<Vec<(String, Vec<CommandEntry>)>> {
    let storage_path = get_storage_path(location);
    let mut all_groups = Vec::new();

    // Check if directory exists
    if !storage_path.exists() {
        return Ok(Vec::new());
    }

    // Read all yaml files
    for entry in fs::read_dir(&storage_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Only process YAML files
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = fs::read_to_string(&path)?;
            let group_entry: GroupEntry = serde_yaml::from_str(&content)?;
            
            // Sort commands within the group
            let mut sorted_commands = group_entry.commands;
            sorted_commands.sort_by(|a, b| a.command.cmp(&b.command));
            
            all_groups.push((group_entry.group, sorted_commands));
        }
    }

    // Sort groups
    all_groups.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
    
    Ok(all_groups)
}

pub fn list_commands(location: Option<PathBuf>) -> Result<()> {
    let all_groups = load_all_commands(location.clone())?;

    if all_groups.is_empty() {
        println!("{}", "No commands stored yet.".yellow());
        return Ok(());
    }

    // Create a vector of display strings and corresponding commands
    let mut display_items = Vec::new();
    let mut command_map = Vec::new();

    // Build the display list and command mapping
    for (group_name, commands) in &all_groups {
        display_items.push(format!("{}: ", group_name.bold().green()));
        command_map.push(None); // Group header has no command
        
        for (index, command) in commands.iter().enumerate() {
            let mut display = format!("{:2}. {}", index + 1, command.command.cyan());
            
            // Aliases (if any)
            if let Some(aliases) = &command.aliases {
                display.push_str(&format!(" {}", format!("[{}]", aliases.join(", ")).dimmed()));
            }
            
            // Comment (if any)
            if let Some(comment) = &command.comment {
                display.push_str(&format!(" - {}", comment.italic().dimmed()));
            }
            
            display_items.push(display);
            command_map.push(Some(command.clone()));
        }
    }

    let total_commands = command_map.iter().filter(|cmd| cmd.is_some()).count();

    // Setup terminal
     let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    let stdin = io::stdin();
    let mut keys = stdin.keys();
    
    let mut selected_index = 1; // Start with the first actual command
    
    // Find the first actual command if we're not already on one
    while selected_index < command_map.len() && command_map[selected_index].is_none() {
        selected_index += 1;
    }

    // Main interaction loop
    loop {
        // Clear screen and reset cursor
        write!(screen, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
        writeln!(screen, "{}\r{}", "Stored Commands:".bold().blue(),"===============".blue())?;
        
        // Display items with highlighting for the selected one
        for (i, item) in display_items.iter().enumerate() {
            if i == selected_index {
                writeln!(screen, "{} <<<", item.bold().on_blue())?;
            } else {
                writeln!(screen, "{}", item)?;
            }
        }
        
        // Instructions at the bottom
        writeln!(screen, "{}{}\r{}", total_commands.to_string().bold().blue(), " Total Command." ,"Use ↑/↓ keys to navigate, Enter to execute, q to quit".yellow())?;
        screen.flush()?;
        
        // Get user key input
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Esc => {
                    break;
                },
                Key::Char('\n') => {
                    // Execute selected command if it exists
                    if let Some(cmd_entry) = &command_map[selected_index] {
                        // Return to normal terminal mode temporarily
                        drop(screen);
                        //drop(stdout);
                        
                        println!("\nExecuting: {}", cmd_entry.command.green());
                        
                        // Execute the command
                        match execute_command(&cmd_entry.command) {
                            Ok(_) => println!("Command executed successfully."),
                            Err(e) => println!("Error executing command: {}", e),
                        }
                        
                        println!("Press Enter to continue...");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        
                        // Return to raw mode
                        //screen = stdout()
                        //       .into_raw_mode()
                        //       .unwrap()
                        //       .into_alternate_screen()
                        //      .unwrap();
                        
                    }
                    break;
                },
                Key::Down | Key::Char('j') => {
                    // Move down
                    selected_index = find_next_command(selected_index, &command_map, true);
                },
                Key::Up | Key::Char('k') => {
                    // Move up
                    selected_index = find_next_command(selected_index, &command_map, false);
                },
                _ => {}
            }
        }
    }

    Ok(())
}

// Helper function to find the next or previous command
fn find_next_command(current: usize, command_map: &[Option<CommandEntry>], forward: bool) -> usize {
    let len = command_map.len();
    
    if forward {
        // Find next command
        for i in 1..len {
            let idx = (current + i) % len;
            if command_map[idx].is_some() {
                return idx;
            }
        }
    } else {
        // Find previous command
        for i in 1..len {
            let idx = (current + len - i) % len;
            if command_map[idx].is_some() {
                return idx;
            }
        }
    }
    
    current // Return current if no other command found
}

// Execute a command using the shell
fn execute_command(cmd: &str) -> Result<()> {
    #[cfg(target_family = "unix")]
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()?;

    #[cfg(target_family = "windows")]
    let output = Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .status()?;

    if output.success() {
        Ok(())
    } else {
        Err(anyhow!("Command exited with non-zero status: {:?}", output))
    }
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
