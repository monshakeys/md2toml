#[derive(Debug, Clone)]
pub struct Md2TomlConfig {
    pub verbose: bool,
    pub skip_hidden: bool,
    pub output_format: OutputFormat,
    pub overwrite: bool,
}

impl Default for Md2TomlConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            skip_hidden: true,
            output_format: OutputFormat::Toml,
            overwrite: false,
        }
    }
}

impl Md2TomlConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn with_skip_hidden(mut self, skip: bool) -> Self {
        self.skip_hidden = skip;
        self
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn validate(&self) -> Result<(), super::errors::Md2TomlError> {
        Ok(())
    }
}

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
