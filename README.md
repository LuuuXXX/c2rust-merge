# c2rust-merge

A tool for merging C2Rust translation outputs and verifying the result.

## Overview

`c2rust-merge` combines independent Rust analysis outputs produced by
`code_analyse` into files that correspond one-to-one with the original C source
files, then automatically validates that the merged code builds and passes all
tests.

## Usage

```sh
c2rust-merge merge --feature <feature>
```

The `--feature` flag defaults to `default` if omitted.

## What happens during a merge

Running `c2rust-merge merge` executes two phases:

### Phase 1 – Merge

Calls `code_analyse --merge --feature <feature>` to combine the translated Rust
fragments into final source files.

### Phase 2 – Verification (4 steps)

After a successful merge the tool automatically runs the following verification
steps:

| Step | Description |
|------|-------------|
| **1/4 – Cargo build** | Compiles the pure-Rust sub-project located at `.c2rust/<feature>/rust` using `cargo build`. Warnings are suppressed so that only real errors are surfaced. |
| **2/4 – Clean hybrid build** | Runs the project-specific clean command (read from `c2rust-config`) to remove stale hybrid-build artefacts. |
| **3/4 – Hybrid build** | Runs the project-specific build command to compile the C + Rust hybrid binary. |
| **4/4 – Run tests** | Runs the project-specific test command to confirm that the merged code behaves correctly. |

Each step prints its progress, elapsed time, and a clear ✓/✗ result.

### If verification fails

When any verification step fails the tool exits with a non-zero status and
prints the relevant error output.  Fix the underlying issue (for example, edit
the merged Rust source) and re-run `c2rust-merge merge` to repeat both the
merge and the full verification.
