// Infrastructure Layer - Directory I/O Operations Helper Functions
//
// This module provides helper functions for directory operations.
// The main directory conversion logic is implemented in directory_conversion_use_case.rs

use crate::domain::errors::Result;
use crate::domain::ports::FileSystem;
use std::path::{Path, PathBuf};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::FileSystem;
    use std::path::{Path, PathBuf};

    struct MockFileSystem;
    impl FileSystem for MockFileSystem {
        fn read_to_string(&self, _path: &Path) -> Result<String> {
            Ok(String::new())
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

    #[test]
    fn test_ensure_parent_dir() {
        let fs = MockFileSystem;
        let path = Path::new("some/nested/path/file.txt");
        assert!(ensure_parent_dir(&fs, path).is_ok());
    }
}
