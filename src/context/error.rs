use std::io;

#[derive(Debug)]
pub enum CRUDContextError {
    FileNotFound(String),
    ContextNotFound(String),
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
    InvalidDirectory(String),
    InvalidPath(String),
}

impl From<io::Error> for CRUDContextError {
    fn from(e: io::Error) -> Self {
        CRUDContextError::IoError(e)
    }
}
impl From<serde_yaml::Error> for CRUDContextError {
    fn from(e: serde_yaml::Error) -> Self {
        CRUDContextError::SerdeYamlError(e)
    }
}
