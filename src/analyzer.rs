use anyhow::Result;
use std::process::Command;

use crate::util;

/// Merge code analysis results for a single feature.
///
/// This function runs the external `code_analyse` tool with
/// `--merge --feature <feature>` from the project root, combining
/// independent Rust analysis outputs into larger files that correspond
/// one-to-one with the original C source files.
///
/// # Parameters
///
/// * `feature` - The name of the feature to merge. It must contain only
///   ASCII alphanumeric characters, hyphens (`-`), and underscores (`_`).
///   Any other character will cause this function to return an error.
///
/// # Errors
///
/// Returns an error if:
/// * the `feature` name is invalid according to the constraints above;
/// * the project root cannot be determined by [`util::find_project_root`];
/// * the `code_analyse` executable cannot be launched (for example, it is
///   not installed or not found in `PATH`);
/// * `code_analyse` exits with a non-zero status when running
///   `code_analyse --merge --feature <feature>`.
///
/// # External dependency
///
/// This function requires the `code_analyse` binary to be installed and
/// available on the system `PATH`. If it is missing, an error is returned.
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
            "code_analyse merge failed.\n\nstdout:\n{}\n\nstderr:\n{}",
            stdout,
            stderr
        );
    }

    if !stdout.trim().is_empty() {
        println!("code_analyse merge output:\n{}", stdout);
    }

    Ok(())
}
