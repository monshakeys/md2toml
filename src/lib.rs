//! md2toml - Slash Commands Converter
//!
//! Core-System-First architecture with clear separation of concerns:
//! - **Domain Layer**: Pure business logic without infrastructure dependencies
//! - **Application Layer**: Use case orchestration
//! - **Infrastructure Layer**: I/O implementations
//!
//! # Examples
//!
//! ```rust
//! use md2toml::{Md2TomlService, Md2TomlServiceBuilder, StandardDirectoryConversionUseCase, domain::Md2TomlConfig};
//! use md2toml::infra::{FsAdapter, SimpleLogger};
//! use std::sync::Arc;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Md2TomlConfig::default();
//! let fs = Arc::new(FsAdapter::new());
//! let directory_converter = Arc::new(StandardDirectoryConversionUseCase::new(fs.clone()));
//! let service = Md2TomlServiceBuilder::new()
//!     .with_filesystem(fs.clone())
//!     .with_directory_converter(directory_converter)
//!     .with_logger(Arc::new(SimpleLogger))
//!     .with_config(config)
//!     .build();
//! # Ok(())
//! # }
//! ```

pub mod domain;

pub mod application;

pub mod infra;

pub use domain::{
    apply_all_replacements_simple, convert_markdown_to_toml, detect_input_type, extract_content,
    generate_toml, map_output_base_path, parse_frontmatter, replace_arguments, should_ignore_path,
    validate_required_fields, ConversionStats, DirectoryConversionUseCase, FileType, InputType,
    Md2TomlError, ParsedMarkdown, ProcessingStrategy, TextReplacer,
};

pub use application::{Md2TomlService, Md2TomlServiceBuilder, StandardDirectoryConversionUseCase};

pub use domain::errors::Md2TomlError as Error;
