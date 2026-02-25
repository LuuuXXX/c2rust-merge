use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "c2rust-merge", about = "Merge translated Rust files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Merge translated files for a feature
    Merge {
        /// The feature name to merge
        #[arg(long, default_value = "default")]
        feature: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Merge { feature } => {
            c2rust_merge::merge(&feature)?;
        }
    }

    Ok(())
}
