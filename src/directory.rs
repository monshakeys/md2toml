use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use crate::error::{Result, Md2TomlError};

/// Processing strategy for different file types
#[derive(Debug, Clone, Copy)]
enum ProcessingStrategy {
    /// Convert markdown to TOML (md -> toml)
    Convert,
    /// Copy file as-is (txt -> txt)
    Copy,
}

/// Configuration for processing a specific file type
struct FileTypeConfig {
    /// Source file extension (e.g., "md", "txt")
    source_extension: &'static str,
    /// Output file extension (e.g., "toml", "txt")
    output_extension: &'static str,
    /// Processing strategy to apply
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

/// Represents the type of input path detected
#[derive(Debug, PartialEq)]
pub enum InputType {
    File(PathBuf),
    Directory(PathBuf),
    Invalid(String),
}

/// Detects whether the input path is a file, directory, or invalid
pub fn detect_input_type(path: &Path) -> InputType {
    if !path.exists() {
        return InputType::Invalid(format!("Path does not exist: {:?}", path));
    }

    if path.is_file() {
        if path.extension() == Some("md".as_ref()) {
            return InputType::File(path.to_path_buf());
        } else if path.extension() == Some("txt".as_ref()) {
            return InputType::File(path.to_path_buf());
        } else {
            return InputType::Invalid(
                format!("Input file must be a .md or .txt file, got: {:?}", path.extension())
            );
        }
    }

    if path.is_dir() {
        return InputType::Directory(path.to_path_buf());
    }

    InputType::Invalid(format!("Invalid input type: {:?}", path))
}

/// Checks if a path should be ignored based on prefix rules
///
/// Files or directories are ignored if they start with:
/// - `~` (tilde): temporary files, drafts, backup directories
/// - `.` (dot): hidden files and directories (e.g., `.DS_Store`, `.git`)
///
/// # Examples
/// ```
/// # use std::path::Path;
/// # use md2toml::should_ignore_path;
/// assert!(should_ignore_path(Path::new("~temp.md")));
/// assert!(should_ignore_path(Path::new(".hidden.md")));
/// assert!(should_ignore_path(Path::new(".git")));
/// assert!(!should_ignore_path(Path::new("normal.md")));
/// ```
pub fn should_ignore_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('~') || name.starts_with('.'))
        .unwrap_or(false)
}

/// Recursively collects all files with a specific extension in a directory
///
/// Ignores files and directories starting with `~` or `.` using `filter_entry`
/// for performance (avoids traversing ignored directories).
pub fn collect_files_by_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let root_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());

    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Don't filter the root directory itself
            let entry_path = e.path().canonicalize().unwrap_or_else(|_| e.path().to_path_buf());
            if entry_path == root_dir {
                return true;
            }
            !should_ignore_path(e.path())
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && path.extension() == Some(extension.as_ref()) {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

/// Maps a source path to its corresponding destination base path (without changing extension)
///
/// Preserves the directory structure.
///
/// # Example
/// ```text
/// src: /project/docs/api/intro.md
/// src_root: /project/docs
/// dest_root: /output
/// => /output/api/intro.md  (note: extension is kept, not changed to .toml)
/// ```
pub fn map_output_base_path(src: &Path, src_root: &Path, dest_root: &Path) -> Result<PathBuf> {
    let relative = src.strip_prefix(src_root)
        .map_err(|_| Md2TomlError::InvalidFrontmatter(
            format!("Failed to get relative path: {:?} from {:?}", src, src_root)
        ))?;

    let dest = dest_root.join(relative);

    Ok(dest)
}

/// Tracks conversion statistics for batch operations
#[derive(Debug, Default)]
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

    pub fn add_failure(&mut self, path: PathBuf, error: String) {
        self.failed.push((path, error));
    }

    pub fn add_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }
}

/// Prompts user to confirm overwriting an existing directory
pub fn confirm_overwrite(path: &Path) -> bool {
    use std::io::{self, Write};

    print!("Directory {:?} already exists. Overwrite? [y/N]: ", path);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    input == "y" || input == "yes"
}

/// Ensures the parent directory of a path exists
pub fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Md2TomlError::IoError(e))?;
    }
    Ok(())
}

