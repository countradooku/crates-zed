# Crates Autocomplete for Zed

A Zed extension that provides intelligent autocomplete, versioning, and features for Rust crates in `Cargo.toml` files. This extension wraps the excellent [crates-lsp](https://github.com/MathiasPius/crates-lsp) language server.

## Features

- **Crate Name Autocomplete**: Get intelligent suggestions while typing crate names
- **Version Autocomplete**: Automatically suggest available versions with smart completion
- **Outdated Version Indicators**: Visual inlay hints showing when dependencies need updating
- **VS Code-style Inlay Hints**: Display package status inline (✅ for up-to-date, ❌ with version for updates needed)
- **Smart Diagnostics**: Configurable warnings and errors for outdated and unknown dependencies
- **Hover Information**: Detailed package information when hovering over dependencies
- **Smart Caching**: Local caching to minimize API calls to crates.io

### Current Limitations

**Note**: The underlying `crates-lsp` language server currently has some limitations:

- **Features autocomplete** is not yet supported by crates-lsp (this is a limitation of the LSP itself, not this extension)
- **Version list display** depends on your editor's LSP completion UI
- For the best experience similar to VS Code or JetBrains, ensure your Zed editor settings have LSP completions enabled

We've optimized the extension configuration to provide the best possible experience within these constraints.

## Installation

### From Zed Extensions Gallery

1. Open Zed
2. Press `Cmd/Ctrl + Shift + P` to open the command palette
3. Type "extensions" and select "zed: extensions"
4. Search for "Crates Autocomplete"
5. Click Install

### Manual Installation (Development)

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/crates-autocomplete
   cd crates-autocomplete
   ```

2. Build the extension:
   ```bash
   cargo build --release
   ```

3. Install as a dev extension in Zed:
   - Open Zed
   - Go to Extensions panel
   - Click "Install Dev Extension"
   - Select the extension directory

## Usage

Once installed, the extension will automatically activate when you open a `Cargo.toml` file. Simply start typing in the dependencies section:

```toml
[dependencies]
serde = "1.0"  # Autocomplete will suggest versions and features
tokio = { version = "1.0", features = ["full"] }  # Feature autocomplete available
```

### Features

- Type a crate name to see autocomplete suggestions
- The extension will show if a crate version is outdated
- Hover over dependencies to see more information
- Use `Ctrl+Space` to manually trigger autocomplete

## Configuration

The extension comes with **VS Code-style defaults** optimized for the best experience, matching the popular VS Code "crates" extension. You can customize these in your Zed settings file (`settings.json`):

```json
{
  "lsp": {
    "crates-lsp": {
      "settings": {
        "inlay_hints": true,
        "diagnostics": true,
        "up_to_date_hint": "✅",
        "needs_update_hint": "❌ {}",
        "needs_update_severity": 2,
        "unknown_dep_severity": 1,
        "up_to_date_severity": 4,
        "cache_directory": null
      }
    }
  }
}
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `inlay_hints` | boolean | `true` | Toggle visual hints for package status |
| `diagnostics` | boolean | `true` | Enable/disable diagnostics for outdated packages |
| `up_to_date_hint` | string | `"✅"` | Text shown next to up-to-date dependencies (VS Code style) |
| `needs_update_hint` | string | `"❌ {}"` | Text shown for packages needing updates (VS Code style, {} = new version) |
| `needs_update_severity` | integer | `2` | Severity for outdated packages (1=ERROR, 2=WARNING, 3=INFO, 4=HINT) |
| `unknown_dep_severity` | integer | `1` | Severity for unknown/not-found packages (ERROR level like VS Code) |
| `up_to_date_severity` | integer | `4` | Severity for up-to-date packages (subtle HINT level) |
| `cache_directory` | string | OS-specific | Custom directory for caching crates.io data |

## VS Code Comparison

This extension provides a similar experience to the popular VS Code "crates" extension with these features:

| Feature | VS Code Extension | Crates Autocomplete (Zed) | Status |
|---------|-------------------|---------------------------|--------|
| Crate name autocomplete | ✅ | ✅ | Full support |
| Version autocomplete | ✅ | ✅ | Full support |
| Inline version hints | ✅ Compatible (✅) / Incompatible (❌) | ✅ Up-to-date (✅) / Needs update (❌) | Equivalent |
| Outdated warnings | ✅ | ✅ | Full support |
| Unknown crate errors | ✅ | ✅ | Full support |
| Hover documentation | ✅ | ✅ | Full support |
| Features autocomplete | ❌ (deprecated extension) | ⚠️ (LSP limitation) | Not yet available |
| Update commands | ✅ | ⚠️ Coming soon | Planned |

**Default Settings Match VS Code**: The extension now uses VS Code-style visual indicators (✅/❌) and severity levels by default for a familiar experience.

### Customizing the Visual Style

If you prefer different visual indicators, you can customize them:

```json
{
  "lsp": {
    "crates-lsp": {
      "settings": {
        "up_to_date_hint": "✓",        // Simple checkmark
        "needs_update_hint": "⬆ {}",   // Arrow with version
        "needs_update_severity": 3      // INFO level for less prominence
      }
    }
  }
}
```

### Note on Features Autocomplete

Currently, `crates-lsp` does not support autocomplete for crate features. This is a known limitation of the language server itself. For features, you'll need to:

1. Visit the crate's documentation on [docs.rs](https://docs.rs)
2. Check the crate's README on [crates.io](https://crates.io)
3. Or use `cargo add <crate> --features <feature>` to see available features

We're monitoring [crates-lsp development](https://github.com/MathiasPius/crates-lsp) for future support of this feature.

## How It Works

This extension integrates the [crates-lsp](https://github.com/MathiasPius/crates-lsp) language server into Zed. The language server:

1. Parses your `Cargo.toml` file
2. Queries crates.io for available versions and features
3. Provides intelligent autocomplete suggestions
4. Caches results to minimize API calls
5. Shows diagnostics for outdated dependencies

The extension automatically downloads and manages the crates-lsp binary for your platform.

## Supported Platforms

- macOS (Intel and Apple Silicon)
- Linux (x86_64 and ARM64)
- Windows (x86_64)

## Troubleshooting

### Extension not activating

Make sure you're editing a `Cargo.toml` file in a Rust project with a valid `Cargo.toml` at the root.

### Autocomplete not working

1. Check the Zed log for errors: `Cmd/Ctrl + Shift + P` → "zed: open log"
2. Verify the extension is installed: Extensions panel
3. Try restarting Zed

### Manual installation of crates-lsp

If you prefer to manage the language server yourself:

```bash
cargo install --locked crates-lsp
```

The extension will automatically detect and use the system-installed version.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Credits

This extension wraps the [crates-lsp](https://github.com/MathiasPius/crates-lsp) language server created by [Mathias Pius Hennig](https://github.com/MathiasPius).

## License

MIT License - see LICENSE file for details

## Links

- [GitHub Repository](https://github.com/yourusername/crates-autocomplete)
- [crates-lsp](https://github.com/MathiasPius/crates-lsp)
- [Zed Extension Documentation](https://zed.dev/docs/extensions)
