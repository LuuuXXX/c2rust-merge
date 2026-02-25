# c2rust-merge

`c2rust-merge` is a command-line tool that merges C-to-Rust translated source
files into an existing Rust project. It is designed to run **after**
`c2rust translate` has produced its output, integrating the translated `.rs`
files into your target Rust codebase.

## Installation

```bash
cargo install --path .
```

## Usage

```
c2rust-merge --translated <TRANSLATED_DIR> --dest <DEST_DIR>
```

| Option | Short | Description |
|---|---|---|
| `--translated <DIR>` | `-t` | Directory produced by `c2rust translate` |
| `--dest <DIR>` | `-d` | Destination Rust project directory |

## Workflow

1. **Translate** – use `c2rust translate` to convert your C source files into Rust:

   ```bash
   c2rust translate compile_commands.json -o translated/
   ```

2. **Merge** – run `c2rust-merge` to copy the translated files into your Rust project:

   ```bash
   c2rust-merge --translated translated/ --dest my_rust_project/src/
   ```

   The tool preserves the relative directory structure of the translation output
   and overwrites any existing file at the destination, so your project always
   receives the latest translation.

## Example

```
$ c2rust-merge --translated ./translated --dest ./my_project/src
Merging translated Rust files
  from: ./translated
  into: ./my_project/src
  merged: lib.rs
  merged: helpers/utils.rs
Done. 2 file(s) merged.
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.