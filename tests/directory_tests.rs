use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

/// 測試完整的資料夾轉換（包含多層結構）
#[test]
fn test_convert_directory_with_nested_structure() {
    // 1. 準備輸入（使用版本控制的 fixtures）
    let input_dir = Path::new("tests/fixtures/test_project");

    // 2. 準備輸出（臨時目錄，測試後自動清理）
    let output_dir = assert_fs::TempDir::new().unwrap();

    // 3. 執行轉換
    let stats = md2toml::convert_directory(
        input_dir,
        output_dir.path(),
        false  // 不顯示 verbose
    ).unwrap();

    // 4. 驗證統計資訊
    assert_eq!(stats.total, 4, "Should find 4 markdown files");
    assert_eq!(stats.success, 4, "All 4 files should convert successfully");
    assert_eq!(stats.failed.len(), 0, "No files should fail");
    assert!(!stats.has_failures(), "Should have no failures");

    // 5. 驗證檔案結構（檢查所有檔案都被建立）
    output_dir.child("intro.toml")
        .assert(predicate::path::exists());

    output_dir.child("api").child("auth.toml")
        .assert(predicate::path::exists());

    output_dir.child("api").child("users.toml")
        .assert(predicate::path::exists());

    output_dir.child("guides").child("setup.toml")
        .assert(predicate::path::exists());

    // 6. 驗證 intro.toml 的內容關鍵字
    let intro_content = fs::read_to_string(
        output_dir.child("intro.toml").path()
    ).unwrap();

    // 檢查 metadata
    assert!(intro_content.contains("title = \"Introduction\""),
        "Should have title");
    assert!(intro_content.contains("description = \"Getting started guide\""),
        "Should have description");
    assert!(intro_content.contains("version = \"1.0\""),
        "Should preserve version");

    // 檢查 prompt 區塊
    assert!(intro_content.contains("prompt = \"\"\""),
        "Should have prompt block");

    // 檢查特殊替換有執行
    assert!(intro_content.contains("{{ args }}"),
        "$ARGUMENTS should be replaced with {{{{ args }}}}");
    assert!(intro_content.contains("!{npm install}"),
        "!`npm install` should be replaced with !{{npm install}}");

    // 7. 驗證 api/auth.toml 的內容
    let auth_content = fs::read_to_string(
        output_dir.child("api").child("auth.toml").path()
    ).unwrap();

    assert!(auth_content.contains("title = \"Authentication API\""));
    assert!(auth_content.contains("description = \"How to authenticate with the API\""));
    assert!(auth_content.contains("status = \"stable\""));
    assert!(auth_content.contains("author = \"API Team\""));
    assert!(auth_content.contains("!{login}"),
        "Should contain replaced command");
    assert!(auth_content.contains("{{ args }}"),
        "Should contain replaced arguments");

    // 8. 驗證 metadata 按字母排序
    let lines: Vec<&str> = auth_content.lines().collect();
    let metadata_lines: Vec<&str> = lines.iter()
        .take_while(|line| !line.is_empty())
        .copied()
        .collect();

    // author < description < status < title (字母順序)
    assert!(metadata_lines[0].starts_with("author"));
    assert!(metadata_lines[1].starts_with("description"));
    assert!(metadata_lines[2].starts_with("status"));
    assert!(metadata_lines[3].starts_with("title"));
}

/// 測試資料夾轉換時的驗證失敗處理
#[test]
fn test_convert_directory_with_validation_failures() {
    // 使用現有的 fixtures（大部分檔案缺少 description）
    let input_dir = Path::new("fixtures");
    let output_dir = assert_fs::TempDir::new().unwrap();

    let stats = md2toml::convert_directory(
        input_dir,
        output_dir.path(),
        false
    ).unwrap();

    // 只有 complex.md 有 title + description
    assert_eq!(stats.total, 6, "Should find 6 markdown files");
    assert_eq!(stats.success, 1, "Only 1 file should succeed");
    assert_eq!(stats.failed.len(), 5, "5 files should fail validation");
    assert!(stats.has_failures(), "Should have failures");

    // 驗證成功的檔案存在
    output_dir.child("complex.toml")
        .assert(predicate::path::exists());

    // 驗證失敗的檔案不存在
    output_dir.child("simple.toml")
        .assert(predicate::path::missing());

    // 驗證錯誤訊息包含驗證失敗的原因
    let has_validation_error = stats.failed.iter()
        .any(|(_, error)| error.contains("This is not a markdown file for cc slash command"));

    assert!(has_validation_error, "Should have validation error messages");
}

