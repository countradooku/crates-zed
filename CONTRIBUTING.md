# Contributing to Crates Autocomplete

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the Crates Autocomplete extension for Zed.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## Getting Started

### Prerequisites

- Rust installed via [rustup](https://rustup.rs/)
- Zed editor
- Git
- Familiarity with Rust and WebAssembly

### Setting Up Your Development Environment

1. **Fork the repository**
   ```bash
   # On GitHub, click "Fork" button
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/yourusername/crates-autocomplete
   cd crates-autocomplete
   ```

3. **Add the upstream remote**
   ```bash
   git remote add upstream https://github.com/originalowner/crates-autocomplete
   ```

4. **Install WebAssembly target**
   ```bash
   rustup target add wasm32-wasip1
   ```

5. **Build the extension**
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

6. **Install as dev extension in Zed**
   - Open Zed
   - `Cmd/Ctrl + Shift + P` → "zed: extensions"
   - Click "Install Dev Extension"
   - Select the project directory

## Development Workflow

### Making Changes

1. **Create a new branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes**
   - Write clean, documented code
   - Follow Rust best practices
   - Add tests if applicable

3. **Test your changes**
   ```bash
   # Check for compilation errors
   cargo check

   # Run tests
   cargo test

   # Check formatting
   cargo fmt --all -- --check

   # Run clippy
   cargo clippy --all-features -- -D warnings

   # Build the extension
   cargo build --target wasm32-wasip1 --release
   ```

4. **Test in Zed**
   - Reload the dev extension in Zed
   - Test with real Cargo.toml files
   - Verify all features work as expected

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Examples:**
```bash
feat(autocomplete): add support for workspace dependencies
fix(download): correct asset name matching for Windows
docs(readme): update installation instructions
```

### Pull Request Process

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request**
   - Go to GitHub and create a PR
   - Fill out the PR template completely
   - Link any related issues
   - Add screenshots/recordings if applicable

4. **Code Review**
   - Respond to review comments
   - Make requested changes
   - Push updates to your branch

5. **Merge**
   - Once approved, your PR will be merged
   - Delete your feature branch

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` and fix all warnings
- Write self-documenting code with clear variable names
- Add comments for complex logic

### Code Organization

```rust
// Imports grouped and sorted
use std::fs;
use zed_extension_api::{self as zed, Result};

// Constants at the top
const DEFAULT_TIMEOUT: u64 = 30;

// Types and structs
struct ExtensionName {
    field: Type,
}

// Implementation
impl ExtensionName {
    fn new() -> Self {
        // implementation
    }

    // Public methods first
    pub fn public_method(&self) {}

    // Private methods after
    fn private_method(&self) {}
}
```

### Documentation

- Add rustdoc comments for public APIs
- Include examples in documentation
- Update README.md for user-facing changes
- Update DEVELOPMENT.md for developer changes

### Testing

- Write unit tests for new functionality
- Test edge cases and error handling
- Test on multiple platforms if possible
- Update TESTING.md with new test scenarios

## Reporting Issues

### Bug Reports

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml) and include:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- Zed and extension versions
- Operating system
- Relevant logs

### Feature Requests

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml) and include:
- Problem statement
- Proposed solution
- Examples of how it would work
- Any alternatives considered

## Release Process

Releases are handled by maintainers:

1. Update version in `Cargo.toml` and `extension.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.x.x`
4. Push the tag: `git push --tags`
5. GitHub Actions will create the release automatically

## CI/CD

All pull requests must pass CI checks:
- ✅ Code compiles without errors
- ✅ Tests pass
- ✅ Code is formatted (`cargo fmt`)
- ✅ No clippy warnings
- ✅ WebAssembly build succeeds
- ✅ Extension validation passes

## Project Structure

```
crates_autocomplete/
├── .github/
│   ├── workflows/          # CI/CD workflows
│   ├── ISSUE_TEMPLATE/     # Issue templates
│   └── PULL_REQUEST_TEMPLATE.md
├── src/
│   └── lib.rs              # Main extension code
├── target/                 # Build artifacts (ignored)
├── Cargo.toml              # Rust dependencies
├── extension.toml          # Extension metadata
├── README.md               # User documentation
├── DEVELOPMENT.md          # Developer guide
├── TESTING.md              # Testing guide
├── CONTRIBUTING.md         # This file
├── CHANGELOG.md            # Version history
└── LICENSE                 # MIT license
```

## Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [zed_extension_api Docs](https://docs.rs/zed_extension_api)
- [crates-lsp Repository](https://github.com/MathiasPius/crates-lsp)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly Documentation](https://webassembly.org/)

## Getting Help

- Check existing issues and discussions
- Read the documentation (README, DEVELOPMENT, TESTING)
- Ask questions in issues or discussions
- Join the Zed community

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes
- README acknowledgments (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Crates Autocomplete! 🚀
