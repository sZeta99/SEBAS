use std::{fmt, io};

// -------- GROUP ERROR ---------
// TODO: IMPLEMENT thiserror and anyhow
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

// Implement ToString for GroupError using Display
impl std::string::ToString for GroupError {
    fn to_string(&self) -> String {
        self.to_string() // Calls the Display implementation
    }
}

// -------- CRUD ERROR ---------

#[derive(Debug)]
pub enum CRUDGroupError {
    FileNotFound(String),
    ContextNotFound(String),
    IoError(String),
    SerdeYamlError(String),
    InvalidDirectory(String),
    InvalidPath(String),
    AliasFaild(String),
}

impl fmt::Display for CRUDGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CRUDGroupError::FileNotFound(filename) => {
                write!(f, "File '{}' not found.", filename)
            }
            CRUDGroupError::ContextNotFound(context) => {
                write!(f, "Context '{}' not found.", context)
            }
            CRUDGroupError::IoError(err) => {
                write!(f, "I/O error: {}", err)
            }
            CRUDGroupError::SerdeYamlError(err) => {
                write!(f, "YAML error: {}", err)
            }
            CRUDGroupError::InvalidDirectory(dir) => {
                write!(f, "Invalid directory: '{}'", dir)
            }
            CRUDGroupError::InvalidPath(path) => {
                write!(f, "Invalid path: '{}'", path)
            }
            CRUDGroupError::AliasFaild(alias) => {
                write!(f, "Alias creation failed for '{}'.", alias)
            }
        }
    }
}

// Implement ToString for CRUDGroupError using Display
impl std::string::ToString for CRUDGroupError {
    fn to_string(&self) -> String {
        self.to_string() // Calls the Display implementation
    }
}
