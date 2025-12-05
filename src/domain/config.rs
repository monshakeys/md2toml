/// Configuration structure for md2toml conversion
///
/// This defines the shape of configuration data without any I/O logic.
#[derive(Debug, Clone)]
pub struct Md2TomlConfig {
    /// Whether to enable verbose output
    pub verbose: bool,

    /// Whether to skip files starting with ~ or .
    pub skip_hidden: bool,

    /// Maximum file size to process (in bytes)
    pub max_file_size: Option<u64>,

    /// Output format (TOML is the only supported format currently)
    pub output_format: OutputFormat,

    /// Whether to overwrite existing files
    pub overwrite: bool,

    /// Whether to enable parallel processing
    pub parallel: bool,

    /// Number of worker threads for parallel processing
    pub worker_threads: Option<usize>,

    /// Custom replacements to apply
    pub custom_replacements: Vec<ReplacementRule>,
}

impl Default for Md2TomlConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            skip_hidden: true,
            max_file_size: None,
            output_format: OutputFormat::Toml,
            overwrite: false,
            parallel: false,
            worker_threads: None,
            custom_replacements: Vec::new(),
        }
    }
}

impl Md2TomlConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Set max file size
    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = Some(size);
        self
    }

    /// Set skip hidden files flag
    pub fn with_skip_hidden(mut self, skip: bool) -> Self {
        self.skip_hidden = skip;
        self
    }

    /// Set overwrite flag
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Enable parallel processing
    pub fn parallel(mut self, worker_threads: Option<usize>) -> Self {
        self.parallel = true;
        self.worker_threads = worker_threads;
        self
    }

    /// Enable overwriting existing files
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// Add custom replacement rule
    pub fn add_replacement(mut self, pattern: String, replacement: String) -> Self {
        self.custom_replacements.push(ReplacementRule {
            pattern,
            replacement,
        });
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), super::errors::Md2TomlError> {
        if let Some(size) = self.max_file_size {
            if size == 0 {
                return Err(super::errors::Md2TomlError::Config(
                    "Max file size must be greater than 0".to_string(),
                ));
            }
        }

        if self.parallel {
            if let Some(threads) = self.worker_threads {
                if threads == 0 {
                    return Err(super::errors::Md2TomlError::Config(
                        "Worker threads must be greater than 0".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Output format enumeration
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Toml,
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Toml => "toml",
        }
    }
}

/// Custom replacement rule
#[derive(Debug, Clone)]
pub struct ReplacementRule {
    pub pattern: String,
    pub replacement: String,
}

impl ReplacementRule {
    pub fn new(pattern: String, replacement: String) -> Self {
        Self {
            pattern,
            replacement,
        }
    }
}
