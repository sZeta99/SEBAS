use std::fs;
use std::io;
use std::path::PathBuf;

use crate::context::Context;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    IoError(io::Error),
    InvalidDirectory,
    // Add other error variants as needed
}
pub trait FileSystem {
    fn load_bookmarks(&self, directory_path: &PathBuf) -> Result<Vec<String>, FileSystemError>;
}

impl FileSystem for Context {
    /// Load bookmarks from a specified directory path.
    fn load_bookmarks(&self, directory_path: &PathBuf) -> Result<Vec<String>, FileSystemError> {
        // Check if the path is a valid directory
        if !directory_path.is_dir() {
            return Err(FileSystemError::InvalidDirectory);
        }

        // Read the directory contents
        let entries = fs::read_dir(directory_path).map_err(FileSystemError::IoError)?;

        let mut bookmarks = Vec::new();

        for entry in entries {
            let entry = entry.map_err(FileSystemError::IoError)?;
            let path = entry.path();

            // Filter files with the .bookmark extension
            if path.is_file()
                && path.extension().and_then(|ext| ext.to_str())
                    == Some(&self.config.format.to_string())
            {
                // Read the file contents and add to the bookmarks list
                let contents = fs::read_to_string(&path).map_err(FileSystemError::IoError)?;
                bookmarks.push(contents);
            }
        }

        Ok(bookmarks)
    }
}
