use crate::domain::errors::{Md2TomlError, Result};
use crate::domain::ports::{FileMetadata, FileSystem};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Standard file system adapter
///
/// Provides concrete implementation of file system operations
/// by delegating to std::fs and std::path.
pub struct FsAdapter;

impl FsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem for FsAdapter {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(|_| Md2TomlError::IoError)
    }

    fn write(&self, path: &Path, content: &str) -> Result<usize> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|_| Md2TomlError::IoError)?;
        }

        fs::write(path, content).map_err(|_| Md2TomlError::IoError)?;

        Ok(content.len())
    }

    fn copy(&self, from: &Path, to: &Path) -> Result<u64> {
        // Ensure parent directory exists
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(|_| Md2TomlError::IoError)?;
        }

        fs::copy(from, to).map_err(|_| Md2TomlError::IoError)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(|_| Md2TomlError::IoError)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn is_directory(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(|_| Md2TomlError::IoError)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path).map_err(|_| Md2TomlError::IoError)
    }

    fn metadata(&self, path: &Path) -> Result<FileMetadata> {
        let fs_metadata = fs::metadata(path).map_err(|e| {
            Md2TomlError::FileSystem(format!("Failed to read metadata for {:?}: {}", path, e))
        })?;

        Ok(FileMetadata::new(
            fs_metadata.len(),
            fs_metadata.is_dir(),
            fs_metadata.modified().ok(),
        ))
    }

    fn list_files_with_extension(&self, dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let root_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());

        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                let entry_path = e
                    .path()
                    .canonicalize()
                    .unwrap_or_else(|_| e.path().to_path_buf());
                if entry_path == root_dir {
                    return true;
                }
                let file_name = e.file_name();
                !file_name
                    .to_str()
                    .is_some_and(|s| s.starts_with('~') || s.starts_with('.'))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::{
        fixture::{FileWriteStr, PathChild},
        TempDir,
    };

    #[test]
    fn test_write_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir").join("file.txt");

        let adapter = FsAdapter::new();
        let result = adapter.write(&nested_path, "content");

        assert!(result.is_ok());
        assert!(nested_path.exists());
    }

    #[test]
    fn test_copy_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.child("source.txt");
        source_file.write_str("content").unwrap();

        let dest_path = temp_dir.path().join("nested").join("dest.txt");

        let adapter = FsAdapter::new();
        let result = adapter.copy(source_file.path(), &dest_path);

        assert!(result.is_ok());
        assert!(dest_path.exists());
        assert_eq!(std::fs::read_to_string(&dest_path).unwrap(), "content");
    }
}
