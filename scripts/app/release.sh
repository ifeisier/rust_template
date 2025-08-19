#!/bin/bash

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
dir="$(cd "$script_dir/../../" && pwd)"

cd "$dir"
cargo fmt --all
cargo clean
cargo build --release
