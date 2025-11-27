pub mod error;
pub mod parser;
pub mod replacer;
pub mod generator;
pub mod directory;

pub use error::{Md2TomlError, Result};

pub use parser::{ParsedMarkdown, parse_markdown, parse_frontmatter, extract_content, validate_required_fields};
pub use replacer::{replace_arguments, replace_backtick_patterns, apply_all_replacements};
pub use generator::generate_toml;

pub use directory::{
    InputType, detect_input_type, collect_files_by_extension, map_output_base_path,
    ConversionStats, confirm_overwrite, ensure_parent_dir, convert_directory,
    should_ignore_path,
};
