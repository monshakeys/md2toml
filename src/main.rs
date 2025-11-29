use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use md2toml::{InputType, detect_input_type};

// ANSI color codes for modern terminal output
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const DIM: &str = "\x1b[2m";

    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const CYAN: &str = "\x1b[36m";
    pub const RED: &str = "\x1b[31m";

    pub const BOLD_GREEN: &str = "\x1b[1;32m";
    pub const BOLD_BLUE: &str = "\x1b[1;34m";
    pub const BOLD_CYAN: &str = "\x1b[1;36m";
    pub const BOLD_RED: &str = "\x1b[1;31m";
}

// Helper functions for styled output
fn print_header(text: &str) {
    println!("\n{}┌─{}─┐{}", colors::BOLD_CYAN, "─".repeat(text.len()), colors::RESET);
    println!("{}│ {} │{}", colors::BOLD_CYAN, text, colors::RESET);
    println!("{}└─{}─┘{}", colors::BOLD_CYAN, "─".repeat(text.len()), colors::RESET);
}

fn print_section(title: &str) {
    println!("\n{}▸ {}{}", colors::BOLD_BLUE, title, colors::RESET);
}

fn print_success(msg: &str) {
    println!("{}✓{} {}", colors::BOLD_GREEN, colors::RESET, msg);
}

fn print_error(msg: &str) {
    eprintln!("{}✗{} {}", colors::BOLD_RED, colors::RESET, msg);
}

fn print_info(msg: &str) {
    println!("{}ℹ{} {}", colors::BOLD_BLUE, colors::RESET, msg);
}

fn print_stat(label: &str, value: &str, color: &str) {
    println!("  {}{:.<20}{} {}{}{}",
        colors::DIM, label, colors::RESET,
        color, value, colors::RESET);
}

#[derive(Parser, Debug)]
#[command(name = "md2toml")]
#[command(version)]
#[command(author)]
#[command(about = "🔄 md2toml - Slash Commands Converter", long_about = None)]
#[command(after_help = r#"EXAMPLES:
    # Convert a single file
    md2toml input.md                    → Creates input.toml
    md2toml input.md -o output.toml     → Creates output.toml

    # Convert entire directory (recursive)
    md2toml docs/                       → Creates docs_toml/ with same structure
    md2toml docs/ -o output/            → Creates output/ with same structure
    md2toml docs/ -v                    → With verbose output

REQUIREMENTS:
    • Markdown files MUST contain 'title' and 'description' in frontmatter
    • Frontmatter must use YAML format (enclosed in ---)

SPECIAL REPLACEMENTS:
    $ARGUMENTS   →   {{args}}
    !`command`   →   !{command}

SUPPORTED FILE TYPES:
    .md files    → Converted to .toml with content processing
    .txt files   → Copied as-is (no conversion)

For more information, visit: https://github.com/yourusername/md2toml
"#)]
struct Args {
    /// Input file or directory path
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output file or directory path
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Show detailed progress for each file
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

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

/// Processes a single file based on its extension
fn convert_single_file(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // Determine processing strategy based on file extension
    let ext = input.extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Cannot determine file extension"))?;

    match ext {
        "md" => convert_markdown_file(input, output),
        "txt" => copy_text_file(input, output),
        _ => Err(anyhow::anyhow!("Unsupported file type: .{}", ext)),
    }
}

/// Converts a single markdown file to TOML format
fn convert_markdown_file(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    print_info(&format!("Converting: {}", input.display()));

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
    let toml_output = md2toml::generate_toml(&metadata, &processed_content)?;

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("toml");
        path
    });

    // Write output file
    fs::write(&output_path, toml_output)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    print_success(&format!("Created: {}", output_path.display()));

    Ok(())
}

/// Copies a text file as-is
fn copy_text_file(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    print_info(&format!("Copying: {}", input.display()));

    // Determine output path (keep .txt extension)
    let output_path = output.unwrap_or_else(|| input.clone());

    // Copy file
    fs::copy(&input, &output_path)
        .with_context(|| format!("Failed to copy file from {:?} to {:?}", input, output_path))?;

    print_success(&format!("Copied: {}", output_path.display()));

    Ok(())
}

/// Converts all markdown files in a directory to TOML format
fn convert_directory_mode(input_dir: PathBuf, output: Option<PathBuf>, verbose: bool) -> Result<()> {
    let output_dir = output.unwrap_or_else(|| {
        let dir_name = input_dir
            .file_name()
            .expect("BUG: input_dir should have been validated by detect_input_type")
            .to_str()
            .expect("BUG: directory name contains non-UTF8 characters");
        PathBuf::from(format!("{}_toml", dir_name))
    });

    if output_dir.exists() {
        if !md2toml::confirm_overwrite(&output_dir) {
            println!("{}Operation cancelled{}", colors::YELLOW, colors::RESET);
            return Ok(());
        }
        fs::remove_dir_all(&output_dir)
            .with_context(|| format!("Failed to remove existing directory: {:?}", output_dir))?;
    }

    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    print_header("Starting Conversion");
    println!("{}Source:{} {}", colors::DIM, colors::RESET, input_dir.display());
    println!("{}Target:{} {}", colors::DIM, colors::RESET, output_dir.display());

    let stats = md2toml::convert_directory(&input_dir, &output_dir, verbose)?;

    print_header("Conversion Summary");
    
    print_stat("Total files", &stats.total.to_string(), colors::CYAN);
    print_stat("Succeeded", &stats.success.to_string(), colors::GREEN);

    if stats.has_failures() {
        print_stat("Failed", &stats.failed.len().to_string(), colors::RED);
        
        print_section("Failed Files");
        for (path, error) in &stats.failed {
            print_error(&format!("{}", path.display()));
            eprintln!("    {}{}{}",
                colors::DIM, error, colors::RESET);
        }

        if stats.success == 0 {
            println!();
            std::process::exit(1);
        }
    } else {
        println!();
        print_success("All files converted successfully!");
    }

    Ok(())
}
