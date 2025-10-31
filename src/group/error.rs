use std::fmt;

#[derive(Debug, Clone)]
pub enum GroupError {
    EmptyName,
    BookmarkNotFound(String),
    BookmarkAlreadyExists(String),
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::EmptyName => write!(f, "Group name cannot be empty."),
            GroupError::BookmarkNotFound(id) => {
                write!(f, "Bookmark with ID '{}' not found in the group.", id)
            }
            GroupError::BookmarkAlreadyExists(id) => {
                write!(f, "Bookmark with ID '{}' already exists in the group.", id)
            }
        }
    }
}
