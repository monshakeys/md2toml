# Task Completion Checklist - md2toml

## Pre-Commit Checklist

### Code Quality
- [ ] Code compiles without warnings: `cargo check`
- [ ] All tests pass: `cargo test` or `cargo nextest run`
- [ ] Security audit passes: `cargo audit`
- [ ] Code formatting is correct: `cargo fmt -- --check`
- [ ] Clippy linting passes: `cargo clippy -- -D warnings`

### Build Verification
- [ ] Debug build succeeds: `cargo build`
- [ ] Release build succeeds: `cargo build --release`
- [ ] Binary runs correctly: `./target/debug/md2toml --help`

### Architecture Compliance
- [ ] No business logic in `src/main.rs`
- [ ] Domain layer is pure (no I/O, no println!)
- [ ] Infrastructure implements Domain Ports correctly
- [ ] Proper separation of concerns across layers

### Testing
- [ ] Unit tests cover domain logic (no I/O dependencies)
- [ ] Integration tests cover real file operations
- [ ] Test fixtures are in `tests/fixtures/` directory
- [ ] Fixtures accessed via `CARGO_MANIFEST_DIR`
- [ ] Timeout control appropriate for test type:
  - [ ] Pure logic: No timeout
  - [ ] Async in-memory: 1 second
  - [ ] External I/O: 5 seconds
  - [ ] Heavy integration: 30-60 seconds

### Documentation
- [ ] Module documentation (using `//!`) is present
- [ ] Public functions documented (using `///`)
- [ ] README.md is updated if needed
- [ ] Code comments explain "why" not "what"
- [ ] No redundant or outdated comments

### Error Handling
- [ ] Errors defined as enums in Domain layer
- [ ] Infrastructure converts external errors to Domain errors
- [ ] No direct error formatting in Infrastructure
- [ ] User-friendly error messages in Presentation

## Testing Checklist

### Domain Layer Tests
- [ ] Pure Rust logic tests (no external dependencies)
- [ ] State machine transitions tested
- [ ] Edge cases and boundary conditions covered
- [ ] No file system or network operations
- [ ] Fast execution (microsecond-level)

### Infrastructure Layer Tests
- [ ] Real file I/O operations tested
- [ ] External dependencies mocked or isolated
- [ ] Error conditions tested (missing files, permissions, etc.)
- [ ] Proper cleanup of test files

### Integration Tests
- [ ] End-to-end workflow tested
- [ ] CLI argument parsing tested
- [ ] File conversion tested with real fixtures
- [ ] Error scenarios tested (invalid input, corrupted files)

### Test Fixtures
- [ ] All fixtures in `tests/fixtures/` directory
- [ ] Fixture names match test names
- [ ] Fixtures include edge cases and typical scenarios
- [ ] No hardcoded paths in tests

## Performance Checklist

### Optimization
- [ ] Efficient data structures (e.g., `BTreeMap` for sorted metadata)
- [ ] Proper use of borrowing vs ownership
- [ ] Static regex compilation with `once_cell`
- [ ] Minimal allocations in hot paths

### I/O Efficiency
- [ ] Directory filtering during traversal (not after)
- [ ] Batched file operations where possible
- [ ] Appropriate buffer sizes
- [ ] Proper resource cleanup

## Security Checklist

### Dependency Security
- [ ] Run `cargo audit` with no vulnerabilities
- [ ] All dependencies from reputable sources
- [ ] No deprecated or unmaintained dependencies
- [ ] Cargo.lock is committed

### Input Validation
- [ ] All user inputs validated
- [ ] File paths validated and sanitized
- [ ] Hidden/system files properly filtered
- [ ] No path traversal vulnerabilities

### Error Information
- [ ] No sensitive data in error messages
- [ ] Proper error classification
- [ ] Secure fallback behavior

## Documentation Checklist

### README
- [ ] Installation instructions current
- [ ] Usage examples work correctly
- [ ] Architecture section updated if changed
- [ ] Conversion rules documented

### Code Documentation
- [ ] Public API fully documented
- [ ] Examples in documentation compile
- [ ] Complex algorithms explained
- [ ] Design decisions documented

## Git Workflow Checklist

### Before Commit
- [ ] `git status` shows expected changes only
- [ ] `git diff` reviewed for correctness
- [ ] No sensitive data committed (.env, credentials, etc.)
- [ ] .gitignore properly configured

### Commit Message
- [ ] Follows conventional commits format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `test:` for test additions/changes
  - `docs:` for documentation
  - `refactor:` for code refactoring
  - `chore:` for maintenance
- [ ] Clear and descriptive message
- [ ] Imperative mood ("Add" not "Added")

### After Commit
- [ ] Recent commit verified: `git log --oneline -5`
- [ ] Remote push successful: `git push`
- [ ] CI/CD pipeline passes (if applicable)

## Release Checklist

### Pre-Release
- [ ] Version updated in `Cargo.toml`
- [ ] CHANGELOG.md updated (if exists)
- [ ] Release build tested: `cargo build --release`
- [ ] Binary size reasonable
- [ ] Performance benchmarked

### Post-Release
- [ ] Git tag created: `git tag v1.0.0`
- [ ] Tag pushed: `git push origin v1.0.0`
- [ ] GitHub release created (if applicable)
- [ ] Homebrew formula updated (if applicable)

## Feature Development Checklist

### Design Phase
- [ ] Requirements clearly defined
- [ ] Architecture impact assessed
- [ ] Domain layer changes isolated
- [ ] Port definitions created before implementation

### Implementation Phase
- [ ] Domain logic implemented first
- [ ] Ports defined in `domain/ports.rs`
- [ ] Infrastructure implements Ports
- [ ] Application layer orchestrates workflow
- [ ] Presentation layer handles CLI/UI only

### Testing Phase
- [ ] Domain logic tested in isolation
- [ ] Port implementations tested with mocks
- [ ] Integration tests verify end-to-end
- [ ] Error cases thoroughly tested

## Bug Fix Checklist

### Investigation
- [ ] Root cause identified (not just symptoms)
- [ ] Bug reproduced consistently
- [ ] Affected areas cataloged
- [ ] Test case created to prevent regression

### Fix
- [ ] Minimal change to fix the issue
- [ ] No breaking changes (unless intentional)
- [ ] All tests pass after fix
- [ ] Error messages improved if relevant

### Verification
- [ ] Bug no longer reproducible
- [ ] New test case passes
- [ ] Related functionality still works
- [ ] Performance not degraded

## Code Review Checklist

### For Author
- [ ] Self-review completed using this checklist
- [ ] Code follows established patterns
- [ ] Tests included and passing
- [ ] Documentation updated

### For Reviewer
- [ ] Architecture compliance verified
- [ ] Code style consistency checked
- [ ] Test coverage adequate
- [ ] Security considerations addressed
- [ ] Performance impact assessed
- [ ] Documentation sufficient

## Quick Reference Commands

### Full Quality Check
```bash
cargo check && cargo test && cargo audit && cargo fmt -- --check && cargo clippy -- -D warnings
```

### Test with Coverage
```bash
cargo test --all-features
```

### Performance Benchmark
```bash
cargo build --release && hyperfine './target/release/md2toml commands/'
```

### Security Audit
```bash
cargo audit --db /path/to/advisory-db
```

## Notes
- Always run the full quality check before committing
- Use `cargo nextest run` for faster test feedback
- Keep Domain layer pure and independent
- When in doubt, follow the established patterns in existing code
