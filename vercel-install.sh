#!/bin/bash
set -eux

# Install Rust non-interactively (default to stable, no prompts)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal

# Add cargo and rustup to PATH for this session
export PATH="$HOME/.cargo/bin:$PATH"

cargo install dioxus-cli

