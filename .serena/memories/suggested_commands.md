# Suggested Commands for md2toml Development

## Build Commands

### Development Build
```sh
cargo build
# Builds debug executable at ./target/debug/md2toml
```

### Production Build
```sh
cargo build --locked --release
# Builds optimized executable at ./target/release/md2toml
```

### Run Without Building
```sh
cargo run              # Debug mode
cargo run --release    # Release mode
```

### Install Locally
```sh
cargo install --path .
# Installs to ~/.cargo/bin/md2toml
```

## Testing Commands

### Run All Tests
```sh
cargo test
# Runs unit and integration tests
```

### Run Tests with Nextest (Recommended)
```sh
cargo nextest run
# Faster test execution with better output
```

### Run Specific Test
```sh
cargo test test_integration_simple
# Run specific test by name
```

### Test with Output
```sh
cargo test -- --nocapture
# Show println! output during tests
```

### Run Integration Tests Only
```sh
cargo test --test integration_tests
```

## Code Quality Commands

### Security Audit
```sh
cargo audit
# Checks Cargo.lock for security vulnerabilities
```

### Check Code
```sh
cargo check
# Fast compile check without building
```

### Format Code
```sh
cargo fmt
# Formats code according to rustfmt standards
cargo fmt -- --check
# Check formatting without applying changes
```

### Lint with Clippy
```sh
cargo clippy
# Run linting checks
cargo clippy -- -D warnings
# Treat warnings as errors
```

### Full CI Check
```sh
cargo check && cargo test && cargo audit && cargo fmt -- --check && cargo clippy -- -D warnings
# Complete quality check pipeline
```

## Project Usage Examples

### Convert Single File
```sh
md2toml input.md
# Creates input.toml

md2toml input.md -o custom.toml
# Creates custom.toml
```

### Convert Directory
```sh
md2toml .claude/commands/
# Converts entire directory

md2toml .claude/commands/ -o .gemini/commands/
# Convert to specific output directory

md2toml commands/ --verbose
# Show detailed progress
```

### Advanced Options
```sh
md2toml commands/ -o output/ --overwrite
# Overwrite existing files

md2toml commands/ --skip-hidden false
# Process hidden files (default: true)
```

## Git Commands

### Standard Workflow
```sh
git add .
git commit -m "message"
git push

# With conventional commits
git commit -m "feat: add new conversion rule"
git commit -m "fix: handle edge case in parser"
git commit -m "test: add integration test for X"
```

### View Changes
```sh
git diff
# Show unstaged changes

git diff --staged
# Show staged changes

git log --oneline -10
# Show recent commits
```

## System Commands (Darwin)

### File Operations
```sh
ls -la                    # List files with details
find . -name "*.rs"       # Find all Rust files
grep -r "pattern" src/    # Search in source code
tree src -I target        # Show directory structure
```

### Process Management
```sh
ps aux | grep md2toml     # Check running processes
killall md2toml          # Kill all md2toml processes
```

### Development Tools
```sh
which cargo              # Check cargo installation
which rustc              # Check rustc installation
rustc --version          # Check Rust version
cargo --version          # Check Cargo version
```

## Quick Reference

| Command | Purpose |
|---------|---------|
| `cargo build` | Debug build |
| `cargo build --release` | Release build |
| `cargo test` | Run tests |
| `cargo audit` | Security check |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint code |
| `md2toml input.md` | Convert single file |
| `md2toml dir/` | Convert directory |

## Performance Tips

- Use `cargo check` instead of `cargo build` during development for faster feedback
- Use `cargo nextest run` for faster test execution
- Use `cargo build --release` for production benchmarking
- Use `--verbose` flag for detailed conversion progress
- Use `--skip-hidden` to control filtering behavior

## Error Handling

- Check logs with `cargo test -- --nocapture` for debug output
- Use `--verbose` flag to see which files are being processed
- Check exit codes: 0 = success, 1 = failures occurred
- Review failed files list after conversion completion
