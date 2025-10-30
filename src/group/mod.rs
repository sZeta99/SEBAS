pub mod error;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::bookmark::Bookmark;
use crate::group::error::GroupError;

#[derive(Debug, Clone)]
pub struct Group {
    id: String,
    name: String,
    bookmarks: HashMap<String, Bookmark>,
    path: PathBuf,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Group {
    /// Creates a new `Group` instance
    pub fn new(name: String, path: PathBuf) -> Result<Self, String> {
        if name.is_empty() {
            return Err(GroupError::EmptyName.to_string());
        }

        let id = Group::generate_id(&name);

        let now = Utc::now();
        Ok(Group {
            id,
            name,
            bookmarks: HashMap::new(),
            path,
            created_at: now,
            modified_at: now,
        })
    }

    /// Generates a unique, deterministic ID for the group (based on name)
    fn generate_id(name: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();
        result.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    /// Adds a `Bookmark` to the group
    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> Result<(), GroupError> {
        if self.bookmarks.contains_key(bookmark.get_id()) {
            return Err(GroupError::BookmarkAlreadyExists(bookmark.get_id().clone()));
        }
        self.bookmarks.insert(bookmark.get_id().clone(), bookmark);
        self.update_modified_time();
        Ok(())
    }

    /// Removes a `Bookmark` from the group by its ID

    pub fn remove_bookmark(&mut self, bookmark_id: String) -> Result<(), GroupError> {
        if self.bookmarks.remove(&bookmark_id).is_none() {
            return Err(GroupError::BookmarkNotFound(bookmark_id));
        }
        self.update_modified_time();
        Ok(())
    }

    /// Retrieves all bookmarks in the group
    pub fn get_bookmarks(&self) -> HashMap<String, Bookmark> {
        self.bookmarks.clone()
    }

    /// Retrieves a single bookmark by ID
    pub fn get_bookmark(&self, bookmark_id: &str) -> Option<Bookmark> {
        self.bookmarks.get(bookmark_id).cloned()
    }

    /// Returns the name of the group
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the ID of the group
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Returns the path of the group
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Returns the modified time of the group
    pub fn get_modified_time(&self) -> DateTime<Utc> {
        self.modified_at.clone()
    }
    /// Returns the created time of the group
    pub fn get_created_time(&self) -> DateTime<Utc> {
        self.created_at.clone()
    }

    /// Updates the `modified_at` timestamp
    pub fn update_modified_time(&mut self) {
        self.modified_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_new_empty_name() {
        let result = Group::new("".to_string(), PathBuf::new());
        assert!(
            matches!(result, Err(GroupError::EmptyName)),
            "Empty name should raise GroupError::EmptyName"
        );
    }

    #[test]
    fn test_add_duplicate_bookmark() {
        let mut group = Group::new("Test Group".to_string(), PathBuf::new()).unwrap();
        let bookmark = Bookmark::new(
            "Test Bookmark".to_string(),
            "echo Hello".to_string(),
            None,
            None,
        )
        .unwrap();

        group.add_bookmark(bookmark.clone()).unwrap();
        let result = group.add_bookmark(bookmark);

        assert!(
            matches!(result, Err(GroupError::BookmarkAlreadyExists(_))),
            "Duplicate bookmark should raise GroupError::BookmarkAlreadyExists"
        );
    }

    #[test]
    fn test_remove_nonexistent_bookmark() {
        let mut group = Group::new("Test Group".to_string(), PathBuf::new()).unwrap();
        let bookmark_id = "invalid_id".to_string();

        let result = group.remove_bookmark(bookmark_id);

        assert!(
            matches!(result, Err(GroupError::BookmarkNotFound(_))),
            "Nonexistent bookmark should raise GroupError::BookmarkNotFound"
        );
    }
}
