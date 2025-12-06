//! Directory Conversion Use Case Implementation
//!
//! Orchestrates the directory conversion business workflow.

use crate::domain::errors::Result;
use crate::domain::models::ConversionStats;
use crate::domain::ports::DirectoryConversionUseCase;
use std::path::{Path, PathBuf};

/// Standard directory conversion use case implementation
///
/// Orchestrates the business workflow:
/// - Scans directory for files
/// - Determines processing strategy for each file
/// - Coordinates file processing
/// - Tracks conversion statistics
///
/// Delegates I/O operations to the FileSystem port.
pub struct StandardDirectoryConversionUseCase {
    fs: std::sync::Arc<dyn crate::domain::ports::FileSystem>,
}

impl StandardDirectoryConversionUseCase {
    pub fn new(fs: std::sync::Arc<dyn crate::domain::ports::FileSystem>) -> Self {
        Self { fs }
    }
}

impl DirectoryConversionUseCase for StandardDirectoryConversionUseCase {
    fn convert_directory(
        &self,
        src_dir: &Path,
        dest_dir: &Path,
        _verbose: bool,
    ) -> Result<ConversionStats> {
        let mut stats = ConversionStats::new();

        // Collect markdown and text files via FileSystem port
        let md_files = self.fs.list_files_with_extension(src_dir, "md")?;
        let txt_files = self.fs.list_files_with_extension(src_dir, "txt")?;

        let all_files: Vec<(PathBuf, &str)> = md_files
            .iter()
            .map(|p| (p.clone(), "md"))
            .chain(txt_files.iter().map(|p| (p.clone(), "txt")))
            .collect();

        stats.total = all_files.len();

        if all_files.is_empty() {
            return Err(crate::domain::errors::Md2TomlError::InvalidFrontmatter(
                format!("No supported files found in directory: {:?}", src_dir),
            ));
        }

        for (src_path, ext) in all_files {
            let relative_path = src_path.strip_prefix(src_dir).map_err(|_| {
                crate::domain::errors::Md2TomlError::PathError(format!(
                    "Failed to get relative path: {:?} from {:?}",
                    src_path, src_dir
                ))
            })?;
            let mut dest_path = dest_dir.join(relative_path);

            match ext {
                "md" => {
                    dest_path.set_extension("toml");
                    match self.process_markdown_file(&src_path, &dest_path) {
                        Ok(_) => stats.add_success(),
                        Err(e) => stats.add_failure(src_path, &e.to_string()),
                    }
                }
                "txt" => {
                    dest_path.set_extension("txt");
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
}

impl StandardDirectoryConversionUseCase {
    /// Process a single markdown file
    fn process_markdown_file(&self, src: &Path, dest: &Path) -> Result<()> {
        use crate::domain::convert_markdown_to_toml;

        // I/O: Read input file via FileSystem port
        let input_content = self.fs.read_to_string(src)?;

        // Business logic: Use pure domain function
        let toml_output = convert_markdown_to_toml(&input_content)?;

        // I/O: Ensure parent directory and write output via FileSystem port
        if let Some(parent) = dest.parent() {
            self.fs.create_dir_all(parent)?;
        }
        self.fs.write(dest, &toml_output)?;

        Ok(())
    }
}
