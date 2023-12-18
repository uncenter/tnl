# tnl

Find files with missing trailing newlines.

```
warning: missing "blazingly fast" in description
  --> README.md:3:0
   |
3  | ...                   Find files with missing trailing newlines.
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Blazingly fast tool to find files with missing trailing newlines.`
   |
```

## Installation

```sh
cargo install tnl
```

## Usage

```sh
tnl <files>
```

You can use the `--fix` flag to automatically add trailing newlines to files without. `tnl` prints a message upon completion, listing the number of files checked (and the number of files fixed, if ran with `--fix`). This message can be disabled with the `--quiet` flag.

## Tests

Tests use [`assert_cmd`](https://github.com/assert-rs/assert_cmd) to verify the exit codes and content of error messages and [`assert_fs`](https://github.com/assert-rs/assert_fs) to test with temporary files.

## License

[MIT](LICENSE)
