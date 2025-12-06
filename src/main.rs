//! CLI entry point for md2toml conversion tool
//!
//! This module handles user interaction and delegates to the application layer.

use clap::Parser;
use md2toml::application::{ErrorClassifier, StandardDirectoryConversionUseCase};
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

    // Shared dependencies across the application
    let logger: Arc<dyn md2toml::domain::Logger> = Arc::new(SimpleLogger);
    let fs = Arc::new(md2toml::infra::FsAdapter::new());
    let directory_converter = Arc::new(StandardDirectoryConversionUseCase::new(fs.clone()))
        as Arc<dyn md2toml::domain::ports::DirectoryConversionUseCase>;

    // Classify errors for appropriate user presentation
    let error_classifier = ErrorClassifier::new();

    let mut config = Md2TomlConfig::default();

    // CLI arguments override environment configuration
    config = config
        .verbose()
        .with_skip_hidden(args.skip_hidden)
        .with_overwrite(args.overwrite);

    if args.parallel {
        config = config.parallel(args.workers);
    }

    // Configuration validation is handled by Md2TomlConfig::validate()
    let service = match Md2TomlServiceBuilder::new()
        .with_filesystem(fs.clone())
        .with_directory_converter(directory_converter)
        .with_logger(logger.clone())
        .with_config(config)
        .build()
    {
        Ok(service) => service,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            process::exit(1);
        }
    };

    match service.convert(args.input, args.output) {
        Ok(stats) => {
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
            // Apply classification-based error presentation
            let category = error_classifier.classify(&e);
            let exit_code = error_classifier.exit_code(category);

            match category {
                md2toml::application::ErrorCategory::UserInput => {
                    eprintln!("\n❌ Input Error: {}", e);
                    eprintln!("\n💡 Please check:");
                    eprintln!("  • File format is correct");
                    eprintln!("  • Required fields (title, description) are present");
                    eprintln!("  • Frontmatter uses YAML format");
                }
                md2toml::application::ErrorCategory::System => {
                    eprintln!("\n💥 System Error: {}", e);
                    eprintln!("\n🔧 Technical details:");
                    eprintln!("  {:?}", e);
                }
                md2toml::application::ErrorCategory::Transient => {
                    eprintln!("\n⚠️  Temporary Error: {}", e);
                    eprintln!("\n🔄 Suggestion: Please try again");
                }
            }

            eprintln!("\nRun 'md2toml --help' for usage information.");
            process::exit(exit_code);
        }
    }
}
