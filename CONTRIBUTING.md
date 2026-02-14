# Contributing to opensubsonic

Thank you for your interest in contributing to opensubsonic! This document provides guidelines for contributing to this project.

## Code of Conduct

This project adheres to a standard of respectful and constructive collaboration. Please be kind and professional in all interactions.

## How to Contribute

### Reporting Issues

If you find a bug or have a feature request:

1. Check if the issue already exists in the [issue tracker](https://github.com/M0Rf30/opensubsonic-rs/issues)
2. If not, create a new issue with:
   - A clear title and description
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Your environment (Rust version, OS, etc.)

### Submitting Changes

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards
3. **Add tests** for any new functionality
4. **Run the test suite**: `cargo test`
5. **Run linting**: `cargo clippy` and `cargo fmt`
6. **Update documentation** if needed
7. **Submit a pull request** with a clear description of the changes

### Coding Standards

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for code formatting
- Address all Clippy warnings
- Write documentation for public APIs
- Add examples for new features

### Commit Messages

Use clear and descriptive commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Example:
```
Add support for album artwork caching

Implements client-side caching for album artwork to reduce
API calls. The cache is configurable via the builder pattern.

Fixes #123
```

## Development Setup

```bash
# Clone the repository
git clone https://github.com/M0Rf30/opensubsonic-rs.git
cd opensubsonic-rs

# Build the project
cargo build

# Run tests
cargo test

# Run with all features
cargo test --all-features

# Generate documentation
cargo doc --no-deps

# Check formatting
cargo fmt --check

# Run Clippy
cargo clippy --all-targets --all-features
```

## Testing

All contributions should include tests:

- Unit tests for new functions
- Integration tests for API changes
- Documentation tests for code examples

Run the full test suite before submitting:
```bash
cargo test
cargo test --doc
```

## Documentation

- Use rustdoc comments (`///`) for public APIs
- Include code examples in documentation
- Update the README if adding new features
- Update CHANGELOG.md with your changes

## Release Process

Maintainers will handle releases. The process involves:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release date
3. Create a git tag: `git tag -a vX.Y.Z -m "Version X.Y.Z"`
4. Push the tag: `git push origin vX.Y.Z`
5. The CI will automatically publish to crates.io

## Questions?

Feel free to open an issue for questions or join discussions.

Thank you for contributing!
