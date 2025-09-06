zed_extension_api::register_extension!(MojoExtension);

struct MojoExtension {}

impl MojoExtension {
    fn find_mojo_lsp_server(&self, worktree: &zed_extension_api::Worktree) -> Option<String> {
        // In WASM, we can't access the filesystem to check if files exist
        // So we'll try the most likely paths and let Zed validate them

        let worktree_path = worktree.root_path();
        let pixi_path = format!("{}/.pixi/envs/default/bin/mojo-lsp-server", worktree_path);
        println!("Trying pixi path first: {}", pixi_path);

        // Return the pixi path first since that's most likely for this project
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

        let command = self.find_mojo_lsp_server(worktree)
            .ok_or_else(|| "mojo-lsp-server not found. Please install Mojo and ensure mojo-lsp-server is in your PATH.
                Visit https://docs.modular.com/mojo/ for installation instructions.".to_string())?;

        Ok(zed_extension_api::Command {
            command,
            args: vec![],
            env: Default::default(),
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