/// 測試空資料夾（沒有 .md 檔案）
#[test]
fn test_convert_empty_directory() {
    let input_dir = assert_fs::TempDir::new().unwrap();
    let output_dir = assert_fs::TempDir::new().unwrap();

    // 建立空資料夾結構
    input_dir.child("empty_folder").create_dir_all().unwrap();

    // 應該返回錯誤（沒有找到 .md 檔案）
    let result = md2toml::convert_directory(
        input_dir.path(),
        output_dir.path(),
        false
    );

    assert!(result.is_err(), "Should fail when no .md files found");
    assert!(result.unwrap_err().to_string().contains("No .md files found"));
}

/// 測試只有部分檔案成功的情況
#[test]
fn test_convert_directory_partial_success() {
    let input_dir = assert_fs::TempDir::new().unwrap();
    let output_dir = assert_fs::TempDir::new().unwrap();

    // 建立測試檔案
    // 有效的檔案
    input_dir.child("valid.md").write_str(r#"---
title: Valid File
description: This file is valid
---
Content here with $ARGUMENTS
"#).unwrap();

    // 無效的檔案（缺少 description）
    input_dir.child("invalid.md").write_str(r#"---
title: Invalid File
---
Missing description
"#).unwrap();

    // 另一個有效的檔案
    input_dir.child("subfolder").child("valid2.md").write_str(r#"---
title: Another Valid File
description: Also valid
---
Content with !`command`
"#).unwrap();

    let stats = md2toml::convert_directory(
        input_dir.path(),
        output_dir.path(),
        false
    ).unwrap();

    // 驗證統計
    assert_eq!(stats.total, 3);
    assert_eq!(stats.success, 2);
    assert_eq!(stats.failed.len(), 1);

    // 驗證成功的檔案存在
    output_dir.child("valid.toml")
        .assert(predicate::path::exists());
    output_dir.child("subfolder").child("valid2.toml")
        .assert(predicate::path::exists());

    // 驗證失敗的檔案不存在
    output_dir.child("invalid.toml")
        .assert(predicate::path::missing());

    // 驗證內容
    let valid_content = fs::read_to_string(output_dir.child("valid.toml")).unwrap();
    assert!(valid_content.contains("{{ args }}"),
        "Should have replaced arguments");

    let valid2_content = fs::read_to_string(
        output_dir.child("subfolder").child("valid2.toml")
    ).unwrap();
    assert!(valid2_content.contains("!{command}"),
        "Should have replaced command");
}

/// 測試深層嵌套結構
#[test]
fn test_convert_deeply_nested_directory() {
    let input_dir = assert_fs::TempDir::new().unwrap();
    let output_dir = assert_fs::TempDir::new().unwrap();

    // 建立深層結構
    input_dir.child("level1")
        .child("level2")
        .child("level3")
        .child("deep.md")
        .write_str(r#"---
title: Deep File
description: Deeply nested file
---
Deep content with $ARGUMENTS and !`deep-command`
"#).unwrap();

    let stats = md2toml::convert_directory(
        input_dir.path(),
        output_dir.path(),
        false
    ).unwrap();

    assert_eq!(stats.success, 1);

    // 驗證深層結構被保留
    output_dir.child("level1")
        .child("level2")
        .child("level3")
        .child("deep.toml")
        .assert(predicate::path::exists());

    // 驗證內容
    let deep_content = fs::read_to_string(
        output_dir.child("level1")
            .child("level2")
            .child("level3")
            .child("deep.toml")
    ).unwrap();

    assert!(deep_content.contains("title = \"Deep File\""),
        "Should have title");
    assert!(deep_content.contains("{{ args }}"),
        "Should have replaced arguments");
    assert!(deep_content.contains("!{deep-command}"),
        "Should have replaced command");
}

/// 測試路徑映射正確性
#[test]
fn test_path_mapping_correctness() {
    let input_dir = Path::new("tests/fixtures/test_project");
    let output_dir = assert_fs::TempDir::new().unwrap();

    md2toml::convert_directory(input_dir, output_dir.path(), false).unwrap();

    // 驗證所有路徑都正確映射
    let expected_files = vec![
        "intro.toml",
        "api/auth.toml",
        "api/users.toml",
        "guides/setup.toml",
    ];

    for expected_file in expected_files {
        let file_path = output_dir.path().join(expected_file);
        assert!(file_path.exists(),
            "Expected file should exist: {:?}", expected_file);
    }
}

/// 測試特殊字元在檔案名稱中
#[test]
fn test_special_characters_in_filenames() {
    let input_dir = assert_fs::TempDir::new().unwrap();
    let output_dir = assert_fs::TempDir::new().unwrap();

    // 建立有特殊字元的檔案名稱
    input_dir.child("file-with-dashes.md").write_str(r#"---
title: File with Dashes
description: Testing special chars
---
Content
"#).unwrap();

    input_dir.child("file_with_underscores.md").write_str(r#"---
title: File with Underscores
description: More special chars
---
Content
"#).unwrap();

    let stats = md2toml::convert_directory(
        input_dir.path(),
        output_dir.path(),
        false
    ).unwrap();

    assert_eq!(stats.success, 2);

    output_dir.child("file-with-dashes.toml")
        .assert(predicate::path::exists());
    output_dir.child("file_with_underscores.toml")
        .assert(predicate::path::exists());
}

/// Test ignoring files and directories with ~ and . prefixes
#[test]
fn test_ignore_tilde_and_dot_prefix_files_and_directories() {
    let input_dir = Path::new("tests/fixtures/test_tilde_ignore");
    let output_dir = assert_fs::TempDir::new().unwrap();

    // Execute conversion
    let stats = md2toml::convert_directory(
        input_dir,
        output_dir.path(),
        false
    ).unwrap();

    // Should only process 2 files: normal.md and normal_folder/inside.md
    assert_eq!(stats.total, 2, "Should find only 2 markdown files (ignoring ~ and . prefixed)");
    assert_eq!(stats.success, 2, "Both files should convert successfully");

    // Verify normal files were converted
    output_dir.child("normal.toml")
        .assert(predicate::path::exists());
    output_dir.child("normal_folder").child("inside.toml")
        .assert(predicate::path::exists());

    // Verify tilde-prefixed files were not converted
    output_dir.child("~ignored.toml")
        .assert(predicate::path::missing());

    // Verify files from tilde-prefixed directories were not converted
    output_dir.child("~hidden_folder").child("hidden.toml")
        .assert(predicate::path::missing());

    // Verify dot-prefixed files were not converted
    output_dir.child(".hidden.toml")
        .assert(predicate::path::missing());

    // Verify files from dot-prefixed directories were not converted
    output_dir.child(".git").child("config.toml")
        .assert(predicate::path::missing());
    output_dir.child(".claude").child("commands.toml")
        .assert(predicate::path::missing());

    // Verify converted content is correct
    let normal_content = fs::read_to_string(output_dir.child("normal.toml")).unwrap();
    assert!(normal_content.contains("title = \"Normal File\""));
    assert!(normal_content.contains("description = \"This is a normal file\""));

    let inside_content = fs::read_to_string(
        output_dir.child("normal_folder").child("inside.toml")
    ).unwrap();
    assert!(inside_content.contains("title = \"Inside Normal Folder\""));
    assert!(inside_content.contains("description = \"This is inside a normal folder\""));
}
