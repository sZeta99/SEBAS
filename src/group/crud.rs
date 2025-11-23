use crate::group::alias::GroupAlias;
use crate::group::error::CRUDGroupError;
use crate::group::Group;
use serde_yaml;
use std::fs;
use std::path::{Path, PathBuf};

pub trait GroupCRUD: Sized {
    fn load_group(group_path: &PathBuf) -> Result<Self, CRUDGroupError>;
    fn save_group(&self, group_path: &PathBuf) -> Result<(), CRUDGroupError>;
    fn delete_group(group_path: &PathBuf) -> Result<(), CRUDGroupError>;
    fn exists(path: &Path) -> Result<bool, String>;
}
// TODO: Working on deciding is the write responability is of the alias or the Group using alias
impl GroupCRUD for Group {
    fn load_group(group_path: &PathBuf) -> Result<Self, CRUDGroupError> {
        // Require the file to exist
        if !group_path.exists() {
            return Err(CRUDGroupError::FileNotFound(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }
        // TODO: The loading of the group must be done manualy or the structure of the GroupAlias mast
        // be changed
        let yaml_content = fs::read_to_string(group_path)?;
        let group: GroupAlias = serde_yaml::from_str(&yaml_content)?;

        Ok(group)
    }

    fn save_group(&self, group_path: &PathBuf) -> Result<(), CRUDGroupError> {
        // Require the parent folder to exist, do NOT create it
        if let Some(parent) = group_path.parent() {
            if !parent.exists() {
                return Err(CRUDGroupError::InvalidDirectory(
                    parent.to_str().unwrap_or("InvalidDirectory").to_string(),
                ));
            }
        } else {
            return Err(CRUDGroupError::InvalidPath(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }

        // It is allowed to overwrite any file in an existing directory (matches 'create' semantics)
        let yaml_content = serde_yaml::to_string(self)
            .map_err(|e| CRUDGroupError::SerdeYamlError(e.to_string()))?;
        std::fs::write(group_path, yaml_content)
            .map_err(|e| CRUDGroupError::IoError(e.to_string()))?;
        Ok(())
    }

    fn delete_group(group_path: &PathBuf) -> Result<(), CRUDGroupError> {
        // Require the file to exist
        if !group_path.exists() {
            return Err(CRUDGroupError::FileNotFound(
                group_path.to_str().unwrap_or("InvalidPath").to_string(),
            ));
        }
        fs::remove_file(group_path).map_err(|e| CRUDGroupError::IoError(e.to_string()))?;
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
    fn test_load_group() -> Result<(), CRUDGroupError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Create a sample group file
        let group = GroupAlias::default(); // Assuming `Group` has a `default` method
        fs::write(&file_path, serde_yaml::to_string(&group)?)?;

        // Test loading
        let loaded_group = GroupAlias::load_group(&file_path)?;
        assert_eq!(loaded_group, group);

        // Test loading with a non-existent file
        let non_existent_path = dir.path().join("non_existent.yaml");
        let result = GroupAlias::load_group(&non_existent_path);
        assert!(matches!(result, Err(CRUDGroupError::FileNotFound(_))));

        Ok(())
    }

    #[test]
    fn test_create_group() -> Result<(), CRUDGroupError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Test creating a group
        let group = GroupAlias::default();
        group.save_group(&file_path)?;

        // Verify the file was created
        assert!(file_path.exists());

        // Test creating in a non-existent directory
        let invalid_path = dir.path().join("nonexistent_dir/group.yaml");
        let result = group.save_group(&invalid_path);
        assert!(matches!(result, Err(CRUDGroupError::InvalidDirectory(_))));

        Ok(())
    }

    #[test]
    fn test_delete_group() -> Result<(), CRUDGroupError> {
        let dir = tempdir()?; // Create a temporary directory
        let file_path = dir.path().join("group.yaml");

        // Create a sample group file
        let group = GroupAlias::default();
        fs::write(&file_path, serde_yaml::to_string(&group)?)?;

        // Test deleting the group
        GroupAlias::delete_group(&file_path)?;
        assert!(!file_path.exists());

        // Test deleting a non-existent file
        let result = GroupAlias::delete_group(&file_path);
        assert!(matches!(result, Err(CRUDGroupError::FileNotFound(_))));

        Ok(())
    }
}
