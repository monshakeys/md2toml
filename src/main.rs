use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use md2toml::{InputType, detect_input_type};

#[derive(Parser, Debug)]
#[command(name = "md2toml")]
#[command(about = "Convert Markdown files with YAML frontmatter to TOML format")]
#[command(long_about = r#"Convert Markdown files with YAML frontmatter to TOML format

USAGE:
  # Single file conversion
  md2toml input.md              # Creates input.toml
  md2toml input.md -o out.toml  # Creates out.toml

  # Directory conversion (recursive)
  md2toml docs/                 # Creates docs_toml/ with same structure
  md2toml docs/ -o output/      # Creates output/ with same structure

REQUIREMENTS:
  Markdown files MUST contain both 'title' and 'description' in frontmatter.

SPECIAL REPLACEMENTS:
  $ARGUMENTS  →  {{ args }}
  !`command`  →  !{command}
"#)]
struct Args {
    /// Input path (file or directory)
    input: PathBuf,

    /// Output path (optional)
    /// - For files: defaults to input with .toml extension
    /// - For directories: defaults to <input>_toml/
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Verbose output (show each file being converted)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 智能偵測輸入類型
    match detect_input_type(&args.input) {
        InputType::File(file_path) => {
            convert_single_file(file_path, args.output)?;
        }
        InputType::Directory(dir_path) => {
            convert_directory_mode(dir_path, args.output, args.verbose)?;
        }
        InputType::Invalid(msg) => {
            eprintln!("Error: {}\n", msg);
            eprintln!("Run 'md2toml --help' for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// 轉換單一檔案
fn convert_single_file(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // Read input file
    let input_content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {:?}", input))?;

    // Parse metadata and content
    let metadata = md2toml::parse_frontmatter(&input_content)?;

    // Validate required fields (title and description)
    md2toml::validate_required_fields(&metadata)?;

    let content = md2toml::extract_content(&input_content)?;

    // Apply special replacements
    let processed_content = md2toml::apply_all_replacements(&content);

    // Generate TOML
    let toml_output = md2toml::generate_toml(metadata, processed_content)?;

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("toml");
        path
    });

    // Write output file
    fs::write(&output_path, toml_output)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    println!("Successfully converted {:?} to {:?}", input, output_path);

    Ok(())
}

/// 轉換整個資料夾
fn convert_directory_mode(input_dir: PathBuf, output: Option<PathBuf>, verbose: bool) -> Result<()> {
    // 決定輸出資料夾
    let output_dir = output.unwrap_or_else(|| {
        let dir_name = input_dir.file_name().unwrap().to_str().unwrap();
        PathBuf::from(format!("{}_toml", dir_name))
    });

    // 如果輸出資料夾已存在，詢問使用者
    if output_dir.exists() {
        if !md2toml::confirm_overwrite(&output_dir) {
            println!("Operation cancelled.");
            return Ok(());
        }
        // 刪除舊資料夾
        fs::remove_dir_all(&output_dir)
            .with_context(|| format!("Failed to remove existing directory: {:?}", output_dir))?;
    }

    // 建立輸出資料夾
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    println!("Converting directory: {:?} -> {:?}", input_dir, output_dir);

    // 批次轉換
    let stats = md2toml::convert_directory(&input_dir, &output_dir, verbose)?;

    // 輸出統計
    println!("\n=== Conversion Summary ===");
    println!("Total files found:     {}", stats.total);
    println!("Successfully converted: {}", stats.success);

    if stats.has_failures() {
        println!("Failed:                {}", stats.failed.len());
        println!("\nFailed files:");
        for (path, error) in &stats.failed {
            eprintln!("  ✗ {:?}", path);
            eprintln!("    Error: {}", error);
        }

        // 如果有失敗的檔案，返回錯誤碼
        if stats.success == 0 {
            std::process::exit(1);
        }
    } else {
        println!("\n✓ All files converted successfully!");
    }

    Ok(())
}
