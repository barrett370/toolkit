toolkit [![Rust](https://github.com/stuarthicks/toolkit/actions/workflows/rust.yml/badge.svg)](https://github.com/stuarthicks/toolkit/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/stuarthicks/toolkit/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/stuarthicks/toolkit/actions/workflows/rust-clippy.yml)
=======

    # Using cargo
    cargo build --release
    cargo install --path .

    # Using nix
    nix run github:cargo2nix/cargo2nix
    nix build
    nix profile install .
