#!/bin/bash

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
dir="$(cd "$script_dir/../../" && pwd)"

export LOG_NAME=app
export LOG_LEVEL=trace

cd "$dir"
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic -D clippy::nursery -A clippy::module-name-repetitions
cargo run
