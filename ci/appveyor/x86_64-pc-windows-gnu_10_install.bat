set PLATFORM=x64
set MSYS2_ARCH=x86_64
set MSYS2_DIR=msys64
set MINGW_DIR=mingw64
set BIT=64
set COMPILER=rustc

echo Architecture:    %MSYS2_ARCH%
echo Platform:        %PLATFORM%
echo MSYS2 directory: %MSYS2_DIR%
echo MINGW directory: %MINGW_DIR%
echo Bits:            %BIT%
echo Compiler:        %COMPILER%

set DIRCMD=/O:GN
dir
dir C:\%MSYS2_DIR%
dir C:\%MSYS2_DIR%\%MINGW_DIR%

REM Create a writeable TMPDIR
mkdir %APPVEYOR_BUILD_FOLDER%\tmp
set TMPDIR=%APPVEYOR_BUILD_FOLDER%\tmp

set PATH=%PATH%;C:\Users\appveyor\.cargo\bin
echo %PATH%

curl -sSf -o rustup-init.exe https://win.rustup.rs/
rustup-init.exe -y --default-host %TARGET% --default-toolchain %RUST_VERSION%
rustc -Vv
cargo -V

SET PATH=C:\%MSYS2_DIR%\%MINGW_DIR%\bin;C:\%MSYS2_DIR%\usr\bin;%PATH%
SET LIBRARY_PATH=C:\%MSYS2_DIR%\%MINGW_DIR%\lib;%LIBRARY_PATH%

set 

cd %APPVEYOR_BUILD_FOLDER%
dir

bash -lc "pacman -S --needed --noconfirm pacman-mirrors"
bash -lc "pacman -S --needed --noconfirm git"

REM ### Updating the Msys2- and MinGW64-Packages and -Repositories:
bash -lc "pacman -Syu --noconfirm"

REM ### Install the development toolchain:
bash -lc "pacman -S --needed --noconfirm mingw-w64-x86_64-toolchain"

REM ### Add several necessary development tools:
bash -lc "pacman -S --needed --noconfirm base-devel"
bash -lc "pacman -S --needed --noconfirm msys2-devel"

REM ### Installing the SDL2-Library:
REM ### The SDL2-Libraries wil be found via the Environment Variable LIBRARY_PATH!
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2"

REM #### Then at least install the following:
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_image"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_ttf"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_mixer"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_gfx"

