use crate::domain::errors::Result;
use crate::domain::models::ConversionStats;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub len: u64,
    pub is_dir: bool,
    pub modified: Option<std::time::SystemTime>,
}

impl FileMetadata {
    pub fn new(len: u64, is_dir: bool, modified: Option<std::time::SystemTime>) -> Self {
        Self {
            len,
            is_dir,
            modified,
        }
    }
}

pub trait FileSystem: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String>;

    fn write(&self, path: &Path, content: &str) -> Result<usize>;

    fn copy(&self, from: &Path, to: &Path) -> Result<u64>;

    fn create_dir_all(&self, path: &Path) -> Result<()>;

    fn exists(&self, path: &Path) -> bool;

    fn is_file(&self, path: &Path) -> bool;

    fn is_directory(&self, path: &Path) -> bool;

    fn remove_file(&self, path: &Path) -> Result<()>;

    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    fn metadata(&self, path: &Path) -> Result<FileMetadata>;

    fn list_files_with_extension(&self, dir: &Path, extension: &str) -> Result<Vec<PathBuf>>;
}

pub trait Logger: Send + Sync {
    fn info(&self, message: &str);

    fn warn(&self, message: &str);

    fn error(&self, message: &str);

    fn debug(&self, message: &str);

    fn log(&self, level: LogLevel, message: &str, context: Option<&str>);

    fn is_debug_enabled(&self) -> bool;
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

pub trait ConfigLoader: Send + Sync {
    fn load_config(&self, path: &Path) -> Result<crate::domain::config::Md2TomlConfig>;

    fn load_from_env(&self) -> Result<crate::domain::config::Md2TomlConfig>;
}

pub trait RegexPort: Send + Sync {
    fn replace_backtick_patterns(&self, content: &str) -> String;
}

pub trait DirectoryConversionUseCase: Send + Sync {
    fn convert_directory(
        &self,
        src_dir: &Path,
        dest_dir: &Path,
        verbose: bool,
    ) -> Result<ConversionStats>;
}
