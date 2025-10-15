use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

struct CratesAutocompleteExtension {
    cached_binary_path: Option<String>,
}

impl CratesAutocompleteExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Check if crates-lsp is already installed in PATH
        if let Some(path) = worktree.which("crates-lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        // Try to download and install crates-lsp
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "MathiasPius/crates-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        // Determine the correct asset name based on platform and architecture
        let (asset_name, archive_type) = match platform {
            zed::Os::Mac => match arch {
                zed::Architecture::Aarch64 => (
                    "crates-lsp-aarch64-apple-darwin.tar.gz",
                    zed::DownloadedFileType::GzipTar,
                ),
                zed::Architecture::X8664 => (
                    "crates-lsp-x86_64-apple-darwin.tar.gz",
                    zed::DownloadedFileType::GzipTar,
                ),
                _ => return Err("Unsupported architecture for macOS".to_string()),
            },
            zed::Os::Linux => match arch {
                zed::Architecture::Aarch64 => (
                    "crates-lsp-aarch64-unknown-linux-gnu.tar.gz",
                    zed::DownloadedFileType::GzipTar,
                ),
                zed::Architecture::X8664 => (
                    "crates-lsp-x86_64-unknown-linux-gnu.tar.gz",
                    zed::DownloadedFileType::GzipTar,
                ),
                _ => return Err("Unsupported architecture for Linux".to_string()),
            },
            zed::Os::Windows => match arch {
                zed::Architecture::X8664 => (
                    "crates-lsp-x86_64-pc-windows-msvc.zip",
                    zed::DownloadedFileType::Zip,
                ),
                _ => return Err("Unsupported architecture for Windows".to_string()),
            },
        };

        // Find the matching asset
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("No asset found matching {}", asset_name))?;

        let version_dir = format!("crates-lsp-{}", release.version);
        let binary_path = format!(
            "{version_dir}/{}",
            if platform == zed::Os::Windows {
                "crates-lsp.exe"
            } else {
                "crates-lsp"
            }
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, archive_type)
                .map_err(|e| format!("Failed to download crates-lsp: {e}"))?;

            // Make the binary executable on Unix systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&binary_path, std::fs::Permissions::from_mode(0o755))
                    .map_err(|e| format!("Failed to set executable permission: {e}"))?;
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for CratesAutocompleteExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("crates-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(settings))
    }
}

zed::register_extension!(CratesAutocompleteExtension);
