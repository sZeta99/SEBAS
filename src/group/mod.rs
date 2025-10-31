pub mod error;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::bookmark::Bookmark;
use crate::group::error::GroupError;

#[derive(Debug, Clone)]
pub struct Group {
    name: String,
    bookmarks: HashMap<String, Bookmark>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Group {
    /// Creates a new `Group` instance
    pub fn new(name: String) -> Result<Self, GroupError> {
        if name.is_empty() {
            return Err(GroupError::EmptyName);
        }

        let now = Utc::now();
        Ok(Group {
            name,
            bookmarks: HashMap::new(),
            created_at: now,
            modified_at: now,
        })
    }

    /// Adds a `Bookmark` to the group
    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> Result<(), GroupError> {
        if self.bookmarks.contains_key(bookmark.get_name()) {
            return Err(GroupError::BookmarkAlreadyExists(
                bookmark.get_name().clone(),
            ));
        }
        self.bookmarks.insert(bookmark.get_name().clone(), bookmark);
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
    pub fn get_bookmarks(&self) -> &HashMap<String, Bookmark> {
        &self.bookmarks
    }

    /// Retrieves a single bookmark by ID
    pub fn get_bookmark(&self, bookmark_id: &str) -> Option<&Bookmark> {
        self.bookmarks.get(bookmark_id)
    }

    /// Returns the name of the group
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the modified time of the group
    pub fn get_modified_time(&self) -> &DateTime<Utc> {
        &self.modified_at
    }

    /// Returns the created time of the group
    pub fn get_created_time(&self) -> &DateTime<Utc> {
        &self.created_at
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
        let result = Group::new("".to_string());
        assert!(
            matches!(result, Err(GroupError::EmptyName)),
            "Empty name should raise GroupError::EmptyName"
        );
    }

    #[test]
    fn test_add_duplicate_bookmark() {
        let mut group = Group::new("Test Group".to_string()).unwrap();
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
        let mut group = Group::new("Test Group".to_string()).unwrap();
        let bookmark_id = "invalid_id".to_string();

        let result = group.remove_bookmark(bookmark_id);

        assert!(
            matches!(result, Err(GroupError::BookmarkNotFound(_))),
            "Nonexistent bookmark should raise GroupError::BookmarkNotFound"
        );
    }
}
