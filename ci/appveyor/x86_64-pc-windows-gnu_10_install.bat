cd %APPVEYOR_BUILD_FOLDER%

set DIRCMD=/O:GN
dir

echo Compiler: %COMPILER%
echo Architecture: %MSYS2_ARCH%
echo Platform: %PLATFORM%
echo MSYS2 directory: %MSYS2_DIR%
echo MSYS2 system: %MSYSTEM%
echo Bits: %BIT%

REM Create a writeable TMPDIR
mkdir %APPVEYOR_BUILD_FOLDER%\tmp
set TMPDIR=%APPVEYOR_BUILD_FOLDER%\tmp

SET PATH=C:\%MSYS2_DIR%\%MSYSTEM%\bin;C:\%MSYS2_DIR%\usr\bin;%PATH%

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
REM To check which SDL2-Packets are available for Mingw64:

bash -lc "pacman -Ss --needed --noconfirm mingw-w64-x86_64-SDL2

REM #### Then at least install the following:
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_image"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_ttf"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_mixer"
bash -lc "pacman -S --needed --noconfirm  mingw-w64-x86_64-SDL2_gfx"
