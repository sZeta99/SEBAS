/// Enum representing different formats for bookmark data.
pub enum Format {
    //Json,
    Yaml,
    //Toml,
}

impl Format {
    /// Converts the Format enum variant to its string representation.
    pub fn to_string(&self) -> String {
        match self {
            //Format::Json => "json".to_string(),
            Format::Yaml => "yaml".to_string(),
            //Format::Toml => "toml".to_string(),
        }
    }
}
