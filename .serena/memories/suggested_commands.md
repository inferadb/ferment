# Suggested Commands

## Build & Check
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Check for compilation errors without building
cargo check

# Build documentation
cargo doc --open
```

## Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name::
```

## Linting & Formatting
```bash
# Format code
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run clippy linter
cargo clippy

# Run clippy with all warnings as errors
cargo clippy -- -D warnings
```

## Running Examples
```bash
# Run a specific example
cargo run --example form_demo
cargo run --example styling
cargo run --example layout
cargo run --example theming
```

## System Utilities (Darwin/macOS)
```bash
# Git operations
git status
git add <file>
git commit -m "message"
git push

# File system
ls -la
find . -name "*.rs"
grep -r "pattern" src/

# Process management
ps aux | grep cargo
```

## Dependency Management
```bash
# Update dependencies
cargo update

# Add a new dependency
cargo add <crate-name>

# Check for outdated dependencies
cargo outdated  # (requires cargo-outdated)
```
