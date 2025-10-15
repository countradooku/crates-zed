# Development Guide

This guide will help you develop and test the Crates Autocomplete extension for Zed.

## Prerequisites

- Rust installed via `rustup` (not homebrew or other package managers)
- Zed editor installed
- Git

## Project Structure

```
crates_autocomplete/
├── src/
│   └── lib.rs              # Main extension logic
├── Cargo.toml              # Rust dependencies and build config
├── extension.toml          # Extension metadata and language server config
├── README.md               # User-facing documentation
├── LICENSE                 # MIT License
├── .gitignore             # Git ignore rules
└── DEVELOPMENT.md          # This file
```

## Building the Extension

### Check for Compilation Errors

```bash
cargo check
```

### Build for Development

```bash
cargo build
```

### Build for Release

```bash
cargo build --release
```

### Build as WebAssembly (for Zed)

The extension needs to be compiled to WebAssembly:

```bash
cargo build --target wasm32-wasip1 --release
```

## Testing the Extension

### Install as Development Extension

1. Open Zed
2. Press `Cmd/Ctrl + Shift + P` to open the command palette
3. Type "extensions" and select "zed: extensions"
4. Click "Install Dev Extension" button
5. Navigate to this project directory and select it

### Testing in a Rust Project

1. Create a test Rust project or use an existing one:
   ```bash
   cargo new test-project
   cd test-project
   ```

2. Open the project in Zed:
   ```bash
   zed .
   ```

3. Open `Cargo.toml` and start editing the `[dependencies]` section
4. You should see autocomplete suggestions as you type

### Viewing Logs

If something isn't working:

1. Open the command palette (`Cmd/Ctrl + Shift + P`)
2. Type "zed: open log" and press Enter
3. Look for errors related to `crates-lsp` or the extension

You can also run Zed from the terminal with verbose logging:

```bash
zed --foreground
```

## Code Overview

### Main Components

#### `CratesAutocompleteExtension` struct

Holds the state of the extension, including the cached path to the `crates-lsp` binary.

#### `language_server_binary_path()` method

This method:
1. Checks if crates-lsp is already cached
2. Looks for crates-lsp in the system PATH
3. Downloads the appropriate binary from GitHub releases if not found
4. Handles platform-specific binary names and permissions

#### `language_server_command()` method

Returns the command to start the language server. This is called by Zed when opening a TOML file.

#### `language_server_workspace_configuration()` method

Provides configuration settings to the language server from Zed's settings.json.

## Extension Configuration

The extension is configured via `extension.toml`:

```toml
[language_servers.crates-lsp]
name = "crates-lsp"
language = "TOML"
```

This tells Zed that:
- The extension provides a language server named "crates-lsp"
- It should activate for TOML files

## Debugging Tips

### Extension not loading

1. Check that Rust is installed via rustup
2. Verify the extension.toml is valid TOML
3. Check the Zed log for errors

### Language server not starting

1. Verify the binary path is correct
2. Check file permissions (Unix systems need executable bit)
3. Try running crates-lsp manually: `crates-lsp --version`

### Autocomplete not appearing

1. Ensure you're in a `Cargo.toml` file
2. Check the `[dependencies]` section syntax is valid
3. Verify crates-lsp is running in the Zed log
4. Try manually triggering autocomplete with `Ctrl+Space`

## Making Changes

### Modifying the Extension Logic

1. Edit `src/lib.rs`
2. Run `cargo check` to verify compilation
3. Rebuild the extension
4. Reload Zed or restart to pick up changes

### Updating Dependencies

1. Edit `Cargo.toml`
2. Run `cargo update`
3. Test thoroughly

### Changing Extension Metadata

1. Edit `extension.toml`
2. Increment the version number
3. Reload the extension in Zed

## Publishing the Extension

To publish the extension to the Zed extension registry:

1. Ensure your extension repository is public on GitHub
2. Add a LICENSE file (MIT or Apache 2.0 required)
3. Create a pull request to [zed-industries/extensions](https://github.com/zed-industries/extensions)
4. Add your extension as a Git submodule:
   ```bash
   git submodule add https://github.com/yourusername/crates-autocomplete extensions/crates-autocomplete
   ```
5. Update `extensions.toml` with your extension details

See the [Zed Extension Documentation](https://zed.dev/docs/extensions/developing-extensions) for detailed publishing instructions.

## Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [zed_extension_api Docs](https://docs.rs/zed_extension_api)
- [crates-lsp Repository](https://github.com/MathiasPius/crates-lsp)
- [Zed GitHub Repository](https://github.com/zed-industries/zed)

## Common Issues

### Rust not found

Make sure Rust is installed via rustup, not homebrew or other package managers:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Wrong target

If you get an error about the wasm32-wasip1 target:

```bash
rustup target add wasm32-wasip1
```

### Permission denied (Unix)

The extension sets executable permissions automatically, but if you encounter issues:

```bash
chmod +x /path/to/crates-lsp
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - See LICENSE file for details
