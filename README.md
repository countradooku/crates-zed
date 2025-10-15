# Crates Autocomplete for Zed

A Zed extension that provides intelligent autocomplete, versioning, and features for Rust crates in `Cargo.toml` files. This extension wraps the excellent [crates-lsp](https://github.com/MathiasPius/crates-lsp) language server.

## Features

- **Crate Name Autocomplete**: Get intelligent suggestions while typing crate names
- **Version Autocomplete**: Automatically suggest the latest available versions
- **Outdated Version Indicators**: Visual hints showing when dependencies need updating
- **Feature Suggestions**: Autocomplete for crate features
- **Inlay Hints**: Display package status inline
- **Diagnostics**: Configurable warnings for outdated dependencies

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

You can configure the extension in your Zed settings file (`settings.json`):

```json
{
  "lsp": {
    "crates-lsp": {
      "settings": {
        "inlay_hints": true,
        "diagnostics": true,
        "needs_update_severity": 3,
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
| `needs_update_severity` | integer | `3` | Severity level for outdated package warnings (1-4) |
| `cache_directory` | string | OS-specific | Custom directory for caching crates.io data |

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
