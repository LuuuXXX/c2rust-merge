use anyhow::Result;
use std::process::Command;

use crate::util;

/// Merge code analysis for a feature, combining independent .rs files into larger files
/// corresponding one-to-one with C files.
pub fn merge_code_analysis(feature: &str) -> Result<()> {
    if !feature
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!(
            "Invalid feature name '{}': only alphanumeric characters, hyphens, and underscores are allowed",
            feature
        );
    }

    println!("Running code_analyse --merge --feature {}", feature);

    let project_root = util::find_project_root()?;

    let output = Command::new("code_analyse")
        .current_dir(&project_root)
        .args(["--merge", "--feature", feature])
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow::anyhow!(
                    "code_analyse not found: ensure it is installed and available in PATH"
                )
            } else {
                anyhow::anyhow!("Failed to execute code_analyse --merge: {}", e)
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!(
            "code_analyse merge failed.\nstdout:\n{}\nstderr:\n{}",
            stdout,
            stderr
        );
    }

    if !stdout.trim().is_empty() {
        println!("code_analyse merge output:\n{}", stdout);
    }

    Ok(())
}
