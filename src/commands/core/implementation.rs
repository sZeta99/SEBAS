use std::{fs, io::{self, Read, Write}};
use std::process::Command as ProcessCommand;
use sha2::{Digest, Sha256};
use crate::{commands::core::definition::{CommandGroup, ResolvedCommand}, utils::dir::{find_sebas_dir, get_all_sebas_dirs}, SebasApp};
impl CommandGroup {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl SebasApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sebas_dir = find_sebas_dir()
            .ok_or("No .sebas folder found. Run 'sebas init' to create one.")?;
        Ok(Self { sebas_dir })
    }

    pub fn generate_hash(command: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(command.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)[..8].to_string()
    }

    pub fn get_command_from_stdin() -> Option<String> {
        if atty::is(atty::Stream::Stdin) {
            return None;
        }
        
        let mut input = String::new();
        if io::stdin().read_to_string(&mut input).is_ok() {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    pub fn get_last_shell_command() -> Option<String> {
        let output = ProcessCommand::new("bash")
            .arg("-c")
            .arg("fc -ln -1")
            .output()
            .ok()?;
        
        let command = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if command.is_empty() || command.starts_with("sebas") {
            None
        } else {
            Some(command)
        }
    }

    pub fn confirm(prompt: &str) -> bool {
        print!("{} (y/N) ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    pub fn resolve_all_commands(&self) -> Vec<ResolvedCommand> {
        let mut resolved = Vec::new();
        let dirs = get_all_sebas_dirs();
        
        for dir in dirs {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                            if let Some(group_name) = path.file_stem().and_then(|s| s.to_str()) {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    if let Ok(group) = serde_yaml::from_str::<CommandGroup>(&content) {
                                        for (cmd_idx, cmd) in group.commands.iter().enumerate() {
                                            resolved.push(ResolvedCommand {
                                                command: cmd.clone(),
                                                group: group_name.to_string(),
                                                folder_path: dir.clone(),
                                                index: resolved.len() + 1,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        resolved
    }

    pub fn find_command_by_identifier(&self, identifier: &str) -> Option<ResolvedCommand> {
        let resolved = self.resolve_all_commands();
        
        // Try to parse as index
        if let Ok(index) = identifier.parse::<usize>() {
            return  resolved.get(index - 1).cloned();
        }
        
        // Try to find by hash
        resolved.into_iter().find(|cmd| cmd.command.hash.starts_with(identifier))
    }
}






