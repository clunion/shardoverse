# This script takes care of testing your crate

set -ex

cargo build
cargo build --release

cargo test
cargo test --release

