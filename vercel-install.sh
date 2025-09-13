#!/bin/bash

# This is the install script for vercel deployment

set -eux

# Install Rust non-interactively (default to stable, no prompts)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal
# Add cargo and rustup to PATH for this session
export PATH="$HOME/.cargo/bin:$PATH"

# Setup dioxus, prebuilt by ifsheldon with musl
curl -LO https://github.com/ifsheldon/dioxus/releases/download/v0.6.3/dx-x86_64-unknown-linux-musl
mv dx-x86_64-unknown-linux-musl dx
chmod +x dx
