use thiserror::Error;

#[derive(Error, Debug)]
pub enum Md2TomlError {
    #[error("YAML parsing failed: {0}")]
    YamlParseError(#[from] serde_yaml::Error),

    #[error("Invalid frontmatter format: {0}")]
    InvalidFrontmatter(String),

    #[error("Path operation failed: {0}")]
    PathError(String),

    #[error("IO operation failed")]
    IoError,

    #[error("TOML generation failed: {0}")]
    TomlError(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, Md2TomlError>;
