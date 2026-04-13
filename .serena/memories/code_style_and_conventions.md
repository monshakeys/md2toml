# Code Style and Conventions - md2toml

## Project Identity

**Name**: md2toml  
**Purpose**: Slash Commands Converter (Claude Code to Gemini CLI)  
**Language**: Rust (Edition 2021)  
**Architecture**: Hexagonal Architecture (Ports & Adapters) with DDD principles  
**License**: MIT

## Core Principles

### 1. Core-System-First Architecture
- **System Root**: `src/lib.rs` contains all business logic, domain models, traits, and application flows
- **Presentation**: `src/main.rs` handles only user interaction (CLI), parameter parsing, and dependency injection (Wiring)
- **Forbidden**: Business logic in `main.rs`

### 2. Context Isolation
- **Layers**: Presentation → Application → Domain ← Infrastructure
- **Dependency Flow**: Strictly from outside to inside (Infra depends on Domain Ports)
- **Separation**: Modify business logic only in `domain/`, modify I/O only in `infra/`

## File Layout Structure

```
src/
├── main.rs                    # [Presentation] CLI interface, parameter parsing, dependency injection
├── lib.rs                     # [System Root] module exports, public API
├── application.rs             # [Application] use case orchestration
├── domain/                    # [Domain] pure business logic (core)
│   ├── config.rs             # Configuration models
│   ├── directory.rs          # Directory processing & filtering
│   ├── errors.rs             # Domain error definitions
│   ├── generator.rs          # TOML output generation
│   ├── models.rs             # Domain entities & value objects
│   ├── parser.rs             # Markdown parsing & frontmatter
│   ├── ports.rs              # Port definitions (traits)
│   └── replacer.rs           # Content replacement logic
├── infra/                     # [Infrastructure] I/O implementations
│   ├── fs_adapter.rs         # File system operations
│   ├── logger.rs             # Logging implementation
│   └── regex_service.rs      # Regex service
└── application/               # Application use cases
    ├── config_loader.rs      # Configuration loading
    └── directory_conversion_use_case.rs
```

## Layer Responsibilities

### Presentation Layer (src/main.rs)
**Allowed**:
- CLI argument parsing (clap)
- UI rendering (Emojis, progress bars)
- Dependency injection (Wiring)

**Forbidden**:
- Business logic or workflow decisions
- Direct I/O operations

### Application Layer (src/application.rs)
**Allowed**:
- Use case orchestration
- Coordinating Domain and Ports
- Transaction control

**Forbidden**:
- Low-level technical details

### Domain Layer (src/domain/)
**Allowed**:
- Core business logic
- State machines
- Model definitions
- Config shape definitions (Structs)
- Port (Trait) definitions

**Forbidden**:
- Pure environment: NO I/O, NO DB drivers, NO println!

### Infrastructure Layer (src/infra/)
**Allowed**:
- Implementing Domain Ports (Adapters)
- Reading config files (File I/O, Env Vars)
- Database operations
- File operations
- Network connections

**Forbidden**:
- UI output (println!), must use Logger Port
- Progress bars must use Callback/Trait, not UI library dependencies

## Code Style Guidelines

### Naming Conventions
- **Structs**: PascalCase (e.g., `Md2TomlConfig`)
- **Enums**: PascalCase (e.g., `FileType`)
- **Traits**: PascalCase (e.g., `DirectoryConversionUseCase`)
- **Functions**: snake_case (e.g., `parse_frontmatter`)
- **Variables**: snake_case (e.g., `metadata_map`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `MAX_BUFFER_SIZE`)
- **Modules**: snake_case (e.g., `domain`, `infra`)

### Error Handling
- **Domain**: Define `Enum` error types, errors as "data" with portability
- **Infra**: Catch external errors (`std::io::Error`) and convert to Domain errors
- **Presentation**: Display errors based on type (colors, emojis, friendly suggestions)

### Ownership and Data Flow
- **Principle**: Prefer borrowing (`&str`, `&T`), ownership only when necessary
- **Architecture**: Service/Adapter injection using **`Arc<T>` (shared)** or **Move (transfer)**
- **Forbidden**: Complex lifetime references (`&'a Service`) in struct fields

### Dependency Injection Strategy
- **Default**: Dynamic Dispatch (`Box<dyn Trait>`)
- **Reason**: Avoid "Generic Contagion" - generic type parameters spread outward
- **Advantage**: Clean Application function signatures, faster compilation, intuitive wiring
- **Exception**: Async I/O or performance-sensitive operations use `impl Future` or `async_trait`

## Trait/Port Pattern

### "Define First, Implement Later" Strategy
1. **Define (Domain)**: Define Trait in `domain/ports.rs` first
2. **Verify**: Confirm input/output interface meets business needs
3. **Implement (Infra)**: Generate concrete implementation in `infra/` after interface is finalized

