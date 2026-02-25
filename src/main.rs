use clap::Parser;

mod merge;

/// c2rust-merge – merge C-to-Rust translated output into a Rust project.
///
/// Run this tool after `c2rust translate` has produced translated Rust files.
/// It copies every `.rs` file from the translation output directory into the
/// target destination directory, preserving the relative directory structure.
#[derive(Debug, Parser)]
#[command(name = "c2rust-merge", version, about)]
struct Cli {
    /// Directory containing the translated Rust files (output of `c2rust translate`).
    #[arg(short, long, value_name = "DIR")]
    translated: String,

    /// Destination directory where the translated files will be merged into.
    #[arg(short, long, value_name = "DIR")]
    dest: String,
}

fn main() {
    let cli = Cli::parse();

    let src = merge::resolve_path(&cli.translated);
    let dest = merge::resolve_path(&cli.dest);

    println!(
        "Merging translated Rust files\n  from: {}\n  into: {}",
        src.display(),
        dest.display()
    );

    match merge::merge(&src, &dest) {
        Ok(count) => println!("Done. {} file(s) merged.", count),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
