use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;
use std::time::Instant;

use crate::util;

/// Retrieve a single configuration value for `feature` from `c2rust-config`.
///
/// Runs `c2rust-config config --make --feature <feature> --list <key>` from the
/// `.c2rust` directory and returns the trimmed stdout.
fn get_config_value(key: &str, feature: &str) -> Result<String> {
    let project_root = util::find_project_root()?;
    let c2rust_dir = project_root.join(".c2rust");

    let output = Command::new("c2rust-config")
        .current_dir(&c2rust_dir)
        .args(["config", "--make", "--feature", feature, "--list", key])
        .output()
        .with_context(|| format!("Failed to get {} from config", key))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to retrieve {}: {}", key, stderr);
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if value.is_empty() {
        anyhow::bail!("Empty {} value from config", key);
    }

    Ok(value)
}

/// Execute a shell command string inside the directory identified by `dir_key`
/// in the project configuration.
///
/// `command_type` is used only for display purposes ("build", "clean", "test").
fn execute_command_in_dir(
    command_str: &str,
    dir_key: &str,
    feature: &str,
    command_type: &str,
) -> Result<()> {
    let dir_str = get_config_value(dir_key, feature)?;

    // Reject absolute or path-traversal values coming from config.
    if std::path::Path::new(&dir_str).is_absolute() {
        anyhow::bail!(
            "Directory path from config must be relative, got: {}",
            dir_str
        );
    }
    if dir_str.contains("..") {
        anyhow::bail!(
            "Directory path from config cannot contain '..', got: {}",
            dir_str
        );
    }

    let parts = shell_words::split(command_str)
        .with_context(|| format!("Failed to parse command: {}", command_str))?;

    if parts.is_empty() {
        return Ok(());
    }

    let project_root = util::find_project_root()?;
    let exec_dir = project_root.join(&dir_str);

    if !exec_dir.exists() {
        anyhow::bail!("Directory does not exist: {}", exec_dir.display());
    } else if !exec_dir.is_dir() {
        anyhow::bail!("Path is not a directory: {}", exec_dir.display());
    }

    let colored_label = match command_type {
        "build" => "│ → Executing build command:".bright_blue().to_string(),
        "test" => "│ → Executing test command:".bright_green().to_string(),
        "clean" => "│ → Executing clean command:".bright_red().to_string(),
        _ => format!("│ → Executing {} command:", command_type),
    };
    println!("{}", colored_label);
    println!(
        "│   {}",
        shell_words::join(&parts).bright_yellow()
    );
    println!(
        "│   {}: {}",
        "Working directory".dimmed(),
        exec_dir.display()
    );

    let mut command = Command::new(&parts[0]);
    command.current_dir(&exec_dir);
    if parts.len() > 1 {
        command.args(&parts[1..]);
    }

    let start_time = Instant::now();
    let output = command
        .output()
        .with_context(|| format!("Failed to execute command: {}", command_str))?;
    let duration = start_time.elapsed();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!(
            "│ {} (took {:.2}s)",
            format!("✗ {} failed", command_type.to_uppercase())
                .bright_red()
                .bold(),
            duration.as_secs_f64()
        );

        if !stderr.is_empty() {
            eprintln!("stderr: {}", stderr);
        }
        if !stdout.is_empty() {
            println!("stdout: {}", stdout);
        }

        let stderr_summary = stderr.lines().take(3).collect::<Vec<_>>().join("\n");
        if stderr_summary.is_empty() {
            anyhow::bail!("Command '{}' failed with non-zero exit status", command_str);
        } else {
            anyhow::bail!(
                "Command '{}' failed with non-zero exit status. Stderr (first lines):\n{}",
                command_str,
                stderr_summary
            );
        }
    }

    let success_msg = match command_type {
        "build" => format!(
            "│ {} (took {:.2}s)",
            "✓ Build successful".bright_green().bold(),
            duration.as_secs_f64()
        ),
        "test" => format!(
            "│ {} (took {:.2}s)",
            "✓ Test successful".bright_green().bold(),
            duration.as_secs_f64()
        ),
        "clean" => format!(
            "│ {} (took {:.2}s)",
            "✓ Clean successful".bright_green().bold(),
            duration.as_secs_f64()
        ),
        _ => format!(
            "│ ✓ {} successful (took {:.2}s)",
            command_type,
            duration.as_secs_f64()
        ),
    };
    println!("{}", success_msg);

    Ok(())
}

