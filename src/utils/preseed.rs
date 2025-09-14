use std::process::{Command, Stdio};

use colored::Colorize;

use crate::commands::core::definition::ResolvedCommand;

pub fn preseed(resolved_command: ResolvedCommand) -> Result<(), std::io::Error> {
    // Show additional context if available
    if let Some(comment) = &resolved_command.command.comment {
        println!("{} {}", "Comment:".bright_blue(), comment.dimmed());
    }
    let sanitized_path = resolved_command.folder_path.to_str().unwrap_or_default().replace(".sebas", "");
    println!("{} {}", "Path:".bright_yellow(), sanitized_path.dimmed());
    println!(); // Empty line before execution



    Command::new("bash")
        .arg("-c")
        .arg(format!("echo '{}' | perl -e 'ioctl STDOUT, 0x5412, $_ for split //, do{{ chomp($_ = <>); $_ }}'", resolved_command.command.command))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;


    Ok(())

}
