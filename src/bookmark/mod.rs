pub mod preseedable;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};

/// Represents an individual bookmark with associated metadata
#[derive(Debug, Clone)]
pub struct Bookmark {
    id: String,
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

        let id = Bookmark::generate_id(&name, &command);

        Ok(Bookmark {
            id,
            name,
            command,
            comment,
            description,
            created_at: SystemTime::now(),
            last_used: None,
        })
    }

    /// Generates a unique, deterministic ID for the bookmark using SHA256
    fn generate_id(name: &str, command: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", name, command));
        let result = hasher.finalize();
        result.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    /// Updates the last_used timestamp
    pub fn update_last_used(&mut self) {
        self.last_used = Some(SystemTime::now());
    }

    /// Returns detailed information about the bookmark
    pub fn get_info(&self) -> String {
        format!(
            "Bookmark Info:\n  ID: {}\n  Name: {}\n  Command: {}\n  Comment: {}\n  Description: {:?}\n  Created At: {:?}\n  Last Used: {:?}",
            self.id,
            self.name,
            self.command,
            self.comment.clone().unwrap_or_else(|| "None".to_string()),
            self.description.clone().unwrap_or_else(|| "None".to_string()),
            self.created_at,
            self.last_used
        )
    }

    /// Returns the unique ID of the bookmark
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Returns the command string
    pub fn get_command(&self) -> &String {
        &self.command
    }

    /// Returns the comment string, if available
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
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
    fn test_generate_id() {
        let name = "Test Name";
        let command = "echo Hello";
        let id = Bookmark::generate_id(name, command);

        // Compute the expected hash manually
        let expected_id = {
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", name, command));
            let result = hasher.finalize();
            result
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>()
        };

        assert_eq!(
            id, expected_id,
            "Generated ID does not match the expected hash"
        );
    }

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
        assert_eq!(bookmark.comment, description);
        assert_eq!(
            bookmark.id,
            Bookmark::generate_id(name, command),
            "Generated ID does not match"
        );
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

    #[test]
    fn test_new_same_id_for_same_inputs() {
        let name = "Common Name";
        let command = "echo Common Command";

        let bookmark1 = Bookmark::new(name.to_string(), command.to_string(), None, None).unwrap();
        let bookmark2 = Bookmark::new(name.to_string(), command.to_string(), None, None).unwrap();

        // Ensure the same inputs result in the same ID
        assert_eq!(
            bookmark1.id, bookmark2.id,
            "Generated IDs should match for identical name and command"
        );
    }

    #[test]
    fn test_new_different_id_for_different_inputs() {
        let name1 = "Name One";
        let command1 = "echo Command One";

        let name2 = "Name Two";
        let command2 = "echo Command Two";

        let bookmark1 = Bookmark::new(name1.to_string(), command1.to_string(), None, None).unwrap();
        let bookmark2 = Bookmark::new(name2.to_string(), command2.to_string(), None, None).unwrap();

        // Ensure different inputs generate different IDs
        assert_ne!(
            bookmark1.id, bookmark2.id,
            "Generated IDs should not match for different name and command"
        );
    }
}

