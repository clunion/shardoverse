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
ls -al
tar xzf sdl2.tar.gz
ls -al
pushd SDL2-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.15.tar.gz -O sdl2_ttf.tar.gz
ls -al
tar xzf sdl2_ttf.tar.gz
ls -al
pushd SDL2_ttf-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.tar.gz -O SDL2_image.tar.gz
ls -al
tar xzf SDL2_image.tar.gz
ls -al
pushd SDL2_image-* && ./configure && make && sudo make install && popd

wget https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.4.tar.gz -O sdl2_mixer.tar.gz
ls -al
tar xzf sdl2_mixer.tar.gz
ls -al
pushd SDL2_mixer-* && ./configure && make && sudo make install && popd

wget --content-disposition -c https://sourceforge.net/projects/sdl2gfx/files/SDL2_gfx-1.0.4.tar.gz -O sdl2_gfx.tar.gz 
ls -al
tar xzf sdl2_gfx.tar.gz
ls -al
pushd SDL2_gfx-*/ && ./configure && make && sudo make install && popd
ls -al

# searching for the SDL2-Libs, to get the correct path:
ls -al /usr
ls -al /usr/local
ls -al /usr/local/lib/
export LIBRARY_PATH=/usr/local/lib/
 
find /usr/local -name libSDL2-2.0.so.0


