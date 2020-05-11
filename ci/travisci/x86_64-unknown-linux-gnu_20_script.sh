echo in Script: $0

set -ex

# searching for the SDL2-Libs, to get the correct path:
find /usr/local -name libSDL2-2.0.so.0
ls -al /usr/local/lib/
export LIBRARY_PATH=/usr/local/lib/

cargo build
cargo build --release

cargo test
cargo test --release

