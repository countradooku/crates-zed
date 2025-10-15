# GitHub Actions Workflows

This directory contains the CI/CD workflows for the Crates Autocomplete extension.

## Workflows

### CI Workflow (`ci.yml`)

**Triggers:** Push to main/master/develop, Pull Requests, Manual dispatch

**Jobs:**
- **Check**: Validates code compiles (`cargo check`)
- **Test**: Runs test suite (`cargo test`)
- **Format Check**: Ensures code is formatted (`cargo fmt`)
- **Clippy**: Runs linting (`cargo clippy`)
- **Build WebAssembly**: Compiles extension to wasm32-wasip1
- **Validate Extension**: Checks extension.toml and LICENSE
- **Security Audit**: Runs `cargo audit` for vulnerabilities
- **All Checks**: Final validation that all jobs passed

**Artifacts:** WebAssembly build artifacts

### Release Workflow (`release.yml`)

**Triggers:** Tag push (v*.*.*), Manual dispatch

**Jobs:**
- **Build**: Creates release build and archive
- **Create Release**: Publishes GitHub release with artifacts

**Artifacts:** Release archive (.tar.gz)

**Usage:**
```bash
# Create a new release
git tag v0.1.2
git push --tags

# Or manually trigger from GitHub Actions UI
```

## Badges

Add these badges to your README.md:

```markdown
[![CI](https://github.com/yourusername/crates-autocomplete/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/crates-autocomplete/actions/workflows/ci.yml)
[![Release](https://github.com/yourusername/crates-autocomplete/actions/workflows/release.yml/badge.svg)](https://github.com/yourusername/crates-autocomplete/actions/workflows/release.yml)
```

## Secrets Required

No secrets are required for CI. For releases, the workflow uses:
- `GITHUB_TOKEN` (automatically provided)

## Caching

The workflows use GitHub Actions cache for:
- Cargo registry
- Cargo git index
- Build artifacts

This significantly speeds up builds.

## Running Locally

You can run the same checks locally:

```bash
# All CI checks
cargo check --all-features
cargo test --all-features
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo build --target wasm32-wasip1 --release
cargo audit

# Quick check
cargo check && cargo test && cargo fmt --check && cargo clippy
```

## Customization

### Adding New Checks

Edit `ci.yml` and add a new job:

```yaml
new-check:
  name: New Check
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - run: your-command
```

Don't forget to add it to the `needs` array in the `all-checks` job.

### Changing Trigger Branches

Edit the `on` section:

```yaml
on:
  push:
    branches: [main, develop, feature/*]
```

## Troubleshooting

### CI Failing?

1. Run checks locally first
2. Check the workflow logs on GitHub
3. Ensure all dependencies are up to date
4. Verify the wasm32-wasip1 target is installed

### Release Not Creating?

1. Verify the tag format is `v*.*.*`
2. Check the workflow has write permissions
3. Ensure CHANGELOG.md is updated
4. Check the logs for specific errors

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Guide](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
