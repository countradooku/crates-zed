# Quick Start Guide

Get up and running with the Crates Autocomplete extension in under 5 minutes!

## For Users

### Installation

1. Open Zed editor
2. Open the command palette: `Cmd/Ctrl + Shift + P`
3. Type "extensions" and select **"zed: extensions"**
4. Search for **"Crates Autocomplete"**
5. Click **Install**

### Usage

1. Open any Rust project in Zed
2. Open the `Cargo.toml` file
3. Start typing in the `[dependencies]` section:

```toml
[dependencies]
ser  # <- Autocomplete will suggest "serde" and other crates
```

4. When you type a version:

```toml
[dependencies]
serde = "1.  # <- Autocomplete will suggest the latest versions
```

5. For features:

```toml
[dependencies]
serde = { version = "1.0", features = ["  # <- Autocomplete available here
```

### That's it!

The extension will automatically:
- Suggest crate names from crates.io
- Provide version autocomplete
- Show warnings for outdated dependencies
- Display inline hints about package status

## For Developers

### Setting Up Development Environment

1. Clone the repository:
```bash
git clone https://github.com/yourusername/crates-autocomplete
cd crates-autocomplete
```

2. Install Rust (via rustup):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Add the WebAssembly target:
```bash
rustup target add wasm32-wasip1
```

4. Build the extension:
```bash
cargo check
```

### Testing

1. Open Zed
2. Command palette → "zed: extensions"
3. Click "Install Dev Extension"
4. Select the `crates-autocomplete` directory
5. Open a Rust project and test!

### Making Changes

1. Edit `src/lib.rs`
2. Run `cargo check` to verify
3. Reload Zed to test changes

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development instructions.

## Configuration (Optional)

Add to your Zed `settings.json`:

```json
{
  "lsp": {
    "crates-lsp": {
      "settings": {
        "inlay_hints": true,
        "diagnostics": true,
        "needs_update_severity": 3
      }
    }
  }
}
```

## Troubleshooting

### Extension not working?

1. Check you're editing a `Cargo.toml` file
2. Open Zed log: Command palette → "zed: open log"
3. Look for errors related to `crates-lsp`
4. Try restarting Zed

### Still having issues?

- Check the [README.md](README.md) troubleshooting section
- Review [DEVELOPMENT.md](DEVELOPMENT.md) debugging tips
- Open an issue on GitHub

## Next Steps

- Read the [README.md](README.md) for full feature list
- Check out [DEVELOPMENT.md](DEVELOPMENT.md) for contributing
- Star the repository if you find it useful!

## Credits

Powered by [crates-lsp](https://github.com/MathiasPius/crates-lsp) ❤️
