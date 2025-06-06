#!/bin/sh

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
dir="$(cd "$script_dir/../../" && pwd)"

export LOG_NAME=app
export LOG_LEVEL=trace

cd "$dir"
cargo fmt --all
cargo clippy
cargo run
