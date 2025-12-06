//! Application Layer - Use Case Orchestration
//!
//! Coordinates domain logic with infrastructure ports and implements business workflows.

pub mod directory_conversion_use_case;
pub mod error_classifier;

// Re-export public types for convenience
pub use directory_conversion_use_case::StandardDirectoryConversionUseCase;
pub use error_classifier::{ErrorCategory, ErrorClassifier};

use crate::domain::errors::{Md2TomlError, Result};
use crate::domain::ports::{DirectoryConversionUseCase, FileSystem, Logger};
use crate::domain::{
    detect_input_type, extract_content, generate_toml, parse_frontmatter, validate_required_fields,
    ConversionStats, InputType, Md2TomlConfig,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Orchestrates conversion workflows by coordinating domain logic
/// with infrastructure adapters.
pub struct Md2TomlService {
    fs: Arc<dyn FileSystem>,
    directory_converter: Arc<dyn DirectoryConversionUseCase>,
    logger: Arc<dyn Logger>,
    config: Md2TomlConfig,
}

impl Md2TomlService {
    /// Create new service with dependency injection
    pub fn new(
        fs: Arc<dyn FileSystem>,
        directory_converter: Arc<dyn DirectoryConversionUseCase>,
        logger: Arc<dyn Logger>,
        config: Md2TomlConfig,
    ) -> Self {
        Self {
            fs,
            directory_converter,
            logger,
            config,
        }
    }

    /// Convert a single file to TOML format
    fn convert_single_file(&self, input: PathBuf, output: Option<PathBuf>) -> Result<()> {
        self.logger.info("Starting file conversion");

        let content = self.fs.read_to_string(&input)?;
        self.logger
            .debug(&format!("Read file: {:?}", input.display()));

        let metadata = parse_frontmatter(&content)?;
        validate_required_fields(&metadata)?;

        let content = extract_content(&content)?;

        let processed_content = crate::domain::apply_all_replacements_simple(&content);

        let toml_output = generate_toml(&metadata, &processed_content)?;

        let output_path = output.unwrap_or_else(|| {
            let mut path = input.clone();
            path.set_extension("toml");
            path
        });

        self.logger
            .debug(&format!("Writing to: {:?}", output_path.display()));

        self.fs.write(&output_path, &toml_output)?;

        self.logger.info("File conversion completed");

        Ok(())
    }

    /// Convert input (file or directory) based on detection
    pub fn convert(&self, input: PathBuf, output: Option<PathBuf>) -> Result<ConversionStats> {
        self.logger
            .info(&format!("Starting conversion: {:?}", input.display()));

        let result = match detect_input_type(&input) {
            InputType::File(file_path) => {
                self.convert_single_file(file_path, output)?;
                Ok(ConversionStats {
                    total: 1,
                    success: 1,
                    skipped: 0,
                    failed: Vec::new(),
                })
            }
            InputType::Directory(dir_path) => {
                let output_dir = match output {
                    Some(path) => path,
                    None => self.default_output_dir(&dir_path)?,
                };

                // Prevent accidental overwrites unless explicitly enabled
                if self.fs.exists(&output_dir) && !self.config.overwrite {
                    return Err(Md2TomlError::Config(format!(
                        "Output directory {:?} already exists. Use --overwrite to enable.",
                        output_dir
                    )));
                }

                let stats = self.directory_converter.convert_directory(
                    &dir_path,
                    &output_dir,
                    self.config.verbose,
                )?;

                Ok(stats)
            }
            InputType::Invalid(msg) => Err(Md2TomlError::InvalidFrontmatter(msg)),
        };

        if let Ok(ref stats) = result {
            self.logger.info(&format!(
                "Conversion completed: {} success, {} failed",
                stats.success,
                stats.failed.len()
            ));
        } else if let Err(ref err) = result {
            self.logger.error(&format!("Conversion failed: {}", err));
        }

        result
    }

    /// Generate default output directory name
    fn default_output_dir(&self, input_dir: &Path) -> Result<PathBuf> {
        let dir_name = input_dir
            .file_name()
            .ok_or_else(|| Md2TomlError::PathError("Cannot determine directory name".to_string()))?
            .to_str()
            .ok_or_else(|| {
                Md2TomlError::PathError("Directory name contains non-UTF8 characters".to_string())
            })?;

        Ok(PathBuf::from(format!("{}_toml", dir_name)))
    }
}

/// Factory for creating service instances with dependency injection
pub struct Md2TomlServiceBuilder {
    fs: Option<Arc<dyn FileSystem>>,
    directory_converter: Option<Arc<dyn DirectoryConversionUseCase>>,
    logger: Option<Arc<dyn Logger>>,
    config: Md2TomlConfig,
}

impl Md2TomlServiceBuilder {
    pub fn new() -> Self {
        Self {
            fs: None,
            directory_converter: None,
            logger: None,
            config: Md2TomlConfig::default(),
        }
    }

    pub fn with_filesystem(mut self, fs: Arc<dyn FileSystem>) -> Self {
        self.fs = Some(fs);
        self
    }

    pub fn with_directory_converter(
        mut self,
        converter: Arc<dyn DirectoryConversionUseCase>,
    ) -> Self {
        self.directory_converter = Some(converter);
        self
    }

    pub fn with_logger(mut self, logger: Arc<dyn Logger>) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn with_config(mut self, config: Md2TomlConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> Result<Md2TomlService> {
        let fs = self.fs.expect("FileSystem must be provided");
        let directory_converter = self
            .directory_converter
            .expect("DirectoryConversionUseCase must be provided");
        let logger = self.logger.expect("Logger must be provided");

        // Configuration validation occurs during config construction/modification
        Ok(Md2TomlService::new(
            fs,
            directory_converter,
            logger,
            self.config,
        ))
    }
}

impl Default for Md2TomlServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::{DirectoryConversionUseCase, FileSystem, LogLevel, Logger};

    struct MockFileSystem;
    impl FileSystem for MockFileSystem {
        fn read_to_string(&self, _path: &Path) -> Result<String> {
            Ok("---\ntitle: Test\ndescription: Test\n---\n\nTest content".to_string())
        }
        fn write(&self, _path: &Path, _content: &str) -> Result<usize> {
            Ok(0)
        }
        fn copy(&self, _from: &Path, _to: &Path) -> Result<u64> {
            Ok(0)
        }
        fn create_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn exists(&self, _path: &Path) -> bool {
            true
        }
        fn is_file(&self, _path: &Path) -> bool {
            true
        }
        fn is_directory(&self, _path: &Path) -> bool {
            true
        }
        fn remove_file(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn remove_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn metadata(&self, _path: &Path) -> Result<crate::domain::ports::FileMetadata> {
            Ok(crate::domain::ports::FileMetadata::new(0, false, None))
        }
        fn list_files_with_extension(&self, _dir: &Path, _extension: &str) -> Result<Vec<PathBuf>> {
            Ok(Vec::new())
        }
    }

    struct MockDirectoryConverter;
    impl DirectoryConversionUseCase for MockDirectoryConverter {
        fn convert_directory(
            &self,
            _src_dir: &Path,
            _dest_dir: &Path,
            _verbose: bool,
        ) -> Result<ConversionStats> {
            Ok(ConversionStats {
                total: 2,
                success: 2,
                skipped: 0,
                failed: Vec::new(),
            })
        }
    }

    struct MockLogger;
    impl Logger for MockLogger {
        fn info(&self, _msg: &str) {}
        fn warn(&self, _msg: &str) {}
        fn error(&self, _msg: &str) {}
        fn debug(&self, _msg: &str) {}
        fn log(&self, _level: LogLevel, _msg: &str, _ctx: Option<&str>) {}
        fn is_debug_enabled(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_service_builder() {
        let builder = Md2TomlServiceBuilder::new()
            .with_filesystem(Arc::new(MockFileSystem))
            .with_directory_converter(Arc::new(MockDirectoryConverter))
            .with_logger(Arc::new(MockLogger))
            .with_config(Md2TomlConfig::default());

        let service = builder.build().unwrap();
        assert!(service
            .directory_converter
            .convert_directory(Path::new("."), Path::new("test"), false)
            .is_ok());
    }
}
