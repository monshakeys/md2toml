use std::collections::BTreeMap;
use std::path::PathBuf;

/// Parsed markdown structure containing metadata and content
///
/// Uses `BTreeMap` for automatic alphabetical sorting of metadata keys.
pub struct ParsedMarkdown {
    pub metadata: BTreeMap<String, String>,
    pub content: String,
}

impl ParsedMarkdown {
    pub fn new(metadata: BTreeMap<String, String>, content: String) -> Self {
        Self { metadata, content }
    }
}

/// File type enumeration for processing strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Markdown,
    Text,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Markdown => "md",
            FileType::Text => "txt",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "md" => Some(FileType::Markdown),
            "txt" => Some(FileType::Text),
            _ => None,
        }
    }
}

/// Processing strategy for different file types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessingStrategy {
    Convert, // Convert markdown to TOML (md -> toml)
    Copy,    // Copy file as-is (txt -> txt)
}

impl ProcessingStrategy {
    pub fn from_file_type(file_type: FileType) -> Self {
        match file_type {
            FileType::Markdown => ProcessingStrategy::Convert,
            FileType::Text => ProcessingStrategy::Copy,
        }
    }

    pub fn output_extension(&self) -> &'static str {
        match self {
            ProcessingStrategy::Convert => "toml",
            ProcessingStrategy::Copy => "txt",
        }
    }
}

/// Tracks conversion statistics for batch operations
#[derive(Debug, Default, Clone)]
pub struct ConversionStats {
    pub total: usize,
    pub success: usize,
    pub skipped: usize,
    pub failed: Vec<(PathBuf, String)>,
}

impl ConversionStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self) {
        self.success += 1;
    }

    pub fn add_failure(&mut self, path: PathBuf, error: &str) {
        self.failed.push((path, error.to_string()));
    }

    pub fn add_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }

    pub fn completion_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.success as f64 / self.total as f64) * 100.0
        }
    }
}

/// Input type detection result
#[derive(Debug, PartialEq)]
pub enum InputType {
    File(PathBuf),
    Directory(PathBuf),
    Invalid(String),
}

impl InputType {
    pub fn is_file(&self) -> bool {
        matches!(self, InputType::File(_))
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, InputType::Directory(_))
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, InputType::Invalid(_))
    }

    pub fn into_path_buf(self) -> Option<PathBuf> {
        match self {
            InputType::File(path) | InputType::Directory(path) => Some(path),
            InputType::Invalid(_) => None,
        }
    }
}

/// Conversion result for a single file
#[derive(Debug)]
pub struct ConversionResult {
    pub source_path: PathBuf,
    pub dest_path: PathBuf,
    pub success: bool,
    pub error: Option<String>,
    pub bytes_written: usize,
}

impl ConversionResult {
    pub fn new(source_path: PathBuf, dest_path: PathBuf) -> Self {
        Self {
            source_path,
            dest_path,
            success: false,
            error: None,
            bytes_written: 0,
        }
    }

    pub fn success(mut self, bytes_written: usize) -> Self {
        self.success = true;
        self.bytes_written = bytes_written;
        self
    }

    pub fn failure(mut self, error: String) -> Self {
        self.success = false;
        self.error = Some(error);
        self
    }
}