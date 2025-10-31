use colored::Colorize;
use std::process::{Command, Stdio};

use crate::bookmark::Bookmark;

/// Trait for pre-seeding and executing commandsa
pub trait PreseedStrategy {
    fn preseed(&mut self) -> Result<(), std::io::Error>;
}
pub struct BashPreseedStrategy;

// TODO: have preseed based on system, a strategy patter may be beneficial, this is not a correct
// strategy implementation, contex shoul be move as to not require Bokkmark method to work
impl PreseedStrategy for Bookmark {
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
            .arg(format!("echo '{}' | perl -e 'ioctl STDOUT, 0x5412, $_ for split //, do{{ chomp($_ = <>); $_ }}'", &self.command))
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
