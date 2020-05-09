# This script takes care of testing your crate
echo in Script: $0
echo in Script: x86_64-unknown-linux-gnu_20_script.sh

set -ex

cargo build
cargo build --release

cargo test
cargo test --release

