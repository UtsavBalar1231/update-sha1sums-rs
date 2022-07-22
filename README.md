## update-sha1sums script written in RUST

This is a simple script to update the SHA1 sums of the files in the vendor tree of your device tree.

## Compiling

```rust
cargo build --release
```

## Working

> Run the script inside your device tree with the following command to cleanup the proprietary-files.txt

```bash
./update-sha1sums -c
```

> Run the script inside your device tree with the following command to update the proprietary-files.txt

```bash
./update-sha1sums
```

## Contributing

Kindly create a pull request if you find any bug or if you want to contribute to the project.
