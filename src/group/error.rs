use std::{fmt, io};

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

// -------- CRUD ERROR ---------
#[derive(Debug)]
pub enum CRUDGroupError {
    FileNotFound(String),
    ContextNotFound(String),
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
    InvalidDirectory(String),
    InvalidPath(String),
}

impl From<io::Error> for CRUDGroupError {
    fn from(e: io::Error) -> Self {
        CRUDGroupError::IoError(e)
    }
}

impl From<serde_yaml::Error> for CRUDGroupError {
    fn from(e: serde_yaml::Error) -> Self {
        CRUDGroupError::SerdeYamlError(e)
    }
}
