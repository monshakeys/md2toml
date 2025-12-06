//! Error Classification Service
//!
//! Provides error categorization services for the Application layer.
//! Separates domain error semantics from presentation concerns.

/// Error categories for presentation layer decision making
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// User-facing errors that need friendly messages
    UserInput,
    /// System errors that need technical details
    System,
    /// Temporary errors that can be retried
    Transient,
}

/// Error classifier service
pub struct ErrorClassifier;

impl ErrorClassifier {
    /// Create a new error classifier
    pub fn new() -> Self {
        Self
    }

    /// Classify a domain error into a presentation category
    pub fn classify(&self, error: &crate::domain::Md2TomlError) -> ErrorCategory {
        match error {
            // User input errors - frontend should show friendly messages
            crate::domain::Md2TomlError::InvalidFrontmatter(_) => ErrorCategory::UserInput,
            crate::domain::Md2TomlError::Validation(_) => ErrorCategory::UserInput,
            crate::domain::Md2TomlError::Config(_) => ErrorCategory::UserInput,

            // System errors - show technical details
            crate::domain::Md2TomlError::YamlParseError(_) => ErrorCategory::System,
            crate::domain::Md2TomlError::IoError => ErrorCategory::System,
            crate::domain::Md2TomlError::PathError(_) => ErrorCategory::System,
            crate::domain::Md2TomlError::FileSystem(_) => ErrorCategory::System,

            // Transient errors - can retry
            crate::domain::Md2TomlError::TomlError(_) => ErrorCategory::Transient,
        }
    }

    /// Get recommended exit code for CLI based on error category
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
