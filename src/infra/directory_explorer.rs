// Infrastructure Layer - Directory Explorer Implementation
//
// Concrete implementation of DirectoryExplorer port for file discovery.
// This implements the "Define First, Implement Later" principle from the guidelines.

use crate::domain::errors::Result;
use crate::domain::ports::DirectoryExplorer;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Standard directory explorer implementation
///
/// Recursively discovers markdown files in a directory.
/// Follows .gitignore-like rules by default (ignores dotfiles/dotfolders).
pub struct StandardDirectoryExplorer;

impl StandardDirectoryExplorer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StandardDirectoryExplorer {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectoryExplorer for StandardDirectoryExplorer {
    fn discover_markdown_files(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Filter out hidden files and directories (starting with . or ~)
                let entry_path = e.path();
                if entry_path == root {
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

            if path.is_file() && path.extension() == Some("md".as_ref()) {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }
}
