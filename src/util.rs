use anyhow::{Context, Result};
use std::path::PathBuf;

/// Find the project root by traversing up the directory tree looking for `Cargo.toml`.
///
/// # Returns
///
/// * `Ok(PathBuf)` containing the path to the directory where `Cargo.toml` is found.
/// * `Err` if the current working directory cannot be determined, or if no `Cargo.toml`
///   is found in the current directory or any of its ancestors.
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
