use thiserror::Error;

#[derive(Error, Debug)]
pub enum Md2TomlError {
    #[error("Failed to parse YAML frontmatter: {0}")]
    YamlParseError(#[from] serde_yaml::Error),

    #[error("Invalid frontmatter format: {0}")]
    InvalidFrontmatter(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML generation error: {0}")]
    TomlError(String),
}

pub type Result<T> = std::result::Result<T, Md2TomlError>;
