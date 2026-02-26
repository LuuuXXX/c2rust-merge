use anyhow::Result;
use colored::Colorize;

pub mod analyzer;
pub mod builder;
pub mod util;

/// Merges translated files associated with the given feature and then runs
/// a full build-and-test verification.
///
/// # Arguments
///
/// * `feature` - The name or identifier of the feature whose translated files
///   should be merged. This is passed through to the underlying analyzer and
///   is used to determine which feature's code analysis results to process.
///
/// This function prints progress information to stdout and delegates the
/// actual merge work to [`analyzer::merge_code_analysis`] followed by the
/// four-step verification provided by [`builder::run_full_build_and_test`].
/// It returns `Ok(())` if both steps complete successfully, or an error if
/// either step fails.
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
    println!("│ {}", "✓ Merge step completed".bright_green());

    println!("│");
    println!("│ {}", "Running verification...".bright_blue().bold());
    builder::run_full_build_and_test(feature)?;
    println!("│ {}", "✓ Verification completed".bright_green());

    Ok(())
}
