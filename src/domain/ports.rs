use crate::domain::errors::Result;
use crate::domain::models::{ConversionResult, ConversionStats};
use std::path::Path;
use std::path::PathBuf;

/// Domain-level file metadata abstraction
///
/// Abstracts file system metadata to prevent infrastructure leakage.
/// This structure provides only the information needed by domain logic.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub len: u64,
    pub is_dir: bool,
    pub modified: Option<std::time::SystemTime>,
}

impl FileMetadata {
    /// Create new file metadata
    pub fn new(len: u64, is_dir: bool, modified: Option<std::time::SystemTime>) -> Self {
        Self {
            len,
            is_dir,
            modified,
        }
    }
}

/// File system operations port
///
/// This trait abstracts all file system operations to enable:
/// - Dependency inversion (test with mock implementations)
/// - Different storage backends (local FS, cloud storage, etc.)
/// - Better testability
pub trait FileSystem: Send + Sync {
    /// Read entire file content as string
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Write string content to file
    fn write(&self, path: &Path, content: &str) -> Result<usize>;

    /// Copy file from source to destination
    fn copy(&self, from: &Path, to: &Path) -> Result<u64>;

    /// Create directory including all parent directories
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Check if path exists
    fn exists(&self, path: &Path) -> bool;

    /// Check if path is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Check if path is a directory
    fn is_directory(&self, path: &Path) -> bool;

    /// Remove file
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// Remove directory recursively
    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    /// Get file metadata
    fn metadata(&self, path: &Path) -> Result<FileMetadata>;

    /// List all files in a directory with given extension
    fn list_files_with_extension(&self, dir: &Path, extension: &str) -> Result<Vec<PathBuf>>;
}

/// Logger port for structured logging
///
/// Abstracts logging to enable:
/// - Different output formats (JSON, text, etc.)
/// - Multiple sinks (file, console, syslog)
/// - Structured logging with contexts
pub trait Logger: Send + Sync {
    /// Log info level message
    fn info(&self, message: &str);

    /// Log warning level message
    fn warn(&self, message: &str);

    /// Log error level message
    fn error(&self, message: &str);

    /// Log debug level message (only if debug is enabled)
    fn debug(&self, message: &str);

    /// Log with custom level and context
    fn log(&self, level: LogLevel, message: &str, context: Option<&str>);

    /// Indicate if debug logging is enabled
    fn is_debug_enabled(&self) -> bool;
}

/// Log message structure
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
    pub context: Option<String>,
    pub timestamp: std::time::Instant,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: String, context: Option<String>) -> Self {
        Self {
            level,
            message,
            context,
            timestamp: std::time::Instant::now(),
        }
    }
}

/// Log level enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Repository port for conversion statistics
///
/// Enables persistence and retrieval of conversion results
pub trait ConversionRepository: Send + Sync {
    /// Save conversion result
    fn save_result(&self, result: &ConversionResult) -> Result<()>;

    /// Get all results for a batch
    fn get_batch_results(&self, batch_id: &str) -> Result<Vec<ConversionResult>>;

    /// Clear all results
    fn clear(&self) -> Result<()>;
}

/// Configuration loader port
///
/// Abstracts configuration loading from various sources
pub trait ConfigLoader: Send + Sync {
    /// Load configuration from file
    fn load_config(&self, path: &Path) -> Result<crate::domain::config::Md2TomlConfig>;

    /// Load configuration from environment
    fn load_from_env(&self) -> Result<crate::domain::config::Md2TomlConfig>;
}

/// Regex service port for text pattern matching
///
/// Abstracts regex operations to keep domain layer pure
/// and allow different implementations (regex, manual parsing, etc.)
pub trait RegexPort: Send + Sync {
    /// Replace all occurrences of backtick pattern !`command` with !{command}
    fn replace_backtick_patterns(&self, content: &str) -> String;
}

/// Directory conversion use case port
///
/// Abstracts directory conversion operations to keep Application layer pure.
/// This is a use case that Infrastructure implements.
pub trait DirectoryConversionUseCase: Send + Sync {
    /// Convert all files in a directory structure
    fn convert_directory(
        &self,
        src_dir: &Path,
        dest_dir: &Path,
        verbose: bool,
    ) -> Result<ConversionStats>;
}

/// Directory explorer port
///
/// Abstracts directory traversal to enable:
/// - Dependency inversion (test with mock implementations)
/// - Different directory sources (e.g., virtual file systems)
pub trait DirectoryExplorer: Send + Sync {
    /// Recursively find all markdown files in a directory.
    /// It should ignore dotfiles/dotfolders and follow .gitignore rules by default.
    fn discover_markdown_files(&self, root: &Path) -> Result<Vec<PathBuf>>;
}
