echo in Script: $0

set -ex

# searching for the SDL2-Libs, to get the correct path:
find /usr/local -name libSDL2-2.0.so.0
ls -al /usr/local/lib/
env
export LIBRARY_PATH=/usr/local/lib/:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
env

cargo build
cargo build --release

cargo test
cargo test --release

