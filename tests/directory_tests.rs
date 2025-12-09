use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use md2toml::application::StandardDirectoryConversionUseCase;
use md2toml::domain::ports::DirectoryConversionUseCase;
use md2toml::infra::FsAdapter;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Helper to create a directory converter instance
fn create_directory_converter() -> Arc<dyn DirectoryConversionUseCase> {
    let fs = Arc::new(FsAdapter::new());
    Arc::new(StandardDirectoryConversionUseCase::new(fs))
}

/// External I/O Tests (File System Operations)
/// According to Rust Guidelines 4-3: Requires 5 second timeout
/// for file operations to prevent deadlocks and handle OS scheduling delays.

#[tokio::test]
async fn test_convert_directory_with_nested_structure() {
    // External I/O Test - 5 second timeout (File system operations)
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = fixture_path("test_project");
        let output_dir = assert_fs::TempDir::new().unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(&input_dir, output_dir.path(), false)
            .unwrap();

        assert_eq!(stats.total, 4, "Should find 4 markdown files");
        assert_eq!(stats.success, 4, "All 4 files should convert successfully");
        assert_eq!(stats.failed.len(), 0, "No files should fail");
        assert!(!stats.has_failures(), "Should have no failures");

        output_dir
            .child("intro.toml")
            .assert(predicate::path::exists());

        output_dir
            .child("api")
            .child("auth.toml")
            .assert(predicate::path::exists());

        output_dir
            .child("api")
            .child("users.toml")
            .assert(predicate::path::exists());

        output_dir
            .child("guides")
            .child("setup.toml")
            .assert(predicate::path::exists());

        let intro_content = fs::read_to_string(output_dir.child("intro.toml").path()).unwrap();

        assert!(
            intro_content.contains("title = \"Introduction\""),
            "Should have title"
        );
        assert!(
            intro_content.contains("description = \"Getting started guide\""),
            "Should have description"
        );
        assert!(
            intro_content.contains("version = \"1.0\""),
            "Should preserve version"
        );

        assert!(
            intro_content.contains("prompt = \"\"\""),
            "Should have prompt block"
        );

        assert!(
            intro_content.contains("{{args}}"),
            "$ARGUMENTS should be replaced with {{{{args}}}}"
        );
        assert!(
            intro_content.contains("!{npm install}"),
            "!`npm install` should be replaced with !{{npm install}}"
        );

        let auth_content =
            fs::read_to_string(output_dir.child("api").child("auth.toml").path()).unwrap();

        assert!(auth_content.contains("title = \"Authentication API\""));
        assert!(auth_content.contains("description = \"How to authenticate with the API\""));
        assert!(auth_content.contains("status = \"stable\""));
        assert!(auth_content.contains("author = \"API Team\""));
        assert!(
            auth_content.contains("!{login}"),
            "Should contain replaced command"
        );
        assert!(
            auth_content.contains("{{args}}"),
            "Should contain replaced arguments"
        );

        let lines: Vec<&str> = auth_content.lines().collect();
        let metadata_lines: Vec<&str> = lines
            .iter()
            .take_while(|line| !line.is_empty())
            .copied()
            .collect();

        assert!(metadata_lines[0].starts_with("author"));
        assert!(metadata_lines[1].starts_with("description"));
        assert!(metadata_lines[2].starts_with("status"));
        assert!(metadata_lines[3].starts_with("title"));
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_convert_directory_with_validation_failures() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = assert_fs::TempDir::new().unwrap();
        let output_dir = assert_fs::TempDir::new().unwrap();

        input_dir
            .child("simple.md")
            .write_str(
                r#"---
title: Simple
---
Content without description
"#,
            )
            .unwrap();

        input_dir
            .child("complex.md")
            .write_str(
                r#"---
title: Complex
description: This has both required fields
---
Content here
"#,
            )
            .unwrap();

        input_dir
            .child("with_arguments.md")
            .write_str(
                r#"---
title: With Arguments
---
Missing description
"#,
            )
            .unwrap();

        input_dir
            .child("with_backticks.md")
            .write_str(
                r#"---
title: With Backticks
---
No description field
"#,
            )
            .unwrap();

        input_dir
            .child("empty_content.md")
            .write_str(
                r#"---
title: Empty
---
"#,
            )
            .unwrap();

        input_dir
            .child("no_metadata.md")
            .write_str(
                "Just content without frontmatter
",
            )
            .unwrap();

        input_dir
            .child("simple.txt")
            .write_str("Plain text file")
            .unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(input_dir.path(), output_dir.path(), false)
            .unwrap();

        assert_eq!(
            stats.total, 7,
            "Should find 6 markdown files and 1 txt file"
        );
        assert_eq!(
            stats.success, 2,
            "Only complex.md should succeed conversion, and simple.txt should succeed copy"
        );
        assert_eq!(stats.failed.len(), 5, "5 md files should fail validation");
        assert!(stats.has_failures(), "Should have failures");

        output_dir
            .child("complex.toml")
            .assert(predicate::path::exists());
        output_dir
            .child("simple.txt")
            .assert(predicate::path::exists());

        output_dir
            .child("simple.toml")
            .assert(predicate::path::missing());

        let has_validation_error = stats
            .failed
            .iter()
            .any(|(_, error)| error.contains("缺少必要欄位"));

        assert!(
            has_validation_error,
            "Should have validation error messages"
        );
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_convert_empty_directory() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = assert_fs::TempDir::new().unwrap();
        let output_dir = assert_fs::TempDir::new().unwrap();

        input_dir.child("empty_folder").create_dir_all().unwrap();

        let converter = create_directory_converter();
        let result = converter.convert_directory(input_dir.path(), output_dir.path(), false);

        assert!(result.is_err(), "Should fail when no supported files found");
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No supported files found"));
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_convert_directory_partial_success() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = assert_fs::TempDir::new().unwrap();
        let output_dir = assert_fs::TempDir::new().unwrap();

        input_dir
            .child("valid.md")
            .write_str(
                r#"---
title: Valid File
description: This file is valid
---
Content here with $ARGUMENTS
"#,
            )
            .unwrap();

        input_dir
            .child("invalid.md")
            .write_str(
                r#"---
title: Invalid File
---
Missing description
"#,
            )
            .unwrap();

        input_dir
            .child("subfolder")
            .child("valid2.md")
            .write_str(
                r#"---
title: Another Valid File
description: Also valid
---
Content with !`command`
"#,
            )
            .unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(input_dir.path(), output_dir.path(), false)
            .unwrap();

        assert_eq!(stats.total, 3);
        assert_eq!(stats.success, 2);
        assert_eq!(stats.failed.len(), 1);

        output_dir
            .child("valid.toml")
            .assert(predicate::path::exists());
        output_dir
            .child("subfolder")
            .child("valid2.toml")
            .assert(predicate::path::exists());

        output_dir
            .child("invalid.toml")
            .assert(predicate::path::missing());

        let valid_content = fs::read_to_string(output_dir.child("valid.toml")).unwrap();
        assert!(
            valid_content.contains("{{args}}"),
            "Should have replaced arguments"
        );

        let valid2_content =
            fs::read_to_string(output_dir.child("subfolder").child("valid2.toml")).unwrap();
        assert!(
            valid2_content.contains("!{command}"),
            "Should have replaced command"
        );
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_convert_deeply_nested_directory() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = assert_fs::TempDir::new().unwrap();
        let output_dir = assert_fs::TempDir::new().unwrap();

        input_dir
            .child("level1")
            .child("level2")
            .child("level3")
            .child("deep.md")
            .write_str(
                r#"---
title: Deep File
description: Deeply nested file
---
Deep content with $ARGUMENTS and !`deep-command`
"#,
            )
            .unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(input_dir.path(), output_dir.path(), false)
            .unwrap();

        assert_eq!(stats.success, 1);

        output_dir
            .child("level1")
            .child("level2")
            .child("level3")
            .child("deep.toml")
            .assert(predicate::path::exists());

        let deep_content = fs::read_to_string(
            output_dir
                .child("level1")
                .child("level2")
                .child("level3")
                .child("deep.toml"),
        )
        .unwrap();

        assert!(
            deep_content.contains("title = \"Deep File\""),
            "Should have title"
        );
        assert!(
            deep_content.contains("{{args}}"),
            "Should have replaced arguments"
        );
        assert!(
            deep_content.contains("!{deep-command}"),
            "Should have replaced command"
        );
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_path_mapping_correctness() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = fixture_path("test_project");
        let output_dir = assert_fs::TempDir::new().unwrap();

        let converter = create_directory_converter();
        converter
            .convert_directory(&input_dir, output_dir.path(), false)
            .unwrap();

        let expected_files = vec![
            "intro.toml",
            "api/auth.toml",
            "api/users.toml",
            "guides/setup.toml",
        ];

        for expected_file in expected_files {
            let file_path = output_dir.path().join(expected_file);
            assert!(
                file_path.exists(),
                "Expected file should exist: {:?}",
                expected_file
            );
        }
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_special_characters_in_filenames() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = assert_fs::TempDir::new().unwrap();
        let output_dir = assert_fs::TempDir::new().unwrap();

        input_dir
            .child("file-with-dashes.md")
            .write_str(
                r#"---
title: File with Dashes
description: Testing special chars
---
Content
"#,
            )
            .unwrap();

        input_dir
            .child("file_with_underscores.md")
            .write_str(
                r#"---
title: File with Underscores
description: More special chars
---
Content
"#,
            )
            .unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(input_dir.path(), output_dir.path(), false)
            .unwrap();

        assert_eq!(stats.success, 2);

        output_dir
            .child("file-with-dashes.toml")
            .assert(predicate::path::exists());
        output_dir
            .child("file_with_underscores.toml")
            .assert(predicate::path::exists());
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}

