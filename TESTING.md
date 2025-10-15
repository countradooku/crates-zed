# Testing the Extension Locally

This guide walks you through testing the Crates Autocomplete extension in Zed.

## Prerequisites

- Zed editor installed
- Rust installed via rustup
- This extension repository cloned/downloaded

## Step 1: Build the Extension

The extension has already been built! The WebAssembly file is located at:
```
target/wasm32-wasip1/release/crates_autocomplete.wasm
```

If you need to rebuild:

```bash
# Add WebAssembly target (only needed once)
rustup target add wasm32-wasip1

# Build the extension
cargo build --target wasm32-wasip1 --release
```

## Step 2: Install as Dev Extension in Zed

### Option A: Using Zed UI (Recommended)

1. **Open Zed**

2. **Open Extensions Panel**:
   - Press `Cmd/Ctrl + Shift + P` (Command Palette)
   - Type "extensions"
   - Select **"zed: extensions"**

3. **Install Dev Extension**:
   - Click the **"Install Dev Extension"** button at the top
   - Navigate to this directory: `C:\Users\radu0\Desktop\Code\Rust\crates_autocomplete`
   - Select/Open the folder

4. **Verify Installation**:
   - You should see "Crates Autocomplete" in your extensions list
   - It should show as "Dev" or "Development"

### Option B: Manual Installation

Copy the extension to Zed's extensions directory:

**Windows**:
```bash
mkdir -p "%APPDATA%\Zed\extensions\installed\crates-autocomplete"
xcopy /E /I . "%APPDATA%\Zed\extensions\installed\crates-autocomplete"
```

**macOS**:
```bash
mkdir -p ~/Library/Application\ Support/Zed/extensions/installed/crates-autocomplete
cp -r . ~/Library/Application\ Support/Zed/extensions/installed/crates-autocomplete/
```

**Linux**:
```bash
mkdir -p ~/.config/zed/extensions/installed/crates-autocomplete
cp -r . ~/.config/zed/extensions/installed/crates-autocomplete/
```

## Step 3: Create a Test Project

Create a simple Rust project to test with:

```bash
# Create a new Rust project
cargo new test-crates-extension
cd test-crates-extension

# Open in Zed
zed .
```

## Step 4: Test the Extension

### Test 1: Crate Name Autocomplete

1. Open `Cargo.toml` in Zed
2. Go to the `[dependencies]` section
3. Start typing a crate name:

```toml
[dependencies]
ser
```

**Expected**: Autocomplete suggestions showing "serde", "serde_json", etc.

### Test 2: Version Autocomplete

1. Type a crate name and start the version:

```toml
[dependencies]
serde = "1.
```

**Expected**: Autocomplete showing available versions like "1.0.228", "1.0.227", etc.

### Test 3: Features Autocomplete

1. Add features to a crate:

```toml
[dependencies]
serde = { version = "1.0", features = ["
```

**Expected**: Suggestions for available features like "derive", "alloc", etc.

### Test 4: Outdated Version Warnings

1. Add an old version:

```toml
[dependencies]
serde = "1.0.100"
```

**Expected**: A diagnostic/warning indicating the crate is outdated

### Test 5: Inlay Hints

1. With valid dependencies, you should see inline hints showing:
   - Latest available versions
   - Update recommendations

## Step 5: Check Logs

If something isn't working, check the Zed logs:

1. **Open Command Palette**: `Cmd/Ctrl + Shift + P`
2. Type and select: **"zed: open log"**
3. Look for messages related to:
   - `crates-lsp`
   - `crates-autocomplete`
   - Language server errors

Alternatively, run Zed from terminal with verbose logging:

```bash
zed --foreground
```

## Troubleshooting

### Extension Not Loading

**Symptom**: Extension doesn't appear in the extensions list

**Solutions**:
1. Check that `extension.toml` is valid TOML
2. Verify the wasm file was built: `ls target/wasm32-wasip1/release/*.wasm`
3. Restart Zed completely
4. Check Zed logs for errors

### No Autocomplete Appearing

**Symptom**: No suggestions when typing in Cargo.toml

**Solutions**:
1. Verify you're in a `Cargo.toml` file
2. Check the file is in a valid Rust project (has Cargo.toml at root)
3. Try manually triggering autocomplete: `Ctrl+Space`
4. Check if `crates-lsp` is starting (check logs)
5. Ensure proper TOML syntax

### Language Server Not Starting

**Symptom**: Logs show errors about crates-lsp

**Solutions**:
1. The extension will try to download crates-lsp automatically
2. You can manually install it: `cargo install --locked crates-lsp`
3. Check your internet connection (for downloading)
4. On Unix systems, check file permissions: `chmod +x crates-lsp`

### Download Failures

**Symptom**: Extension can't download crates-lsp

**Solutions**:
1. Install manually: `cargo install --locked crates-lsp`
2. Check GitHub is accessible
3. Verify your platform is supported (Windows x64, macOS Intel/ARM, Linux x64/ARM64)

## Testing Different Scenarios

### Test with Multiple Dependencies

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
```

### Test with Workspace Cargo.toml

```toml
[workspace]
members = ["crate-a", "crate-b"]

[workspace.dependencies]
serde = "1.0"
```

### Test with Build Dependencies

```toml
[build-dependencies]
cc = "1.0"
```

### Test with Dev Dependencies

```toml
[dev-dependencies]
criterion = "0.5"
```

## Advanced Testing

### Custom Configuration

Create or edit `.config/zed/settings.json`:

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

Restart Zed and test with different configuration values.

### Performance Testing

Test with a large Cargo.toml:

```bash
# Clone a large Rust project
git clone https://github.com/tokio-rs/tokio
cd tokio
zed Cargo.toml
```

Check that:
- Autocomplete is still responsive
- No significant lag
- Language server doesn't crash

## Success Criteria

The extension is working correctly if:

✅ Crate names appear in autocomplete
✅ Version numbers are suggested
✅ Features are autocompleted
✅ Outdated dependencies show warnings
✅ No errors in Zed logs
✅ Language server starts automatically
✅ Works across different dependency sections

## Reporting Issues

If you find bugs during testing:

1. Check the logs (`zed: open log`)
2. Note the exact steps to reproduce
3. Check your Zed version
4. Open an issue with:
   - Description of the problem
   - Steps to reproduce
   - Log output
   - System information

## Next Steps

Once testing is complete:

1. Make any necessary fixes
2. Update documentation
3. Consider publishing to Zed extensions registry
4. Share with the community!

## Quick Test Checklist

- [ ] Extension builds successfully
- [ ] Extension installs in Zed
- [ ] Appears in extensions list
- [ ] Activates on Cargo.toml files
- [ ] Crate name autocomplete works
- [ ] Version autocomplete works
- [ ] Features autocomplete works
- [ ] Outdated warnings appear
- [ ] No errors in logs
- [ ] Works on multiple files
- [ ] Configuration is respected

Happy testing! 🚀
