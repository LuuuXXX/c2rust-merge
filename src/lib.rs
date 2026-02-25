use anyhow::Result;
use colored::Colorize;

pub mod analyzer;
pub mod util;

/// Merge translated files for a given feature
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
