#!/usr/bin/env bash

set -e

echo "==> Formatting"
cargo fmt --check

echo "==> Checking"
cargo check

echo "==> Linting"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> Testing"
cargo test

echo
echo "All checks passed!"