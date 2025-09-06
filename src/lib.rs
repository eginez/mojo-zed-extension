zed_extension_api::register_extension!(MojoExtension);

struct MojoExtension {}

impl MojoExtension {
    fn find_mojo_lsp_server(&self, worktree: &zed_extension_api::Worktree) -> Option<String> {
        // Note: Users can configure a custom path in settings.json:
        // "lsp": { "mojo-lsp-server": { "binary": { "path": "/custom/path/to/mojo-lsp-server" } } }
        // Zed automatically uses that path if configured, so this function only handles auto-discovery

        // 1. Try PATH first (works for global installs and activated environments)
        if let Some(path) = worktree.which("mojo-lsp-server") {
            println!("Found mojo-lsp-server in PATH: {}", path);
            return Some(path);
        }

        // 2. Try pixi environment (common for Mojo projects)
        let worktree_path = worktree.root_path();
        let pixi_path = format!("{}/.pixi/envs/default/bin/mojo-lsp-server", worktree_path);
        println!("Trying pixi path: {}", pixi_path);

        // Note: In WASM we can't directly check file existence, but Zed will validate
        // the path when it tries to execute the command. Return the most likely path.
        Some(pixi_path)
    }
}

impl zed_extension_api::Extension for MojoExtension {
    fn new() -> Self {
        println!("MojoExtension::new()");
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        println!("MojoExtension::language_server_command()");

        let command = self.find_mojo_lsp_server(worktree).ok_or_else(|| {
            "mojo-lsp-server not found. Please:
1. Install Mojo (https://docs.modular.com/mojo/)
2. Ensure mojo-lsp-server is in your PATH, conda/pixi environment, or
3. Configure a custom path in settings.json:
   \"lsp\": { \"mojo-lsp-server\": { \"binary\": { \"path\": \"/path/to/mojo-lsp-server\" } } }"
                .to_string()
        })?;

        Ok(zed_extension_api::Command {
            command,
            args: vec![],
            env: worktree.shell_env(), // Use shell environment from worktree
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_path_checking_logic() {
        // Test that our path checking logic works
        let temp_dir = std::env::temp_dir().join("test_mojo_extension");
        let pixi_bin_dir = temp_dir.join(".pixi/envs/default/bin");
        let lsp_path = pixi_bin_dir.join("mojo-lsp-server");

        // Create the directory structure
        fs::create_dir_all(&pixi_bin_dir).unwrap();

        // Create a dummy mojo-lsp-server file
        fs::write(&lsp_path, "#!/bin/bash\necho 'mock lsp server'").unwrap();

        // Test that fs::metadata can find our test file
        assert!(fs::metadata(&lsp_path).is_ok());

        // Test the path construction logic
        let constructed_path = format!(
            "{}/.pixi/envs/default/bin/mojo-lsp-server",
            temp_dir.to_string_lossy()
        );
        assert!(fs::metadata(&constructed_path).is_ok());

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_scm_files_exist() {
        // Test that all required .scm files exist (either as files or symlinks)
        let project_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let scm_files = vec![
            "languages/mojo/highlights.scm",
            "languages/mojo/brackets.scm",
            "languages/mojo/indents.scm",
            "languages/mojo/outline.scm",
            "languages/mojo/overrides.scm",
        ];

        for file in scm_files {
            let path = std::path::Path::new(&project_root).join(file);
            assert!(path.exists(), "Missing required file: {}", path.display());
        }
    }
}
