echo in Script: $0

set -xe

# let cargo find the SDL2-Libs:
#export LIBRARY_PATH=/usr/local/lib/:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
#echo $LIBRARY_PATH
echo $LD_LIBRARY_PATH

cargo build
cargo build --release

cargo test
cargo test --release

