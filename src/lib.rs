use anyhow::Result;
use colored::Colorize;

pub mod analyzer;
pub mod util;

/// Merges translated files associated with the given feature.
///
/// # Arguments
///
/// * `feature` - The name or identifier of the feature whose translated files
///   should be merged. This is passed through to the underlying analyzer and
///   is used to determine which feature's code analysis results to process.
///
/// This function prints progress information to stdout and delegates the
/// actual merge work to [`analyzer::merge_code_analysis`]. It returns `Ok(())`
/// if the merge completes successfully, or an error if the underlying
/// analysis or merge step fails.
pub fn merge(feature: &str) -> Result<()> {
    println!(
        "\n{}",
        format!("Merge Files for feature: {}", feature)
            .bright_cyan()
            .bold()
    );

    println!("│");
    println!("│ {}", "Merging translated files...".bright_blue().bold());
    analyzer::merge_code_analysis(feature)?;
    println!("│ {}", "✓ Merge step completed (see details above)".bright_green());

    Ok(())
}
