pub mod alias; // Module for alias-related functionality

pub mod crud; // Module for CRUD (Create, Read, Update, Delete) operations

pub mod error; // Module defining custom errors

// Importing the necessary crates
use chrono::{DateTime, Utc}; // For handling date and time
use serde::{Deserialize, Serialize}; // For serializing and deserializing data structures
use std::collections::HashMap; // For using hash maps

// Importing specific types from other modules
use crate::bookmark::Bookmark;
use crate::group::alias::GroupAlias;
// Bookmark type from the bookmark module
use crate::group::error::GroupError; // GroupError type from the error module

/**
 * Represents a group of bookmarks in the system.
 * This structure contains a name and a collection of bookmarks.
 */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Group {
    name: String,                         // Name of the group
    bookmarks: HashMap<String, Bookmark>, // Collection of bookmarks identified by name
    created_at: DateTime<Utc>,            // Timestamp for when the group was created
    modified_at: DateTime<Utc>,           // Timestamp for the last modification
}

impl Group {
    /// Creates a new `Group` instance with a specified name.
    /// Returns an error if the name is empty.
    pub fn new(name: String) -> Result<Self, GroupError> {
        if name.is_empty() {
            return Err(GroupError::EmptyName); // Error for empty name
        }

        let now = Utc::now(); // Get the current time

        // Return a new Group instance
        Ok(Group {
            name,
            bookmarks: HashMap::new(), // Initialize an empty bookmarks HashMap
            created_at: now,           // Set the creation time
            modified_at: now,          // Set the modification time
        })
    }

    /// Adds a `Bookmark` to the group.
    /// Returns an error if the bookmark already exists.
    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> Result<(), GroupError> {
        if self.bookmarks.contains_key(bookmark.get_name()) {
            return Err(GroupError::BookmarkAlreadyExists(
                bookmark.get_name().clone(),
            )); // Error for duplicate bookmark
        }

        self.bookmarks.insert(bookmark.get_name().clone(), bookmark); // Add bookmark
        self.update_modified_time(); // Update modification time
        Ok(())
    }

    /// Removes a `Bookmark` from the group by its name.
    /// Returns an error if the bookmark is not found.
    pub fn remove_bookmark(&mut self, bookmark_name: String) -> Result<(), GroupError> {
        if self.bookmarks.remove(&bookmark_name).is_none() {
            return Err(GroupError::BookmarkNotFound(bookmark_name)); // Error if bookmark not found
        }

        self.update_modified_time(); // Update modification time
        Ok(())
    }

    /// Retrieves all bookmarks in the group.
    pub fn get_bookmarks(&self) -> &HashMap<String, Bookmark> {
        &self.bookmarks // Return reference to bookmarks
    }

    /// Retrieves a single bookmark by its ID.
    pub fn get_bookmark(&self, bookmark_id: &str) -> Option<&Bookmark> {
        self.bookmarks.get(bookmark_id) // Return the bookmark if it exists
    }

    /// Returns the name of the group.
    pub fn get_name(&self) -> &String {
        &self.name // Return reference to the group's name
    }

    /// Returns the modified time of the group.
    pub fn get_modified_time(&self) -> &DateTime<Utc> {
        &self.modified_at // Return reference to the modified timestamp
    }

    /// Returns the created time of the group.
    pub fn get_created_time(&self) -> &DateTime<Utc> {
        &self.created_at // Return reference to the created timestamp
    }

    /// Updates the `modified_at` timestamp to the current time.
    pub fn update_modified_time(&mut self) {
        self.modified_at = Utc::now(); // Set modified_at to current time
    }
}

// Unit tests for the Group struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_new_empty_name() {
        // Test creating a Group with an empty name
        let result = Group::new("".to_string());
        assert!(
            matches!(result, Err(GroupError::EmptyName)),
            "Empty name should raise GroupError::EmptyName"
        );
    }

    #[test]
    fn test_add_duplicate_bookmark() {
        // Test adding a duplicate bookmark
        let mut group = Group::new("Test Group".to_string()).unwrap();
        let bookmark = Bookmark::new(
            "Test Bookmark".to_string(),
            "echo Hello".to_string(),
            None,
            None,
        )
        .unwrap();

        group.add_bookmark(bookmark.clone()).unwrap(); // Add initial bookmark
        let result = group.add_bookmark(bookmark); // Try to add duplicate bookmark

        assert!(
            matches!(result, Err(GroupError::BookmarkAlreadyExists(_))),
            "Duplicate bookmark should raise GroupError::BookmarkAlreadyExists"
        );
    }

    #[test]
    fn test_remove_nonexistent_bookmark() {
        // Test removing a bookmark that doesn't exist
        let mut group = Group::new("Test Group".to_string()).unwrap();
        let bookmark_id = "invalid_id".to_string();
        let result = group.remove_bookmark(bookmark_id); // Attempt to remove by invalid ID

        assert!(
            matches!(result, Err(GroupError::BookmarkNotFound(_))),
            "Nonexistent bookmark should raise GroupError::BookmarkNotFound"
        );
    }
}
