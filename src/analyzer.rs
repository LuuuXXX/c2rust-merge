use anyhow::{Context, Result};
use std::process::Command;

use crate::util;

/// Merge code analysis for a feature, combining independent .rs files into larger files
/// corresponding one-to-one with C files.
pub fn merge_code_analysis(feature: &str) -> Result<()> {
    println!("Running code_analyse --merge --feature {}", feature);

    let project_root = util::find_project_root()?;

    let output = Command::new("code_analyse")
        .current_dir(&project_root)
        .args(["--merge", "--feature", feature])
        .output()
        .context("Failed to execute code_analyse --merge")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("code_analyse merge failed: {}", stderr);
    }

    Ok(())
}
