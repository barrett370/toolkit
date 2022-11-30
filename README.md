# toolkit [![Rust](https://github.com/stuarthicks/toolkit/actions/workflows/rust.yml/badge.svg)](https://github.com/stuarthicks/toolkit/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/stuarthicks/toolkit/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/stuarthicks/toolkit/actions/workflows/rust-clippy.yml)

## Dependencies

| Project             | Version |
|---------------------|---------|
| rust-lang.org       | ^1.65   |
| rust-lang.org/cargo | ^0.65   |

## Getting Started

Direct install using cargo:

```sh
cargo build --release
cargo install --path .
```

Or via nix:

```sh
nix-env -i -f .
```

## Updating dependencies

```sh
cargo update
nix-shell --run 'crate2nix generate'
```
