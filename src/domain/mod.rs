// Domain Layer - Core Business Logic
// Pure business rules, free from infrastructure concerns

pub mod config;
pub mod directory;
pub mod errors;
pub mod generator;
pub mod models;
pub mod parser;
pub mod ports;
pub mod replacer;

pub use config::Md2TomlConfig;
pub use errors::{Md2TomlError, Result};
pub use models::{ConversionStats, FileType, ParsedMarkdown, ProcessingStrategy};
pub use ports::{DirectoryConversionUseCase, DirectoryExplorer, FileSystem, LogLevel, Logger};

// Re-export core business modules
pub use directory::{detect_input_type, map_output_base_path, should_ignore_path};
pub use generator::generate_toml;
pub use models::{ConversionStats as DirectoryConversionStats, InputType};
pub use parser::{
    convert_markdown_to_toml, extract_content, parse_frontmatter, validate_required_fields,
};
pub use replacer::{apply_all_replacements_simple, replace_arguments, TextReplacer};

// Note: collect_files_by_extension, confirm_overwrite, ensure_parent_dir remain in legacy directory.rs
// as they involve I/O operations that shouldn't be in pure domain logic
