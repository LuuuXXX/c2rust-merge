use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Merge translated Rust source files from `src_dir` into the `dest_dir`.
///
/// This function is designed to run after `c2rust translate` has produced
/// translated Rust files. It copies all `.rs` files from the translation
/// output directory into the destination directory, preserving the relative
/// directory structure.
///
/// If a file already exists at the destination it is overwritten, so the
/// translated code always takes precedence.
pub fn merge(src_dir: &Path, dest_dir: &Path) -> io::Result<u32> {
    if !src_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("source directory does not exist: {}", src_dir.display()),
        ));
    }

    fs::create_dir_all(dest_dir)?;

    let mut count = 0u32;
    merge_dir(src_dir, src_dir, dest_dir, &mut count)?;
    Ok(count)
}

/// Recursively walk `current_dir` (rooted at `src_root`) and copy every `.rs`
/// file into the mirrored location under `dest_dir`.
fn merge_dir(
    src_root: &Path,
    current_dir: &Path,
    dest_dir: &Path,
    count: &mut u32,
) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            merge_dir(src_root, &path, dest_dir, count)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            // Compute path relative to src_root and mirror it under dest_dir.
            let rel = path.strip_prefix(src_root).expect("path is inside src_root");
            let dest_file = dest_dir.join(rel);

            if let Some(parent) = dest_file.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(&path, &dest_file)?;
            println!("  merged: {}", rel.display());
            *count += 1;
        }
    }
    Ok(())
}

/// Resolve a path, returning an absolute `PathBuf`.
pub fn resolve_path(p: &str) -> PathBuf {
    PathBuf::from(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_file(dir: &Path, rel: &str, content: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_merge_copies_rs_files() {
        let src = TempDir::new().unwrap();
        let dest = TempDir::new().unwrap();

        write_file(src.path(), "lib.rs", "pub fn foo() {}");
        write_file(src.path(), "sub/bar.rs", "pub fn bar() {}");

        let count = merge(src.path(), dest.path()).unwrap();
        assert_eq!(count, 2);

        assert!(dest.path().join("lib.rs").exists());
        assert!(dest.path().join("sub/bar.rs").exists());
        assert_eq!(
            fs::read_to_string(dest.path().join("lib.rs")).unwrap(),
            "pub fn foo() {}"
        );
    }

    #[test]
    fn test_merge_skips_non_rs_files() {
        let src = TempDir::new().unwrap();
        let dest = TempDir::new().unwrap();

        write_file(src.path(), "notes.txt", "ignore me");
        write_file(src.path(), "main.rs", "fn main() {}");

        let count = merge(src.path(), dest.path()).unwrap();
        assert_eq!(count, 1);
        assert!(!dest.path().join("notes.txt").exists());
    }

    #[test]
    fn test_merge_overwrites_existing_file() {
        let src = TempDir::new().unwrap();
        let dest = TempDir::new().unwrap();

        write_file(src.path(), "lib.rs", "pub fn updated() {}");
        write_file(dest.path(), "lib.rs", "pub fn old() {}");

        merge(src.path(), dest.path()).unwrap();

        assert_eq!(
            fs::read_to_string(dest.path().join("lib.rs")).unwrap(),
            "pub fn updated() {}"
        );
    }

    #[test]
    fn test_merge_missing_src_returns_error() {
        let dest = TempDir::new().unwrap();
        let result = merge(Path::new("/nonexistent/path"), dest.path());
        assert!(result.is_err());
    }
}
