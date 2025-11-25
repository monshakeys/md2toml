pub mod error;
pub mod parser;
pub mod replacer;
pub mod generator;
pub mod directory;

// 匯出錯誤類型
pub use error::{Md2TomlError, Result};

// 匯出優化版本的解析函數和結構體
pub use parser::{ParsedMarkdown, parse_markdown, parse_frontmatter, extract_content, validate_required_fields};
pub use replacer::{replace_arguments, replace_backtick_patterns, apply_all_replacements};
pub use generator::generate_toml;

// 匯出資料夾處理功能
pub use directory::{
    InputType, detect_input_type, collect_md_files, map_output_path,
    ConversionStats, confirm_overwrite, ensure_parent_dir, convert_directory,
    should_ignore_path,
};