### Example
```rust
// Domain - Define Port
pub trait DirectoryConversionUseCase {
    fn convert_directory(&self, input: PathBuf, output: Option<PathBuf>) -> Result<ConversionStats, Md2TomlError>;
}

// Infra - Implement Port
pub struct StandardDirectoryConversionUseCase {
    fs: Arc<dyn FileSystem>,
}

impl DirectoryConversionUseCase for StandardDirectoryConversionUseCase {
    fn convert_directory(&self, input: PathBuf, output: Option<PathBuf>) -> Result<ConversionStats, Md2TomlError> {
        // Implementation
    }
}
```

## Testing Standards

### Test Structure
- **Unit Tests**: In `src` files with `#[cfg(test)]`
- **Integration Tests**: In `tests/` directory, feature-based naming (e.g., `tests/login.rs`)
- **Tools**: Use `assert_fs::TempDir` for filesystem tests (NOT `tempfile`)
- **Fixtures**: All test static files in `tests/fixtures/`
- **Access**: Use `CARGO_MANIFEST_DIR` environment variable for path construction

### Layer-Specific Testing
- **Domain**: Unit Tests (Pure Rust) - Pure logic, state machines, edge cases, NO I/O, microsecond-level execution
- **Infrastructure**: Integration Tests - Real I/O, database, file operations, third-party APIs
- **Presentation**: Snapshot Testing (e.g., insta crate) - UI consistency, CLI output format

### Timeout Control Strategy
- **Goal**: Prevent deadlocks/infinite loops, NOT for performance benchmarking
- **Runner**: Use `nextest`
- **Implementation**: Prefer `tokio::time::timeout` wrappers inside test functions
- **Duration**: Must be clearly visible at test start, don't rely on global settings

### Timeout Reference
| Test Type | Duration | Example |
|-----------|----------|---------|
| Pure Logic (CPU-bound) | None | Domain logic, math, string parsing |
| Async Logic (In-Memory) | 1 second | Channels, actors, async mutex |
| External I/O (Network/FS) | 5 seconds | File ops, HTTP, TCP |
| Heavy Integration (DB/Container) | 30-60 seconds | Docker startup, database migration |

## Documentation Standards

### Module Documentation
- Use `//!` for module-level documentation
- Include purpose, examples, and architectural notes
- Example: `src/lib.rs` has comprehensive module docs

### Function Documentation
- Use `///` for public function documentation
- Include parameter descriptions and return values
- Include usage examples in code blocks

### Code Comments
- Use `//` for implementation comments
- Explain "why" not "what"
- Remove redundant comments ("Comments Don't Make Up for Bad Code")

## Rust Specific Conventions

### Imports
- Use explicit imports, avoid glob imports except for `prelude`
- Group imports: std → external → local
- Example:
```rust
use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use serde_yaml;

use md2toml::domain::Md2TomlError;
```

### Error Types
- Use `thiserror` for error type definitions
- Define errors as enums with clear categories
- Provide context in error messages

### Configuration
- Use builder pattern for configuration (e.g., `Md2TomlConfig`)
- Provide `default()` implementation
- Use method chaining for configuration updates

### Logging
- Use Logger trait from Domain layer
- No direct `println!` in Domain or Infra layers
- Use structured logging with appropriate levels

## Security Standards

### Dependency Security
- Use `cargo-audit` regularly to scan `Cargo.lock`
- Ensure all dependencies comply with RustSec database security standards
- Categories in Cargo.toml: see [https://crates.io/category_slugs](https://crates.io/category_slugs)

### File Handling
- Validate all input paths
- Use proper error handling for file operations
- Filter hidden files and system directories by default

## Performance Considerations

### Memory Management
- Use `BTreeMap` for metadata (automatic alphabetical sorting)
- Prefer borrowing over ownership
- Use `Arc<T>` for shared references

### I/O Operations
- Use `walkdir` with filtering for efficient directory traversal
- Filter during traversal to avoid unnecessary reads
- Batch operations when possible

### Regex Performance
- Use `once_cell` for static regex compilation
- Avoid recompiling patterns on each call
- Cache compiled patterns in static variables

## Special Rules for This Project

### Conversion Rules
- **File Filtering**: Ignore files starting with `~` or `.`
- **Required Fields**: `title` and `description` (case-sensitive)
- **Replacements**:
  - `$ARGUMENTS` → `{{args}}`
  - `` !`command` `` → `!{command}`

### Supported File Types
- `.md` → `.toml` (Convert with frontmatter parsing and content processing)
- `.txt` → `.txt` (Copy as-is without conversion)

### Output Format
- Metadata sorted alphabetically (BTreeMap)
- Content preserved exactly (newlines, indentation, special characters)
- TOML format with proper escaping
