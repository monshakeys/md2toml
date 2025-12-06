# md2toml - Slash Commands Converter  (Claude Code to Gemini CLI)

> Stop manually converting slash commands. Migrate your entire directory in one command.


## Features

- 🎯 **Batch Directory Conversion**: Convert entire command directories in a single operation.
  > *"Dauthen stared at her 200+ Claude slash commands, knowing manual conversion would take weeks. With `md2toml .claude/commands/`, she watched 200 files transform into Gemini format in under 50ms, leaving her evening free for her art class."*

- ⚡ **Automatic Syntax Translation**: Intelligent conversion of Claude-specific syntax to Gemini CLI format.
  > *"Dauthen used to keep a cheat sheet beside her monitor, constantly looking up `$ARGUMENTS` to `{{args}}` mappings until 3 AM. With md2toml's autopilot translation engine, she now migrates entire projects without memorizing a single syntax rule."*

- 🛡️ **Resilient Partial Conversion**: Continue processing even when some files fail validation.
  > *"When 8 of Dauthen's 34 command files had corrupted frontmatter, she braced for a total failure. Instead, md2toml converted the remaining 26 successfully, showing her exactly which files needed fixing while delivering working TOML for the rest."*

- ✨ **Smart Auto-Filtering**: Automatically skip drafts, backups, and system files.
  > *"What Dauthen loved most was how md2toml's filtering became her personal file management system. She prefixed experimental commands with `~experimental-`, marked half-finished drafts with `~draft-`, and let the tool automatically skip them during conversion while she focused on the commands she was actually ready to use. Her directory stayed organized without manual cleanup—just prefix and convert."*


## Installation
### Homebrew (Recommended)

```sh
brew install monshakeys/tap/md2toml
```

### Alternative Methods

- Download the latest binary from the [releases page](https://github.com/monshakeys/md2toml/releases) and place it in your `$PATH`.
- Or these methods:

```sh
# Clone the repository
git clone git@github.com:monshakeys/md2toml.git
cd md2toml

# Build for development
cargo build
./target/debug/md2toml

# Build for production
cargo build --locked --release
./target/release/md2toml

# Run without building an executable
cargo run              # Debug mode
cargo run --release    # Release mode

# Install to local cargo path
cargo install --path .
```

## Usage

### Batch convert entire directory

```sh
# Simplest usage - convert entire commands directory
md2toml .claude/commands/

# Output to specified location
md2toml .claude/commands/ -o .gemini/commands/

# Show conversion status for each file
md2toml .claude/commands/ --verbose
```

### Single file conversion

```sh
# Convert a single file
md2toml input.md

# Specify output filename
md2toml input.md -o custom.toml
```

### Filtering examples

```sh
# Your directory structure:
commands/
└── .git/
│   └── config.md        # ❌ Auto-skipped (hidden directory)
├── ~backup/
│   └── old.md          # ❌ Auto-skipped (backup directory)
├── dev/
│   └── debug.md        # ✅ Will be converted
├── .DS_Store           # ❌ Auto-skipped (system file)
├── ~draft.md           # ❌ Auto-skipped (draft)
└── review.md           # ✅ Will be converted

# Command
md2toml commands/ -o output/
```


## Project Structure

```
src/
├── main.rs                    # Presentation Layer - CLI interface
├── lib.rs                     # System Root - module exports
├── application.rs             # Application Layer - use case orchestration
├── domain/                    # Domain Layer - pure business logic
│   ├── config.rs             # Configuration models
│   ├── directory.rs          # Directory processing & filtering
│   ├── errors.rs             # Domain error definitions
│   ├── generator.rs          # TOML output generation
│   ├── models.rs             # Domain entities & value objects
│   ├── parser.rs             # Markdown parsing & frontmatter
│   ├── ports.rs              # Port definitions (traits)
│   └── replacer.rs           # Content replacement logic
├── infra/                     # Infrastructure Layer - I/O implementations
│   ├── fs_adapter.rs         # File system operations
│   ├── logger.rs             # Logging implementation
│   └── regex_service.rs      # Regex service
└── application/               # Application use cases
    ├── config_loader.rs      # Configuration loading
    └── directory_conversion_use_case.rs
```

> **Architecture Philosophy**
>
> This project implements a **Hexagonal Architecture** (Ports & Adapters) pattern fused with **Domain-Driven Design (DDD)** principles. The architecture emphasizes **clean separation of concerns** through four distinct layers:
>
> - **Presentation Layer** handles user interaction and CLI concerns
> - **Application Layer** orchestrates business workflows and use cases
> - **Domain Layer** contains pure business logic, entities, and business rules
> - **Infrastructure Layer** implements technical concerns and external dependencies
>
> **Dependency Direction Flow**: `Presentation → Application → Domain ← Infrastructure`
>
> This design ensures **high testability**, **loose coupling**, and **business logic isolation** from technical implementation details. All domain logic can be tested without I/O dependencies.


## Output Examples
### Success case

Input (commands/review.md):
```markdown
---
title: Code Review
description: Review code changes
---

Review $ARGUMENTS and provide feedback.
Use !`git diff` to check changes.
```

Output (output/review.toml):
```toml
description = "Review code changes"
title = "Code Review"

prompt = """
Review {{args}} and provide feedback.
Use !{git diff} to check changes.
"""
```

### Failure case

Input (commands/broken.md):
```markdown
---
title: Broken Command
---

Missing description field!
```

Output:
```
✗ "commands/broken.md"
  Error: Invalid frontmatter format: This is not a markdown file for cc slash command.
```

> why it failed: Missing required `description` field. Fix it and re-run the conversion.


## Official Format References

- Gemini CLI: [Gemini CLI Custom Commands](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/custom-commands.md)
- Claude Code: [Claude Code Custom Slash Commands](https://code.claude.com/docs/en/slash-commands#custom-slash-commands)


## FAQ
### Why no reverse conversion?  (Gemini CLI to Claude Code)

- Short answer: Never!
- The technical reason: This is a matter of **architectural asymmetry**. Claude Code and Gemini CLI serve fundamentally different design philosophies. Converting **from Claude Code to Gemini CLI** is a projection; converting back is hallucination.
  > - **Claude Code (cc)** is an engine for **High-Context Engineering**. It operates on massive *implicit state*: cursor position, active selection, dependency trees, and conversational history. It is dynamic, semantic, and alive.
  > - **Gemini CLI (gc)** is a tool for **Deterministic Automation**. Its TOML format is designed to be static, portable, and explicit. It is low-context by design.

---

*P.S. This entire project was built **with no code**. Every line of Rust, every test, and every optimization is the product of **context engineering**.*
