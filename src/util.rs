use anyhow::{Context, Result};
use std::path::PathBuf;

/// Find the project root by traversing up the directory tree looking for Cargo.toml
pub fn find_project_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir().context("Failed to get current directory")?;

    loop {
        if current.join("Cargo.toml").exists() {
            return Ok(current);
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => anyhow::bail!("Could not find project root (no Cargo.toml found in directory tree)"),
        }
    }
}
