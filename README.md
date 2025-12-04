# md2toml - Slash Commands Converter  (Claude Code to Gemini CLI)

> Stop manually converting slash commands. Migrate your entire directory in one command.


## Features

- 🎯 Migrate everything in one command - `md2toml .claude/commands/` and you're done. **50 commands converted in seconds.** 100? Same. Directory structure stays intact, files ready to use.
- ⚡ Syntax translation on autopilot - Claude's `$ARGUMENTS` becomes Gemini's `{{args}}`. `` !`git status` `` becomes `!{git status}`. **No memorization**. **No lookup**. Just works.
- 🛡️ Partial failures handled gracefully - 8 files broken? **Other 26 still convert successfully.** Each file is independent. You get detailed error reports for failures, working TOML for wins.
- ✨ Smart filtering by default - Automatically skips draft files (`~draft.md`), backup folders (`~old/`), system junk (`.DS_Store`, `.git/`). **No manual cleanup required.**


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
├── main.rs         # CLI interface
├── lib.rs          # library entry point
├── parser.rs       # yaml frontmatter parsing + validation
├── replacer.rs     # syntax conversion
├── generator.rs    # toml output generation
├── directory.rs    # batch processing + smart filtering
└── error.rs        # error handling
```


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