/// Run `cargo build` inside the feature's Rust sub-project at
/// `.c2rust/<feature>/rust`.
///
/// Setting `_show_full_output` to `true` is accepted for API parity with the
/// reference implementation but does not change behaviour; cargo errors are
/// always printed in full via the `bail!` message.
pub fn cargo_build(feature: &str, _show_full_output: bool) -> Result<()> {
    util::validate_feature_name(feature)?;

    let project_root = util::find_project_root()?;
    let build_dir = project_root.join(".c2rust").join(feature).join("rust");

    let start_time = Instant::now();

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(&build_dir)
        .env("RUSTFLAGS", "-A warnings")
        .output()
        .context("Failed to execute cargo build")?;

    let duration = start_time.elapsed();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Build error: {}", stderr);
    }

    println!(
        "  {} (took {:.2}s)",
        "Build completed".bright_green(),
        duration.as_secs_f64()
    );

    Ok(())
}

/// Clean the hybrid build environment for the given feature.
///
/// Reads `clean.cmd` and `clean.dir` from the project configuration and runs
/// the resulting command.
pub fn c2rust_clean(feature: &str) -> Result<()> {
    util::validate_feature_name(feature)?;

    let clean_cmd = get_config_value("clean.cmd", feature)?;
    execute_command_in_dir(&clean_cmd, "clean.dir", feature, "clean")
}

/// Run the hybrid build for the given feature.
///
/// Reads `build.cmd` and `build.dir` from the project configuration and runs
/// the resulting command.
pub fn c2rust_build(feature: &str) -> Result<()> {
    util::validate_feature_name(feature)?;

    let build_cmd = get_config_value("build.cmd", feature)?;
    execute_command_in_dir(&build_cmd, "build.dir", feature, "build")
}

/// Run the test suite for the given feature.
///
/// Reads `test.cmd` and `test.dir` from the project configuration and runs the
/// resulting command.
pub fn c2rust_test(feature: &str) -> Result<()> {
    util::validate_feature_name(feature)?;

    let test_cmd = get_config_value("test.cmd", feature)?;
    execute_command_in_dir(&test_cmd, "test.dir", feature, "test")
}

/// Execute the full four-step build-and-test verification flow:
///
/// 1. **Step 1/4** – `cargo build` (pure Rust compilation)
/// 2. **Step 2/4** – clean the hybrid build environment
/// 3. **Step 3/4** – hybrid build (C + Rust)
/// 4. **Step 4/4** – run the test suite
///
/// Any step that fails causes the function to return an error immediately.
pub fn run_full_build_and_test(feature: &str) -> Result<()> {
    println!("│");
    println!(
        "│ {}",
        "Running full build and test flow...".bright_blue().bold()
    );

    // Step 1: Build Rust code
    println!(
        "│ {}",
        "→ Step 1/4: Building Rust code (cargo build)...".bright_blue()
    );
    cargo_build(feature, true)?;
    println!("│ {}", "  ✓ Rust build successful".bright_green());

    // Step 2: Clean hybrid build environment
    println!("│ {}", "→ Step 2/4: Cleaning hybrid build...".bright_blue());
    c2rust_clean(feature)?;

    // Step 3: Hybrid build
    println!(
        "│ {}",
        "→ Step 3/4: Running hybrid build (C + Rust)...".bright_blue()
    );
    c2rust_build(feature)?;
    println!("│ {}", "  ✓ Hybrid build successful".bright_green());

    // Step 4: Run tests
    println!("│ {}", "→ Step 4/4: Running tests...".bright_blue());
    c2rust_test(feature)?;
    println!("│ {}", "  ✓ All tests passed".bright_green().bold());

    Ok(())
}
