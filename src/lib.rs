use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json, settings::LspSettings};

struct CratesAutocompleteExtension {
    cached_binary_path: Option<String>,
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
        let user_settings = LspSettings::for_worktree("crates-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        // Default configuration matching VS Code crates extension UX
        let default_config = serde_json::json!({
            "inlay_hints": true,
            "diagnostics": true,
            "up_to_date_hint": "✅",
            "needs_update_hint": "❌ {}",
            "needs_update_severity": 2, // WARNING level (like VS Code)
            "unknown_dep_severity": 1,  // ERROR level (like VS Code)
            "up_to_date_severity": 4,   // HINT level (subtle)
        });

        // Merge user settings with defaults (user settings take precedence)
        let merged_config = merge_json_values(default_config, user_settings);

        Ok(Some(merged_config))
    }
}

// Helper function to merge two JSON values, with user_value taking precedence
fn merge_json_values(default: serde_json::Value, user: serde_json::Value) -> serde_json::Value {
    if let (Some(default_obj), Some(user_obj)) = (default.as_object(), user.as_object()) {
        let mut merged = default_obj.clone();
        for (key, value) in user_obj {
            merged.insert(key.clone(), value.clone());
        }
        serde_json::Value::Object(merged)
    } else if user.is_null() {
        default
    } else {
        user
    }
}

impl CratesAutocompleteExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok()
        {
            return Ok(path.clone());
        }

        if let Some(path) = worktree.which("crates-lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

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
        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "crates-lsp-aarch64-apple-darwin.tar.gz",
            (zed::Os::Mac, zed::Architecture::X8664) => "crates-lsp-x86_64-apple-darwin.tar.gz",
            (zed::Os::Linux, zed::Architecture::Aarch64) => {
                "crates-lsp-aarch64-unknown-linux-gnu.tar.gz"
            }
            (zed::Os::Linux, zed::Architecture::X8664) => {
                "crates-lsp-x86_64-unknown-linux-gnu.tar.gz"
            }
            (zed::Os::Windows, zed::Architecture::X8664) => "crates-lsp-x86_64-pc-windows-msvc.zip",
            _ => return Err("Unsupported platform".to_string()),
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| "Language server binary not found".to_string())?;

        let version_dir = format!("crates-lsp-{}", release.version);
        let binary_path = format!(
            "{}/{}",
            version_dir,
            if platform == zed::Os::Windows {
                "crates-lsp.exe"
            } else {
                "crates-lsp"
            }
        );

        if fs::metadata(&binary_path).is_err() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                if platform == zed::Os::Windows {
                    zed::DownloadedFileType::Zip
                } else {
                    zed::DownloadedFileType::GzipTar
                },
            )
            .map_err(|e| format!("Failed to download: {}", e))?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&binary_path, std::fs::Permissions::from_mode(0o755))
                    .map_err(|e| format!("Failed to set permissions: {}", e))?;
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(CratesAutocompleteExtension);
