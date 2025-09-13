use anyhow::{anyhow, Context, Error};
use colored::Colorize;
use std::io::Write;
use std::process::{Command as CliCommand, Stdio};
use crate::commands::core::definition::ResolvedCommand;
use crate::utils::preseed::preseed;

pub fn sebas_fzf_run(commands: Vec<ResolvedCommand>) -> Result<(), Error> {
    // Create a vector of display strings for fzf
    let mut display_items = Vec::new();
    // Create a vector to store comments and paths for preview
    let mut preview_items = Vec::new();

    // Build the display list with group and command, and prepare preview data
    for (index, cmd) in commands.iter().enumerate() {
        // Create the display string without comment and path
        let display = format!(
            "[{}] {} | {}", 
            format!("{:1}", index + 1), 
            cmd.group, 
            cmd.command.command
        );
        display_items.push(display);

        // Create the preview string with comment and sanitized path
        let comment = cmd.command.comment.clone().unwrap_or_default();
        let sanitized_path = cmd.folder_path.to_str().unwrap_or_default().replace(".sebas", "");
        let preview = format!(
            "Comment:{}  ||| Path:{}", 
            comment, 
            sanitized_path
        );
        preview_items.push(preview);
    }

    // Join all display items with newlines for fzf input
    let fzf_input = display_items.join("\n");
    // Create a temporary file for storing preview data
    let preview_file = "/tmp/sebas_fzf_preview.txt";
    let mut preview_writer = std::fs::File::create(preview_file)?;
    for preview in preview_items {
        writeln!(preview_writer, "{}", preview)?;
    }

    // Step 1: Launch fzf with enhanced preview and styling
    let mut fzf = CliCommand::new("fzf")
        .arg("--height=60%")
        .arg("--border=rounded")
        .arg("--prompt=SEBAS Commands › ")
        .arg("--header=Select a command to retrieve")
        .arg("--header-lines=0")
        .arg("--info=inline")
        .arg("--layout=reverse")
        .arg("--multi=0")
        .arg("--cycle")
        .arg("--bind=enter:accept")
        .arg("--bind=esc:abort")
        .arg("--bind=ctrl-c:abort")
        // Updated preview to use the preview file
        .arg("--preview=awk NR==$(echo {} | awk '{print $1}' | tr -d '[]') /tmp/sebas_fzf_preview.txt | fold -w 50")
        .arg("--preview-window=right:30%:wrap")
        .arg("--color=fg:#f8f8f2,bg:#282a36,hl:#8be9fd")
        .arg("--color=fg+:#f8f8f2,bg+:#44475a,hl+:#8be9fd")
        .arg("--color=info:#ffb86c,prompt:#50fa7b,pointer:#ff79c6")
        .arg("--color=marker:#ff79c6,spinner:#ffb86c,header:#6272a4")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to start fzf - make sure fzf is installed")?;

    // Step 2: Write the commands to fzf's stdin
    if let Some(mut stdin) = fzf.stdin.take() {
        stdin
            .write_all(fzf_input.as_bytes())
            .context("Failed to write to fzf stdin")?;
    }

    // Step 3: Get the selected command from fzf's stdout
    let fzf_output = fzf
        .wait_with_output()
        .context("Failed to get fzf output")?;

    if !fzf_output.status.success() {
        return Err(anyhow!("No command selected or fzf was cancelled"));
    }

    let selected_display = String::from_utf8(fzf_output.stdout)
        .context("Failed to parse fzf output")?
        .trim()
        .to_string();

    // Step 4: Extract just the command from the selected display string
    // The format is "[index] group | Command"
    let selected_command = selected_display
        .split("| ")
        .nth(1)
        .unwrap_or(&selected_display)
        .trim()
        .to_string();

    // Step 5: Find the matching ResolvedCommand for additional context
    let resolved_command = commands
        .iter()
        .find(|cmd| cmd.command.command == selected_command)
        .ok_or_else(|| anyhow!("Selected command not found in original list"))?;

    // Show additional context if available
    if let Some(comment) = &resolved_command.command.comment {
        println!("{} {}", "Comment:".bright_blue(), comment.dimmed());
    }
    let sanitized_path = resolved_command.folder_path.to_str().unwrap_or_default().replace(".sebas", "");
    println!("{} {}", "Path:".bright_yellow(), sanitized_path.dimmed());
    println!(); // Empty line before execution

    // Step 6: Execute only the actual command (not the display string)
    let _ = preseed(&selected_command);

    Ok(())
}
