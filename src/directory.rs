use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use crate::error::{Result, Md2TomlError};

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
        } else {
            return InputType::Invalid(
                format!("Input file must be a .md file, got: {:?}", path.extension())
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

/// Recursively collects all `.md` files in a directory
///
/// Ignores files and directories starting with `~` or `.` using `filter_entry`
/// for performance (avoids traversing ignored directories).
pub fn collect_md_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut md_files = Vec::new();
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

        if path.is_file() && path.extension() == Some("md".as_ref()) {
            md_files.push(path.to_path_buf());
        }
    }

    Ok(md_files)
}

/// Maps a source path to its corresponding destination path
///
/// Preserves the directory structure and changes `.md` extension to `.toml`.
///
/// # Example
/// ```text
/// src: /project/docs/api/intro.md
/// src_root: /project/docs
/// dest_root: /output
/// => /output/api/intro.toml
/// ```
pub fn map_output_path(src: &Path, src_root: &Path, dest_root: &Path) -> Result<PathBuf> {
    let relative = src.strip_prefix(src_root)
        .map_err(|_| Md2TomlError::InvalidFrontmatter(
            format!("Failed to get relative path: {:?} from {:?}", src, src_root)
        ))?;

    let mut dest = dest_root.join(relative);
    dest.set_extension("toml");

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

/// Converts all markdown files in a directory to TOML format
///
/// Returns statistics about the conversion process. Failed conversions
/// are recorded but don't stop the entire batch operation.
pub fn convert_directory(src_dir: &Path, dest_dir: &Path, verbose: bool) -> Result<ConversionStats> {
    let mut stats = ConversionStats::new();
    let md_files = collect_md_files(src_dir)?;

    if md_files.is_empty() {
        return Err(Md2TomlError::InvalidFrontmatter(
            format!("No .md files found in directory: {:?}", src_dir)
        ));
    }

    stats.total = md_files.len();

    if verbose {
        println!("Found {} markdown files to convert", stats.total);
    }

    for src_path in md_files {
        let dest_path = map_output_path(&src_path, src_dir, dest_dir)?;

        if verbose {
            println!("Converting: {:?} -> {:?}", src_path, dest_path);
        }

        match convert_single_file(&src_path, &dest_path) {
            Ok(_) => {
                stats.add_success();
                if verbose {
                    println!("  ✓ Success");
                }
            }
            Err(e) => {
                stats.add_failure(src_path.clone(), e.to_string());
                if verbose {
                    eprintln!("  ✗ Failed: {}", e);
                }
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
            InputType::Invalid(msg) => assert!(msg.contains("must be a .md file")),
            _ => panic!("Expected Invalid"),
        }
    }

    #[test]
    fn test_collect_md_files() {
        let result = collect_md_files(Path::new("fixtures")).unwrap();

        assert_eq!(result.len(), 6);

        for path in result {
            assert_eq!(path.extension(), Some("md".as_ref()));
        }
    }

    #[test]
    fn test_map_output_path() {
        let src = PathBuf::from("/project/docs/api/intro.md");
        let src_root = PathBuf::from("/project/docs");
        let dest_root = PathBuf::from("/output");

        let result = map_output_path(&src, &src_root, &dest_root).unwrap();

        assert_eq!(result, PathBuf::from("/output/api/intro.toml"));
    }

    #[test]
    fn test_map_output_path_same_level() {
        let src = PathBuf::from("/project/docs/intro.md");
        let src_root = PathBuf::from("/project/docs");
        let dest_root = PathBuf::from("/output");

        let result = map_output_path(&src, &src_root, &dest_root).unwrap();

        assert_eq!(result, PathBuf::from("/output/intro.toml"));
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
    fn test_collect_md_files_ignores_tilde_files() {
        let test_dir = Path::new("tests/fixtures/test_tilde_ignore");
        if test_dir.exists() {
            let result = collect_md_files(test_dir).unwrap();

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
    fn test_collect_md_files_ignores_tilde_directories() {
        let test_dir = Path::new("tests/fixtures/test_tilde_ignore");
        if test_dir.exists() {
            let result = collect_md_files(test_dir).unwrap();

            for path in &result {
                let path_str = path.to_str().unwrap();
                assert!(!path_str.contains("/~"),
                    "Files from directories starting with ~ should be ignored: {:?}", path);
                assert!(!path_str.contains("/."),
                    "Files from directories starting with . should be ignored: {:?}", path);
            }
        }
    }
}
