# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-10-16

### Fixed
- Fixed asset name matching for crates-lsp downloads
- Changed to use compressed archives (.tar.gz for Unix, .zip for Windows) instead of raw binaries
- Corrected DownloadedFileType to handle archive extraction properly

### Changed
- Updated asset names to match actual GitHub release format

## [0.1.0] - 2025-10-16

### Added
- Initial release of Crates Autocomplete extension for Zed
- Integrated crates-lsp language server for Cargo.toml support
- Automatic download and installation of crates-lsp binary
- Support for macOS (Intel and Apple Silicon), Linux (x86_64 and ARM64), and Windows (x86_64)
- Crate name autocomplete from crates.io
- Version autocomplete with latest version suggestions
- Feature autocomplete for crates
- Outdated dependency warnings with inline hints
- Configurable diagnostics severity levels
- Workspace configuration support
- Comprehensive documentation (README, DEVELOPMENT, TESTING, QUICKSTART guides)
- MIT License

### Features
- **Crate Autocomplete**: Intelligent suggestions for crate names while typing
- **Version Suggestions**: Shows latest available versions from crates.io
- **Feature Support**: Autocomplete for crate features
- **Update Notifications**: Visual indicators for outdated dependencies
- **Inlay Hints**: Inline package status information
- **Cross-Platform**: Works on macOS, Linux, and Windows

[Unreleased]: https://github.com/yourusername/crates-autocomplete/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/yourusername/crates-autocomplete/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/yourusername/crates-autocomplete/releases/tag/v0.1.0
