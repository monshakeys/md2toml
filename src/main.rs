//! CLI entry point for md2toml conversion tool
//!
//! This is the presentation layer that orchestrates user interaction,
//! delegates to the application layer, and handles all output formatting.
//!

use clap::Parser;
use md2toml::application::StandardDirectoryConversionUseCase;
use md2toml::domain::Md2TomlConfig;
use md2toml::infra::SimpleLogger;
use md2toml::Md2TomlServiceBuilder;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

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
    md2toml -v                          → With verbose output

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

    /// Skip files and directories starting with ~ or .
    #[arg(long, default_value_t = true)]
    skip_hidden: bool,

    /// Overwrite existing output files
    #[arg(short, long)]
    overwrite: bool,

    /// Enable parallel processing
    #[arg(long)]
    parallel: bool,

    /// Number of worker threads (default: number of CPUs)
    #[arg(long, value_name = "N")]
    workers: Option<usize>,
}

fn main() {
    let args = Args::parse();

    // Arc-based dependency injection for shared dependencies
    let logger: Arc<dyn md2toml::domain::Logger> = Arc::new(SimpleLogger);

    let fs = Arc::new(md2toml::infra::FsAdapter::new());
    let directory_converter = Arc::new(StandardDirectoryConversionUseCase::new(fs.clone()))
        as Arc<dyn md2toml::domain::ports::DirectoryConversionUseCase>;

    let mut config = Md2TomlConfig::default();

    // CLI args take precedence over environment variables
    config = config
        .verbose()
        .with_skip_hidden(args.skip_hidden)
        .with_overwrite(args.overwrite);

    if args.parallel {
        config = config.parallel(args.workers);
    }

    // Follows Rust guidelines: Arc<T> for sharing dependencies
    // Configuration validation happens in Application layer
    let service = match Md2TomlServiceBuilder::new()
        .with_filesystem(fs.clone())
        .with_directory_converter(directory_converter)
        .with_logger(logger.clone())
        .with_config(config)
        .build()
    {
        Ok(service) => service,
        Err(e) => {
            // Presentation layer handles error formatting
            eprintln!("Configuration error: {}", e);
            process::exit(1);
        }
    };

    match service.convert(args.input, args.output) {
        Ok(stats) => {
            // Application layer returns pure data structures (ConversionStats)
            // This is the correct location for UI output logic
            println!("\n✓ Conversion completed successfully!");
            println!("  Total: {}", stats.total);
            println!("  Succeeded: {}", stats.success);

            if stats.has_failures() {
                println!("  Failed: {}", stats.failed.len());
                eprintln!("\nFailed files:");
                for (path, error) in &stats.failed {
                    eprintln!("  {}: {}", path.display(), error);
                }
                process::exit(1);
            }
        }
        Err(e) => {
            // Error formatting belongs in Presentation layer
            let exit_code = match e {
                md2toml::Md2TomlError::InvalidFrontmatter(_) => 2,
                md2toml::Md2TomlError::IoError => 3,
                md2toml::Md2TomlError::YamlParseError(_) => 4,
                md2toml::Md2TomlError::PathError(_) => 5,
                _ => 1,
            };

            eprintln!("\n✗ Error: {}", e);
            eprintln!("\nRun 'md2toml --help' for usage information.");
            process::exit(exit_code);
        }
    }
}