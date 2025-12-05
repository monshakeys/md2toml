//! md2toml - Slash Commands Converter
//!
//! Core-System-First: All business logic is in lib.rs as the system root.
//! This library provides pure business functions without infrastructure dependencies.
//!
//! # Architecture
//!
//! - **Domain Layer** (`domain/`): Pure business logic, models, and port definitions
//! - **Application Layer** (`application.rs`): Use case orchestration and workflows
//! - **Infrastructure Layer** (`infra/`): I/O implementations (separate crate)
//!
//! # Examples
//!
//! ```rust
//! use md2toml::{Md2TomlService, Md2TomlServiceBuilder, domain::Md2TomlConfig};
//! use md2toml::infra::{FsAdapter, SimpleLogger, StandardDirectoryConversionUseCase};
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
//!
//! // Service is ready to use - see actual usage in CLI
//! # Ok(())
//! # }
//! ```
//!
//! # Core-System-First Architecture
//!
//! This library implements a clean architecture with clear separation of concerns:
//!
//! - **Presentation Layer**: CLI entry point in `src/main.rs`
//! - **Application Layer**: Use case orchestration in `src/application.rs`
//! - **Domain Layer**: Pure business logic in `src/domain/`
//! - **Infrastructure Layer**: I/O implementations in `src/infra/`

// Domain Layer - Pure business logic
pub mod domain;

// Application Layer - Use case orchestration
pub mod application;

// Infrastructure Layer - I/O implementations
pub mod infra;

// Re-export public API from domain
pub use domain::{
    apply_all_replacements_simple, convert_markdown_to_toml, detect_input_type, extract_content,
    generate_toml, map_output_base_path, parse_frontmatter, replace_arguments, should_ignore_path,
    validate_required_fields, ConversionStats, DirectoryConversionUseCase, FileType, InputType,
    Md2TomlError, ParsedMarkdown, ProcessingStrategy, TextReplacer,
};

pub use application::{Md2TomlService, Md2TomlServiceBuilder};

// Re-export commonly used types for convenience
pub use domain::errors::Md2TomlError as Error;
