#!/usr/bin/env bash
set -euo pipefail

echo "Installing am..."

if ! command -v cargo &>/dev/null; then
    echo "Error: cargo not found. Install Rust first: https://rustup.rs/"
    exit 1
fi

cargo install --path .

echo ""
echo "Installed! Run 'am --help' to get started."
