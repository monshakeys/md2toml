use std::path::{Path, PathBuf};

use crate::domain::models::InputType;

/// Checks if a path should be ignored based on prefix rules
///
/// Files or directories are ignored if they start with:
/// - `~` (tilde): temporary files, drafts, backup directories
/// - `.` (dot): hidden files and directories (e.g., `.DS_Store`, `.git`)
///
/// # Examples
/// ```
/// # use std::path::Path;
/// # use md2toml::domain::directory::should_ignore_path;
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

/// Detects whether the input path is a file, directory, or invalid
///
/// This is a pure function that doesn't perform I/O checks.
/// Assumes path exists and performs validation based on extension only.
pub fn detect_input_type(path: &Path) -> InputType {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext == "md" || ext == "txt" {
            return InputType::File(path.to_path_buf());
        } else {
            return InputType::Invalid(format!(
                "Input file must be a .md or .txt file, got: .{}",
                ext
            ));
        }
    }

    InputType::Directory(path.to_path_buf())
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
pub fn map_output_base_path(
    src: &Path,
    src_root: &Path,
    dest_root: &Path,
) -> Result<PathBuf, crate::domain::errors::Md2TomlError> {
    let relative = src.strip_prefix(src_root).map_err(|_| {
        crate::domain::errors::Md2TomlError::PathError(format!(
            "Failed to get relative path: {:?} from {:?}",
            src, src_root
        ))
    })?;

    let dest = dest_root.join(relative);

    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
}
