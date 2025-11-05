use crate::group::Group;
use serde_yaml;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum CRUDError {
    FileNotFound(String),
    ContextNotFound(String),
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
    InvalidDirectory(String),
    InvalidPath(String),
}

impl From<io::Error> for CRUDError {
    fn from(e: io::Error) -> Self {
        CRUDError::IoError(e)
    }
}

impl From<serde_yaml::Error> for CRUDError {
    fn from(e: serde_yaml::Error) -> Self {
        CRUDError::SerdeYamlError(e)
    }
}

pub trait GroupCRUD: Sized {
    fn load_group(group_path: &PathBuf) -> Result<Self, CRUDError>;
    fn create_group(&self, group_path: &PathBuf) -> Result<(), CRUDError>;
    fn delete_group(group_path: &PathBuf) -> Result<(), CRUDError>;
    fn exists(path: &Path) -> Result<bool, String>;
}

impl GroupCRUD for Group {
    fn load_group(group_path: &PathBuf) -> Result<Self, CRUDError> {
        // Require the file to exist
        if !group_path.exists() {
            return Err(CRUDError::FileNotFound(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }
        let yaml_content = fs::read_to_string(group_path)?;
        let group: Group = serde_yaml::from_str(&yaml_content)?;
        Ok(group)
    }

    fn create_group(&self, group_path: &PathBuf) -> Result<(), CRUDError> {
        // Require the parent folder to exist, do NOT create it
        if let Some(parent) = group_path.parent() {
            if !parent.exists() {
                return Err(CRUDError::InvalidDirectory(
                    parent.to_str().unwrap_or("InvalidDirectory").to_string(),
                ));
            }
        } else {
            return Err(CRUDError::InvalidPath(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }

        // It is allowed to overwrite any file in an existing directory (matches 'create' semantics)
        let yaml_content = serde_yaml::to_string(self).map_err(CRUDError::SerdeYamlError)?;
        std::fs::write(group_path, yaml_content).map_err(CRUDError::IoError)?;
        Ok(())
    }

    fn delete_group(group_path: &PathBuf) -> Result<(), CRUDError> {
        // Require the file to exist
        if !group_path.exists() {
            return Err(CRUDError::FileNotFound(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }
        fs::remove_file(group_path).map_err(CRUDError::IoError)?;
        Ok(())
    }

    // TODO: Improve this exist inluding the correct format pass it to it
    fn exists(path: &Path) -> Result<bool, String> {
        if !path.exists() {
            return Err(format!("Path '{}' does not exist.", path.display()));
        }

        if !path.is_file() && !path.is_dir() {
            return Err(format!(
                "Path '{}' is neither a file nor a directory.",
                path.display()
            ));
        }

        Ok(true)
    }
}
#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    #[test]
    fn test_load_group() -> Result<(), CRUDError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Create a sample group file
        let group = Group::default(); // Assuming `Group` has a `default` method
        fs::write(&file_path, serde_yaml::to_string(&group)?)?;

        // Test loading
        let loaded_group = Group::load_group(&file_path)?;
        assert_eq!(loaded_group, group);

        // Test loading with a non-existent file
        let non_existent_path = dir.path().join("non_existent.yaml");
        let result = Group::load_group(&non_existent_path);
        assert!(matches!(result, Err(CRUDError::FileNotFound(_))));

        Ok(())
    }

    #[test]
    fn test_create_group() -> Result<(), CRUDError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Test creating a group
        let group = Group::default();
        group.create_group(&file_path)?;

        // Verify the file was created
        assert!(file_path.exists());

        // Test creating in a non-existent directory
        let invalid_path = dir.path().join("nonexistent_dir/group.yaml");
        let result = group.create_group(&invalid_path);
        assert!(matches!(result, Err(CRUDError::InvalidDirectory(_))));

        Ok(())
    }

    #[test]
    fn test_delete_group() -> Result<(), CRUDError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Create a sample group file
        let group = Group::default();
        fs::write(&file_path, serde_yaml::to_string(&group)?)?;

        // Test deleting the group
        Group::delete_group(&file_path)?;
        assert!(!file_path.exists());

        // Test deleting a non-existent file
        let result = Group::delete_group(&file_path);
        assert!(matches!(result, Err(CRUDError::FileNotFound(_))));

        Ok(())
    }

    #[test]
    fn test_modify_group() -> Result<(), CRUDError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Create a sample group file
        let mut group = Group::default();
        fs::write(&file_path, serde_yaml::to_string(&group)?)?;

        // Modify the group
        group.name = "ModifiedGroup".to_string(); // Assuming `Group` has a `name` field
        group.create_group(&file_path)?;

        // Verify the file was modified
        let loaded_group = Group::load_group(&file_path)?;
        assert_eq!(loaded_group.name, "ModifiedGroup");

        Ok(())
    }
}

