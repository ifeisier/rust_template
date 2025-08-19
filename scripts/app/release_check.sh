#!/bin/bash

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
dir="$(cd "$script_dir/../../" && pwd)"

cd "$dir"
# cargo install --locked cargo-deny
rm Cargo.lock
cargo fmt --all
cargo clean
cargo update
cargo check
cargo deny check
cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic -D clippy::nursery -A clippy::module-name-repetitions
