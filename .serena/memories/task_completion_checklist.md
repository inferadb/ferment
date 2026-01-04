# Task Completion Checklist

Before considering a task complete, ensure the following:

## 1. Code Quality
- [ ] Run `cargo fmt` to format code
- [ ] Run `cargo clippy` and address any warnings
- [ ] Run `cargo check` to verify compilation

## 2. Testing
- [ ] Run `cargo test` to ensure all tests pass
- [ ] Add tests for new functionality when appropriate
- [ ] Test examples still compile: `cargo build --examples`

## 3. Documentation
- [ ] Add doc comments for new public items
- [ ] Update README.md if adding new features
- [ ] Update CHANGELOG.md for significant changes

## 4. Build Verification
- [ ] `cargo build` succeeds
- [ ] `cargo build --release` succeeds

## Quick Verification Command
```bash
cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build --examples
```

## Before Committing
```bash
# Full check
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```
