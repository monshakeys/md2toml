#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    UserInput,
    System,
    Transient,
}

pub struct ErrorClassifier;

impl ErrorClassifier {
    /// Create a new error classifier
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, error: &crate::domain::Md2TomlError) -> ErrorCategory {
        match error {
            crate::domain::Md2TomlError::InvalidFrontmatter(_) => ErrorCategory::UserInput,
            crate::domain::Md2TomlError::Validation(_) => ErrorCategory::UserInput,
            crate::domain::Md2TomlError::Config(_) => ErrorCategory::UserInput,

            crate::domain::Md2TomlError::YamlParseError(_) => ErrorCategory::System,
            crate::domain::Md2TomlError::IoError => ErrorCategory::System,
            crate::domain::Md2TomlError::PathError(_) => ErrorCategory::System,
            crate::domain::Md2TomlError::FileSystem(_) => ErrorCategory::System,

            crate::domain::Md2TomlError::TomlError(_) => ErrorCategory::Transient,
        }
    }

    pub fn exit_code(&self, category: ErrorCategory) -> i32 {
        match category {
            ErrorCategory::UserInput => 2,
            ErrorCategory::System => 1,
            ErrorCategory::Transient => 3,
        }
    }
}

impl Default for ErrorClassifier {
    fn default() -> Self {
        Self::new()
    }
}
