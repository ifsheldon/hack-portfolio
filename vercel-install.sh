#!/bin/bash
set -eux

# Install Rust non-interactively (default to stable, no prompts)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal

# Add cargo and rustup to PATH for this session
export PATH="$HOME/.cargo/bin:$PATH"

# Add the musl target for static linking
rustup target add x86_64-unknown-linux-musl

# Install dioxus-cli for the musl target
cargo install dioxus-cli --target x86_64-unknown-linux-musl

