#!/usr/bin/env bash
echo in Script: $0

set -xe

echo "--------------------------------------------------------"
uname -a

echo "--------------------------------------------------------"

sudo apt-get update

sudo apt-get install --assume-yes libgl1-mesa-dev
        
# All SDL2-install-Steps copiedfrom or based on the rust-sdl2 project:
# https://github.com/Rust-SDL2/rust-sdl2

wget https://www.libsdl.org/release/SDL2-2.0.12.tar.gz -O sdl2.tar.gz
tar xzf sdl2.tar.gz
pushd SDL2-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.15.tar.gz -O sdl2_ttf.tar.gz
tar xzf sdl2_ttf.tar.gz
pushd SDL2_ttf-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.tar.gz -O SDL2_image.tar.gz
tar xzf SDL2_image.tar.gz
pushd SDL2_image-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.4.tar.gz -O sdl2_mixer.tar.gz
tar xzf sdl2_mixer.tar.gz
pushd SDL2_mixer-* && ./configure && make && sudo make install && popd

wget --content-disposition -c https://sourceforge.net/projects/sdl2gfx/files/SDL2_gfx-1.0.4.tar.gz -O sdl2_gfx.tar.gz 
tar xzf sdl2_gfx.tar.gz
pushd SDL2_gfx-* && ./configure && make && sudo make install && popd

