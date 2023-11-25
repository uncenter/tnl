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

## Install

```sh
cargo install tnl
```

## Testing

Tests use [`assert_cmd`](https://github.com/assert-rs/assert_cmd) to verify the exit codes and content of error messages and [`assert_fs`](https://github.com/assert-rs/assert_fs) to test with temporary files.

## License

[MIT](LICENSE)
