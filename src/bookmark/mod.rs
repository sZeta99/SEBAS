pub mod preseedable;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// Represents an individual bookmark with associated metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bookmark {
    name: String,
    command: String,
    comment: Option<String>,
    description: Option<String>,
    created_at: SystemTime,
    last_used: Option<SystemTime>,
}

impl Bookmark {
    /// Creates a new Bookmark instance
    pub fn new(
        name: String,
        command: String,
        comment: Option<String>,
        description: Option<String>,
    ) -> Result<Self, String> {
        if name.is_empty() || command.is_empty() {
            return Err("Name and command cannot be empty.".to_string());
        }

        Ok(Bookmark {
            name,
            command,
            comment,
            description,
            created_at: SystemTime::now(),
            last_used: None,
        })
    }

    /// Updates the last_used timestamp
    pub fn update_last_used(&mut self) {
        self.last_used = Some(SystemTime::now());
    }

    /// Returns detailed information about the bookmark
    pub fn get_info(&self) -> String {
        format!(
            "Bookmark Info:\n Name: {}\n  Command: {}\n  Comment: {}\n  Description: {:?}\n  Created At: {:?}\n  Last Used: {:?}",
            self.name,
            self.command,
            self.comment.clone().unwrap_or_else(|| "None".to_string()),
            self.description.clone().unwrap_or_else(|| "None".to_string()),
            self.created_at,
            self.last_used
        )
    }

    /// Returns the command string
    pub fn get_command(&self) -> &String {
        &self.command
    }

    /// Returns the comment string, if available
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Returns the Name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns the last used timestamp (if any)
    pub fn get_last_used(&self) -> Option<Duration> {
        self.last_used
            .map(|time| time.elapsed().unwrap_or(Duration::new(0, 0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_new_success() {
        let name = "Test Bookmark";
        let command = "echo Hello World";
        let comment = Some("This is a test.".to_string());
        let description = Some("This is a test description long.".to_string());

        let bookmark = Bookmark::new(
            name.to_string(),
            command.to_string(),
            comment.clone(),
            description.clone(),
        );

        // Assert the Bookmark is created successfully
        assert!(bookmark.is_ok(), "Bookmark creation failed");

        let bookmark = bookmark.unwrap();

        // Check field values
        assert_eq!(bookmark.name, name);
        assert_eq!(bookmark.command, command);
        assert_eq!(bookmark.comment, comment);
        assert_eq!(bookmark.description, description);
        assert!(
            bookmark.last_used.is_none(),
            "Last used should initially be None"
        );
        assert!(
            bookmark.created_at <= SystemTime::now(),
            "Created at timestamp should not be in the future"
        );
    }

    #[test]
    fn test_new_failure_empty_name() {
        let name = ""; // Empty name
        let command = "echo Hello";

        let bookmark = Bookmark::new(name.to_string(), command.to_string(), None, None);

        // Assert that Bookmark creation fails
        assert!(
            bookmark.is_err(),
            "Bookmark creation should fail with an empty name"
        );

        if let Err(err) = bookmark {
            assert_eq!(
                err, "Name and command cannot be empty.",
                "Error message does not match for empty name"
            );
        }
    }

    #[test]
    fn test_new_failure_empty_command() {
        let name = "Test Bookmark";
        let command = ""; // Empty command

        let bookmark = Bookmark::new(name.to_string(), command.to_string(), None, None);

        // Assert that Bookmark creation fails
        assert!(
            bookmark.is_err(),
            "Bookmark creation should fail with an empty command"
        );

        if let Err(err) = bookmark {
            assert_eq!(
                err, "Name and command cannot be empty.",
                "Error message does not match for empty command"
            );
        }
    }
}
