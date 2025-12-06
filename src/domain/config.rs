/// Configuration structure for md2toml conversion
#[derive(Debug, Clone)]
pub struct Md2TomlConfig {
    pub verbose: bool,
    pub skip_hidden: bool,
    pub output_format: OutputFormat,
    pub overwrite: bool,
    pub parallel: bool,
    pub worker_threads: Option<usize>,
}

impl Default for Md2TomlConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            skip_hidden: true,
            output_format: OutputFormat::Toml,
            overwrite: false,
            parallel: false,
            worker_threads: None,
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

    /// Validate configuration constraints
    pub fn validate(&self) -> Result<(), super::errors::Md2TomlError> {
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
