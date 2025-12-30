# Contributing to snailDB

Thank you for your interest in contributing to snailDB! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue on GitHub with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Any relevant error messages or logs

### Suggesting Features

We welcome feature suggestions! Please open an issue with:

- A clear description of the proposed feature
- Use cases and motivation
- Any potential implementation considerations

### Submitting Pull Requests

1. *Star the repository* if you find it useful!
2. **Fork the repository** and create a branch from `master`
3. **Make your changes** following the coding standards below
4. **Ensure all tests pass**
   ```bash
   cargo test --all
   ```

5. **Commit your changes** with a clear commit message (see Commit Messages section)
6. **Push to your fork** and open a Pull Request

## Development Setup

### Prerequisites

- Rust (latest stable version recommended)
- Cargo

### Building the Project

```bash
# Clone the repository
git clone https://github.com/Hk669/snaildb.git
cd snaildb

# Build all workspace members
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_operations --package snaildb
```

### Project Structure

- `snaildb/` - Core database library
- `snailctl/` - Command-line tool (WIP)

## Coding Standards

### Rust Style

- Follow Rust's official style guide
- Use meaningful variable and function names
- Add comments for complex logic

### Code Organization

- Keep functions focused and single-purpose
- Handle errors explicitly using `Result` types
- Use appropriate error types

## Testing

- Write tests for new features and bug fixes
- Ensure tests are deterministic and don't depend on external state
- Run the full test suite before submitting:
  ```bash
  cargo test --all
  ```

## Commit Messages

Write clear, descriptive commit messages:

- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 72 characters
- Provide more detail in the body if needed
- Reference issue numbers when applicable

Example:
```
Add bloom filter support for SSTable lookups

This improves read performance by avoiding unnecessary disk reads
when a key is definitely not present in an SSTable.

Fixes #123
```

## Documentation

- Add doc comments to public APIs
- Update README.md if you add new features or change behavior

## Pull Request Process

- All pull requests require review before merging
- Be responsive to feedback and questions
- Keep pull requests focused and reasonably sized

## License

By contributing to snailDB, you agree that your contributions will be licensed under the Apache License, Version 2.0, the same license as the project.

## Questions?

If you have questions, please open an issue on GitHub.

Thank you for contributing to snailDB! 🐌