/// Converts a single markdown file to TOML format
fn convert_single_file(src: &Path, dest: &Path) -> Result<()> {
    let input_content = fs::read_to_string(src)?;
    let metadata = crate::parse_frontmatter(&input_content)?;
    crate::validate_required_fields(&metadata)?;
    let content = crate::extract_content(&input_content)?;
    let processed_content = crate::apply_all_replacements(&content);
    let toml_output = crate::generate_toml(metadata, processed_content)?;

    ensure_parent_dir(dest)?;
    fs::write(dest, toml_output)?;

    Ok(())
}

/// Copies a single file from source to destination
fn copy_single_file(src: &Path, dest: &Path) -> Result<()> {
    ensure_parent_dir(dest)?;
    fs::copy(src, dest)?;
    Ok(())
}

/// Process a single file based on the strategy
fn process_file(src: &Path, dest: &Path, strategy: ProcessingStrategy) -> Result<()> {
    match strategy {
        ProcessingStrategy::Convert => convert_single_file(src, dest),
        ProcessingStrategy::Copy => copy_single_file(src, dest),
    }
}

/// Converts all markdown files in a directory to TOML format
///
/// Returns statistics about the conversion process. Failed conversions
/// are recorded but don't stop the entire batch operation.
pub fn convert_directory(src_dir: &Path, dest_dir: &Path, verbose: bool) -> Result<ConversionStats> {
    let mut stats = ConversionStats::new();
    
    // ANSI color codes
    const CYAN: &str = "\x1b[36m";
    const GREEN: &str = "\x1b[32m";
    const RED: &str = "\x1b[31m";
    const YELLOW: &str = "\x1b[33m";
    const DIM: &str = "\x1b[2m";
    const BOLD: &str = "\x1b[1m";
    const RESET: &str = "\x1b[0m";
    
    // Collect all files to process based on SUPPORTED_FILE_TYPES
    let mut all_files: Vec<(PathBuf, &FileTypeConfig)> = Vec::new();
    
    for config in SUPPORTED_FILE_TYPES {
        let files = collect_files_by_extension(src_dir, config.source_extension)?;
        for file in files {
            all_files.push((file, config));
        }
    }
    
    if all_files.is_empty() {
        let extensions: Vec<String> = SUPPORTED_FILE_TYPES
            .iter()
            .map(|c| format!(".{}", c.source_extension))
            .collect();
        return Err(Md2TomlError::InvalidFrontmatter(
            format!("No {} files found in directory: {:?}",
                    extensions.join(" or "), src_dir)
        ));
    }
    
    stats.total = all_files.len();
    
    if verbose {
        println!("\n{}{}Found {} files to process{}", 
            BOLD, CYAN, stats.total, RESET);
        println!("{}{}{}", DIM, "─".repeat(50), RESET);
    }
    
    // Process all files using their respective strategies
    for (i, (src_path, config)) in all_files.iter().enumerate() {
        let mut dest_path = map_output_base_path(&src_path, src_dir, dest_dir)?;
        dest_path.set_extension(config.output_extension);
        
        if verbose {
            let action = match config.strategy {
                ProcessingStrategy::Convert => "Converting",
                ProcessingStrategy::Copy => "Copying   ",
            };
            let file_name = src_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            print!("\n{}[{}/{}]{} {}{}{} {}{}{}",
                DIM, i + 1, stats.total, RESET,
                YELLOW, action, RESET,
                CYAN, file_name, RESET);
        }
        
        match process_file(&src_path, &dest_path, config.strategy) {
            Ok(_) => {
                stats.add_success();
                if verbose {
                    println!(" {}✓{}", GREEN, RESET);
                }
            }
            Err(e) => {
                stats.add_failure(src_path.clone(), e.to_string());
                if verbose {
                    println!(" {}✗{}", RED, RESET);
                    eprintln!("  {}└─ Error: {}{}", DIM, e, RESET);
                }
            }
        }
    }
    
    if verbose {
        println!("{}{}{}", DIM, "─".repeat(50), RESET);
    }
    
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_input_type_file() {
        let path = PathBuf::from("fixtures/simple.md");
        let result = detect_input_type(&path);

        assert_eq!(result, InputType::File(path.clone()));
    }

    #[test]
    fn test_detect_input_type_directory() {
        let path = PathBuf::from("fixtures");
        let result = detect_input_type(&path);

        assert_eq!(result, InputType::Directory(path.clone()));
    }

    #[test]
    fn test_detect_input_type_nonexistent() {
        let path = PathBuf::from("nonexistent.md");
        let result = detect_input_type(&path);

        match result {
            InputType::Invalid(msg) => assert!(msg.contains("does not exist")),
            _ => panic!("Expected Invalid"),
        }
    }

    #[test]
    fn test_detect_input_type_wrong_extension() {
        let path = PathBuf::from("Cargo.toml");
        let result = detect_input_type(&path);

        match result {
            InputType::Invalid(msg) => assert!(msg.contains("must be a .md or .txt file")),
            _ => panic!("Expected Invalid"),
        }
    }

    #[test]
    fn test_detect_input_type_txt_file() {
        let path = PathBuf::from("fixtures/simple.txt"); // Assuming simple.txt exists for test
        let result = detect_input_type(&path);

        assert_eq!(result, InputType::File(path.clone()));
    }

    #[test]
    fn test_collect_files_by_extension_md() {
        let result = collect_files_by_extension(Path::new("fixtures"), "md").unwrap();

        assert_eq!(result.len(), 6);

        for path in result {
            assert_eq!(path.extension(), Some("md".as_ref()));
        }
    }

    #[test]
    fn test_collect_files_by_extension_txt() {
        // Need to create a temporary .txt file in fixtures for this test
        let test_dir = Path::new("fixtures");
        let txt_file_path = test_dir.join("temp_test.txt");
        std::fs::write(&txt_file_path, "test content").unwrap();

        let result = collect_files_by_extension(test_dir, "txt").unwrap();

        // Check if simple.txt and temp_test.txt are found
        assert!(result.iter().any(|p| p.file_name().unwrap() == "simple.txt"));
        assert!(result.iter().any(|p| p.file_name().unwrap() == "temp_test.txt"));
        assert_eq!(result.len(), 2); // simple.txt and temp_test.txt

        for path in result {
            assert_eq!(path.extension(), Some("txt".as_ref()));
        }

        // Clean up
        std::fs::remove_file(&txt_file_path).unwrap();
    }

    #[test]
    fn test_map_output_base_path() {
        let src = PathBuf::from("/project/docs/api/intro.md");
        let src_root = PathBuf::from("/project/docs");
        let dest_root = PathBuf::from("/output");

        let result = map_output_base_path(&src, &src_root, &dest_root).unwrap();

        assert_eq!(result, PathBuf::from("/output/api/intro.md"));
    }

    #[test]
    fn test_map_output_base_path_same_level() {
        let src = PathBuf::from("/project/docs/intro.md");
        let src_root = PathBuf::from("/project/docs");
        let dest_root = PathBuf::from("/output");

        let result = map_output_base_path(&src, &src_root, &dest_root).unwrap();

        assert_eq!(result, PathBuf::from("/output/intro.md"));
    }

    #[test]
    fn test_conversion_stats() {
        let mut stats = ConversionStats::new();

        assert_eq!(stats.total, 0);
        assert_eq!(stats.success, 0);
        assert!(!stats.has_failures());

        stats.add_success();
        stats.add_success();
        assert_eq!(stats.success, 2);

        stats.add_failure(PathBuf::from("test.md"), "error".to_string());
        assert!(stats.has_failures());
        assert_eq!(stats.failed.len(), 1);
    }

    #[test]
    fn test_should_ignore_tilde_prefix_file() {
        assert!(should_ignore_path(Path::new("~temp.md")));
        assert!(should_ignore_path(Path::new("~backup.md")));
        assert!(should_ignore_path(Path::new("path/to/~hidden.md")));
    }

    #[test]
    fn test_should_ignore_dot_prefix_file() {
        assert!(should_ignore_path(Path::new(".hidden.md")));
        assert!(should_ignore_path(Path::new(".DS_Store")));
        assert!(should_ignore_path(Path::new("path/to/.gitignore")));
    }

    #[test]
    fn test_should_not_ignore_normal_file() {
        assert!(!should_ignore_path(Path::new("normal.md")));
        assert!(!should_ignore_path(Path::new("file~with~tilde.md")));
        assert!(!should_ignore_path(Path::new("file.with.dots.md")));
        assert!(!should_ignore_path(Path::new("path/to/normal.md")));
    }

    #[test]
    fn test_should_ignore_tilde_prefix_directory() {
        assert!(should_ignore_path(Path::new("~temp")));
        assert!(should_ignore_path(Path::new("~backup")));
        assert!(should_ignore_path(Path::new("path/to/~hidden")));
    }

    #[test]
    fn test_should_ignore_dot_prefix_directory() {
        assert!(should_ignore_path(Path::new(".git")));
        assert!(should_ignore_path(Path::new(".claude")));
        assert!(should_ignore_path(Path::new("path/to/.hidden")));
    }

    #[test]
    fn test_should_not_ignore_normal_directory() {
        assert!(!should_ignore_path(Path::new("normal")));
        assert!(!should_ignore_path(Path::new("folder~with~tilde")));
        assert!(!should_ignore_path(Path::new("path/to/normal")));
    }

    #[test]
    fn test_collect_files_by_extension_ignores_tilde_files() {
        let test_dir = Path::new("tests/fixtures/test_tilde_ignore");
        if test_dir.exists() {
            let result = collect_files_by_extension(test_dir, "md").unwrap();

            for path in &result {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                assert!(!file_name.starts_with('~'),
                    "File starting with ~ should be ignored: {:?}", path);
                assert!(!file_name.starts_with('.'),
                    "File starting with . should be ignored: {:?}", path);
            }
        }
    }

    #[test]
    fn test_collect_files_by_extension_ignores_tilde_directories() {
        let test_dir = Path::new("tests/fixtures/test_tilde_ignore");
        if test_dir.exists() {
            let result = collect_files_by_extension(test_dir, "md").unwrap();

            for path in &result {
                let path_str = path.to_str().unwrap();
                assert!(!path_str.contains("/~"),
                    "Files from directories starting with ~ should be ignored: {:?}", path);
                assert!(!path_str.contains("/."),
                    "Files from directories starting with . should be ignored: {:?}", path);
            }
        }
    }

    #[test]
    fn test_convert_directory_with_mixed_files() {
        // Use the pre-created test fixtures in tests/fixtures/mixed_files
        let input_dir = Path::new("tests/fixtures/mixed_files");
        let output_dir = Path::new("test_output/mixed_files_output");
        
        // Clean up output directory if it exists from previous test runs
        if output_dir.exists() {
            fs::remove_dir_all(output_dir).unwrap();
        }
        
        // Perform conversion
        let stats = convert_directory(
            input_dir,
            output_dir,
            false,
        ).unwrap();

        // Assert stats
        assert_eq!(stats.total, 4);
        assert_eq!(stats.success, 4);
        assert!(!stats.has_failures());

        // Assert output files
        let expected_toml_path = output_dir.join("test.toml");
        let expected_txt_path = output_dir.join("note.txt");
        let expected_sub_toml_path = output_dir.join("sub").join("sub_test.toml");
        let expected_sub_txt_path = output_dir.join("sub").join("sub_note.txt");

        assert!(expected_toml_path.exists());
        assert!(expected_txt_path.exists());
        assert!(expected_sub_toml_path.exists());
        assert!(expected_sub_txt_path.exists());

        // Verify content for .toml files
        let toml_output = fs::read_to_string(&expected_toml_path).unwrap();
        assert!(toml_output.contains("title = \"Test Title\""));
        assert!(toml_output.contains("description = \"Test Description\""));
        assert!(toml_output.contains("prompt = \"\"\"\n# Hello Markdown\n\"\"\""));

        let sub_toml_output = fs::read_to_string(&expected_sub_toml_path).unwrap();
        assert!(sub_toml_output.contains("title = \"Sub Title\""));
        assert!(sub_toml_output.contains("description = \"Sub Description\""));
        assert!(sub_toml_output.contains("prompt = \"\"\"\nSub Markdown\n\"\"\""));

        // Verify content for .txt files
        let txt_output = fs::read_to_string(&expected_txt_path).unwrap();
        assert_eq!(txt_output, "This is a plain text file.");

        let sub_txt_output = fs::read_to_string(&expected_sub_txt_path).unwrap();
        assert_eq!(sub_txt_output, "Sub plain text file.");

        // Cleanup: Remove the test output directory
        fs::remove_dir_all(output_dir).unwrap();
    }
}
