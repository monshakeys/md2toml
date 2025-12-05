// Application Layer - Use Case Orchestration
//
// Coordinates domain logic with infrastructure ports (traits)
// Implements business workflows and transaction control.

use crate::domain::errors::{Md2TomlError, Result};
use crate::domain::ports::{FileSystem, Logger};
use crate::domain::{
    apply_all_replacements_simple, extract_content, generate_toml, parse_frontmatter,
    validate_required_fields, ConversionStats, InputType, Md2TomlConfig,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Main application service for md2toml conversion
///
/// Orchestrates use cases and coordinates between domain logic
/// and infrastructure adapters.
pub struct Md2TomlService {
    fs: Arc<Box<dyn FileSystem>>,
    logger: Arc<dyn Logger>,
    config: Md2TomlConfig,
}

impl Md2TomlService {
    /// Create new service with dependency injection using Arc for sharing
    pub fn new(
        fs: Arc<Box<dyn FileSystem>>,
        logger: Arc<dyn Logger>,
        config: Md2TomlConfig,
    ) -> Self {
        Self { fs, logger, config }
    }

    /// Convert a single file to TOML format
    fn convert_single_file(&self, input: PathBuf, output: Option<PathBuf>) -> Result<()> {
        self.logger.info("Starting file conversion");

        // Read input file
        let input_content = self.fs.read_to_string(&input)?;
        self.logger
            .debug(&format!("Read file: {:?}", input.display()));

        // Parse metadata and content
        let metadata = parse_frontmatter(&input_content)?;
        validate_required_fields(&metadata)?;

        let content = extract_content(&input_content)?;

        // Apply replacements
        let processed_content = apply_all_replacements_simple(&content);

        // Generate TOML
        let toml_output = generate_toml(&metadata, &processed_content)?;

        // Determine output path
        let output_path = output.unwrap_or_else(|| {
            let mut path = input.clone();
            path.set_extension("toml");
            path
        });

        self.logger
            .debug(&format!("Writing to: {:?}", output_path.display()));

        // Write output file
        self.fs.write(&output_path, &toml_output)?;

        self.logger.info("File conversion completed");

        Ok(())
    }

    /// Convert input (file or directory) based on detection
    pub fn convert(&self, input: PathBuf, output: Option<PathBuf>) -> Result<ConversionStats> {
        self.logger
            .info(&format!("Starting conversion: {:?}", input.display()));

        // Use pure domain function - no I/O validation in Application layer
        // Validation should happen in Presentation layer before calling this method
        let result = match crate::domain::detect_input_type(&input) {
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

                // Application layer does NOT handle directory traversal or I/O
                // It only validates the configuration and delegates to domain logic
                // Directory conversion is handled by Presentation layer via dependency injection

                // Validate overwrite configuration at application layer
                if self.fs.exists(&output_dir) && !self.config.overwrite {
                    return Err(Md2TomlError::Config(format!(
                        "Output directory {:?} already exists. Use --overwrite to enable.",
                        output_dir
                    )));
                }

                // Pure domain logic: directory conversion statistics
                // Actual I/O operations are delegated to infrastructure through Presentation layer
                let stats = self.convert_directory_with_injected_fs(&dir_path, &output_dir)?;

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

    /// Convert directory using injected file system (for testing)
    fn convert_directory_with_injected_fs(
        &self,
        src_dir: &Path,
        dest_dir: &Path,
    ) -> Result<ConversionStats> {
        // This method is a bridge between Application and Infrastructure layers
        // It uses the injected FileSystem port to perform directory conversion
        // This is acceptable because FileSystem is a domain port, not infrastructure-specific

        let mut stats = ConversionStats::new();

        // Collect markdown and text files using the FileSystem port
        let md_files = self.fs.list_files_with_extension(src_dir, "md")?;
        let txt_files = self.fs.list_files_with_extension(src_dir, "txt")?;

        let all_files: Vec<(PathBuf, &str)> = md_files
            .iter()
            .map(|p| (p.clone(), "md"))
            .chain(txt_files.iter().map(|p| (p.clone(), "txt")))
            .collect();

        stats.total = all_files.len();

        if all_files.is_empty() {
            return Err(Md2TomlError::InvalidFrontmatter(format!(
                "No supported files found in directory: {:?}",
                src_dir
            )));
        }

        // Process each file
        for (src_path, ext) in all_files {
            let relative_path = src_path.strip_prefix(src_dir).map_err(|_| {
                Md2TomlError::PathError(format!(
                    "Failed to get relative path: {:?} from {:?}",
                    src_path, src_dir
                ))
            })?;
            let mut dest_path = dest_dir.join(relative_path);

            match ext {
                "md" => {
                    dest_path.set_extension("toml");
                    match self.convert_single_file(src_path.clone(), Some(dest_path.clone())) {
                        Ok(_) => stats.add_success(),
                        Err(e) => stats.add_failure(src_path, &e.to_string()),
                    }
                }
                "txt" => {
                    dest_path.set_extension("txt");
                    // Ensure parent directory exists
                    if let Some(parent) = dest_path.parent() {
                        self.fs.create_dir_all(parent)?;
                    }
                    match self.fs.copy(&src_path, &dest_path) {
                        Ok(_) => stats.add_success(),
                        Err(e) => stats.add_failure(src_path, &e.to_string()),
                    }
                }
                _ => {}
            }
        }

        Ok(stats)
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

/// Factory for creating service instances with Arc-based dependency injection
pub struct Md2TomlServiceBuilder {
    fs: Option<Arc<Box<dyn FileSystem>>>,
    logger: Option<Arc<dyn Logger>>,
    config: Md2TomlConfig,
}

impl Md2TomlServiceBuilder {
    pub fn new() -> Self {
        Self {
            fs: None,
            logger: None,
            config: Md2TomlConfig::default(),
        }
    }

    pub fn with_filesystem(mut self, fs: Arc<Box<dyn FileSystem>>) -> Self {
        self.fs = Some(fs);
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

    pub fn build(self) -> Md2TomlService {
        let fs = self.fs.expect("FileSystem must be provided");
        let logger = self.logger.expect("Logger must be provided");

        Md2TomlService::new(fs, logger, self.config)
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
    use crate::domain::ports::{FileSystem, LogLevel, Logger};

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
        fn is_file(&self, path: &Path) -> bool {
            path.extension().is_some()
        }
        fn is_directory(&self, _path: &Path) -> bool {
            false
        }
        fn remove_file(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn remove_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn metadata(&self, _path: &Path) -> Result<crate::domain::ports::FileMetadata> {
            let fs_metadata = std::fs::metadata(".").unwrap();
            Ok(crate::domain::ports::FileMetadata::from_fs_metadata(
                &fs_metadata,
            ))
        }
        fn list_files_with_extension(&self, _dir: &Path, _extension: &str) -> Result<Vec<PathBuf>> {
            Ok(Vec::new())
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
            .with_filesystem(Arc::new(Box::new(MockFileSystem)))
            .with_logger(Arc::new(MockLogger))
            .with_config(Md2TomlConfig::default());

        let service = builder.build();
        // Test that Arc-based service works correctly
        assert!(service.fs.read_to_string(Path::new("test")).is_ok());
    }
}
