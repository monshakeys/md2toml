// Infrastructure Layer - Directory I/O Operations
//
// Concrete implementations of directory and file operations that involve I/O.
// These functions use the FileSystem port for proper dependency injection.

use crate::domain::errors::{Md2TomlError, Result};
use crate::domain::models::ConversionStats;
use crate::domain::ports::FileSystem;
use crate::domain::convert_markdown_to_toml;
use std::path::{Path, PathBuf};

/// Processing strategy for different file types
#[derive(Debug, Clone, Copy)]
enum ProcessingStrategy {
    Convert, // Convert markdown to TOML (md -> toml)
    Copy,    // Copy file as-is (txt -> txt)
}

/// Configuration for processing a specific file type
struct FileTypeConfig {
    source_extension: &'static str,
    output_extension: &'static str,
    strategy: ProcessingStrategy,
}

impl FileTypeConfig {
    const fn new(
        source_extension: &'static str,
        output_extension: &'static str,
        strategy: ProcessingStrategy,
    ) -> Self {
        Self {
            source_extension,
            output_extension,
            strategy,
        }
    }
}

/// All supported file type configurations
const SUPPORTED_FILE_TYPES: &[FileTypeConfig] = &[
    FileTypeConfig::new("md", "toml", ProcessingStrategy::Convert),
    FileTypeConfig::new("txt", "txt", ProcessingStrategy::Copy),
];

/// Recursively collects all files with a specific extension in a directory
pub fn collect_files_by_extension(
    fs: &dyn FileSystem,
    dir: &Path,
    extension: &str,
) -> Result<Vec<PathBuf>> {
    fs.list_files_with_extension(dir, extension)
}

/// Ensures the parent directory of a path exists
pub fn ensure_parent_dir(fs: &dyn FileSystem, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs.create_dir_all(parent)?;
    }
    Ok(())
}

/// Converts a single markdown file to TOML format
fn convert_single_file(fs: &dyn FileSystem, src: &Path, dest: &Path) -> Result<()> {
    // Read input file (I/O operation)
    let input_content = fs.read_to_string(src)?;

    // Use pure domain function for conversion (no I/O in domain logic)
    let toml_output = convert_markdown_to_toml(&input_content)?;

    // Ensure parent directory and write output (I/O operations)
    ensure_parent_dir(fs, dest)?;
    fs.write(dest, &toml_output)?;

    Ok(())
}

/// Copies a single file from source to destination
fn copy_single_file(fs: &dyn FileSystem, src: &Path, dest: &Path) -> Result<()> {
    ensure_parent_dir(fs, dest)?;
    fs.copy(src, dest)?;
    Ok(())
}

/// Process a single file based on the strategy
fn process_file(
    fs: &dyn FileSystem,
    src: &Path,
    dest: &Path,
    strategy: ProcessingStrategy,
) -> Result<()> {
    match strategy {
        ProcessingStrategy::Convert => convert_single_file(fs, src, dest),
        ProcessingStrategy::Copy => copy_single_file(fs, src, dest),
    }
}

/// Converts all markdown files in a directory to TOML format
pub fn convert_directory(
    fs: Box<dyn FileSystem>,
    src_dir: &Path,
    dest_dir: &Path,
    verbose: bool,
) -> Result<ConversionStats> {
    convert_directory_with_fs(fs, src_dir, dest_dir, verbose)
}

/// Converts all markdown files in a directory to TOML format using default FileSystem
pub fn convert_directory_with_default_fs(
    src_dir: &Path,
    dest_dir: &Path,
    verbose: bool,
) -> Result<ConversionStats> {
    let fs = Box::new(super::FsAdapter::new());
    convert_directory_with_fs(fs, src_dir, dest_dir, verbose)
}

fn convert_directory_with_fs(
    fs: Box<dyn FileSystem>,
    src_dir: &Path,
    dest_dir: &Path,
    _verbose: bool,
) -> Result<ConversionStats> {
    let mut stats = ConversionStats::new();

    // Collect all files to process based on SUPPORTED_FILE_TYPES
    let mut all_files: Vec<(PathBuf, &FileTypeConfig)> = Vec::new();

    for config in SUPPORTED_FILE_TYPES {
        let files = collect_files_by_extension(fs.as_ref(), src_dir, config.source_extension)?;
        for file in files {
            all_files.push((file, config));
        }
    }

    if all_files.is_empty() {
        let extensions: Vec<String> = SUPPORTED_FILE_TYPES
            .iter()
            .map(|c| format!(".{}", c.source_extension))
            .collect();
        return Err(Md2TomlError::InvalidFrontmatter(format!(
            "No {} files found in directory: {:?}",
            extensions.join(" or "),
            src_dir
        )));
    }

    stats.total = all_files.len();

    // Process all files using their respective strategies
    // Infrastructure layer NO OUTPUT - progress handled by Presentation layer
    for (src_path, config) in all_files.iter() {
        let relative_path = src_path.strip_prefix(src_dir).map_err(|_| {
            Md2TomlError::PathError(format!(
                "Failed to get relative path: {:?} from {:?}",
                src_path, src_dir
            ))
        })?;
        let mut dest_path = dest_dir.join(relative_path);
        dest_path.set_extension(config.output_extension);

        match process_file(fs.as_ref(), src_path, &dest_path, config.strategy) {
            Ok(_) => {
                stats.add_success();
                // NO OUTPUT: Infrastructure layer forbids println!/eprintln!
                // Progress display handled by Presentation layer
            }
            Err(e) => {
                stats.add_failure(src_path.clone(), &e.to_string());
                // NO OUTPUT: Infrastructure layer forbids println!/eprintln!
                // Error handling handled by Presentation layer
            }
        }
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_collect_files_by_extension_md() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
        // Use real FileSystem adapter for integration test
        use crate::infra::FsAdapter;
        let fs = FsAdapter::new();
        let result = collect_files_by_extension(&fs, &fixtures_dir, "md").unwrap();

        assert!(!result.is_empty());
        for path in result {
            assert_eq!(path.extension(), Some("md".as_ref()));
        }
    }
}
