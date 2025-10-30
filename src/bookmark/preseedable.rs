use colored::Colorize;
use std::process::{Command, Stdio};

use crate::bookmark::Bookmark;

/// Trait for pre-seeding and executing commands
pub trait Preseedable {
    fn preseed(&mut self) -> Result<(), std::io::Error>;
}

impl Preseedable for Bookmark {
    /// Preseed logic for handling comments and executing commands
    fn preseed(&mut self) -> Result<(), std::io::Error> {
        // Show additional context if available
        if let Some(comment) = self.get_comment() {
            println!("{} {}", "Comment:".bright_blue(), comment.dimmed());
        }

        // Log the bookmark's name and ID
        println!(
            "{}: {}",
            "Executing Bookmark".bright_white(),
            self.name.bright_yellow()
        );
        println!();

        // Execute the command
        Command::new("bash")
            .arg("-c")
            .arg(&self.command)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;

        // Update last used timestamp after execution
        self.update_last_used();

        Ok(())
    }
}
