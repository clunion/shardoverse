# This script takes care of testing your crate
echo in Script: $0

set -ex

cargo build
cargo build --release

cargo test
cargo test --release