#[tokio::test]
async fn test_ignore_tilde_and_dot_prefix_files_and_directories() {
    // External I/O Test - 5 second timeout
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let input_dir = fixture_path("test_tilde_ignore");
        let output_dir = assert_fs::TempDir::new().unwrap();

        let converter = create_directory_converter();
        let stats = converter
            .convert_directory(&input_dir, output_dir.path(), false)
            .unwrap();

        assert_eq!(
            stats.total, 2,
            "Should find only 2 processable files (ignoring ~ and . prefixed)"
        );
        assert_eq!(stats.success, 2, "Both files should convert successfully");

        output_dir
            .child("normal.toml")
            .assert(predicate::path::exists());
        output_dir
            .child("normal_folder")
            .child("inside.toml")
            .assert(predicate::path::exists());

        output_dir
            .child("~ignored.toml")
            .assert(predicate::path::missing());

        output_dir
            .child("~hidden_folder")
            .child("hidden.toml")
            .assert(predicate::path::missing());

        output_dir
            .child(".hidden.toml")
            .assert(predicate::path::missing());

        output_dir
            .child(".git")
            .child("config.toml")
            .assert(predicate::path::missing());
        output_dir
            .child(".claude")
            .child("commands.toml")
            .assert(predicate::path::missing());

        let normal_content = fs::read_to_string(output_dir.child("normal.toml")).unwrap();
        assert!(normal_content.contains("title = \"Normal File\""));
        assert!(normal_content.contains("description = \"This is a normal file\""));

        let inside_content =
            fs::read_to_string(output_dir.child("normal_folder").child("inside.toml")).unwrap();
        assert!(inside_content.contains("title = \"Inside Normal Folder\""));
        assert!(inside_content.contains("description = \"This is inside a normal folder\""));
    })
    .await
    .expect("Timeout: File system operations took too long (>5s)");
}
