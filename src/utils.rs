use std::fs;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

pub const ENV_TEMPLATE: &str = include_str!("../env_template");
pub const COMPOSE_TEMPLATE: &str = include_str!("../docker-compose.yaml");

pub fn find_file(filename: &str) -> bool {
    // First check current working directory (where user runs the installer)
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join(filename).exists() {
        return true;
    }

    // Fallback: check project_root() for backward compatibility
    let root = project_root();
    root.join(filename).exists()
}

pub fn project_root() -> PathBuf {
    // In production/airgapped mode, project root is CWD
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Only do complex lookup if running from development (target/ directory)
    if cwd.to_str().map(|s| s.contains("target")).unwrap_or(false) {
        // Walk up to find Cargo.toml for development
        let mut current = cwd.as_path();
        while let Some(parent) = current.parent() {
            if parent.join("Cargo.toml").exists() {
                return parent.to_path_buf();
            }
            current = parent;
        }
    }

    cwd
}

pub fn ensure_compose_bundle(root: &Path) -> Result<()> {
    // Compose file: only scaffold if none of the common names already exist
    let compose_candidates = [
        "docker-compose.yaml",
        "docker-compose.yml",
        "compose.yaml",
        "compose.yml",
    ];

    if !compose_candidates
        .iter()
        .any(|name| root.join(name).exists())
    {
        let compose_path = root.join("docker-compose.yaml");
        if let Some(parent) = compose_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&compose_path, COMPOSE_TEMPLATE)?;
    }

    // Fluentd config now comes from GHCR image - no local files needed
    // See: ghcr.io/nexusquantum/fluentd-hypervisor-collector:latest

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_file_in_cwd() {
        // Test that find_file checks CWD first
        // This test assumes it runs from project root where Cargo.toml exists
        assert!(
            find_file("Cargo.toml"),
            "Should find Cargo.toml in CWD or project root"
        );
    }

    #[test]
    fn test_find_file_not_exists_in_cwd() {
        // This file should not exist in CWD or project root
        assert!(
            !find_file("definitely_does_not_exist_12345.txt"),
            "Should not find non-existent file"
        );
    }
}
