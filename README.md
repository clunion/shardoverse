# shardoverse

                        Shardoverse

                        A Rougelike
                   Peer-to-Peer Multi Player
                     Dungeon Explorer
                    and Fortres Builder (? maybe later)
                    Game written in Rust



## Setting up the development environment

### Install MSYS2 + MinGW64
    as described here:  https://www.msys2.org/wiki/MSYS2-installation/
    (and possibly this may help:  https://www.booleanworld.com/get-unix-linux-environment-windows-msys2/ )

Now there should be an Icon on the desktop to start the MSys2-Shell.
For the further steps, open this shell.
May be it is now the right time to configure this shell/window now for your liking.

### updating the Msys2- and MinGW64-Packages and -Repositories
    pacman -Syuu
    (repeat this until there is nothing more to update)

- install the development toolchain
    pacman -S mingw-w64-x86_64-toolchain

- add several necessary development tools:
    pacman -S base-devel
    pacman -S msys2-devel

- install additional tools for Msys2:
    git   as described at: https://github.com/git-for-windows/git/wiki/Install-inside-MSYS2-proper

- Just for fun, we'll not really need this:
    pacman -S mingw-w64-x86_64-vulkan-devel

- installing the SDL2-Library:
    to chek which SDL2-Pakets are available for Mingw64:
    pacman -Ss mingw-w64-x86_64-SDL2

Then at least install the following:
    pacman -S mingw-w64-x86_64-SDL2_image
    pacman -S mingw-w64-x86_64-SDL2_ttf
    pacman -S mingw-w64-x86_64-SDL2_mixer
    pacman -S mingw-w64-x86_64-SDL2_gfx

- Generate the manpages for the installed Msys2-Tools:
    /usr/bin/mandb --quiet


------------------
## Install Rust
If not already done, add a Variable named 'HOME' to the Windows Users Environmentvariables:
HOME=C:\<your own homedir>
This directory is the location from which all the dot-files will be read when the shell is started, in Windows notation.
(google how to do it, one description of how to do this can be seen here: https://www.techjunkie.com/environment-variables-windows-10/ )

Add the location of the Mingw64 binaries to the Windows Users Environmentvariables PATH, also in Windows notation:
PATH=C:\msys64\mingw64\bin

Then get the initial Rust-Install (use the one for the 64-Bit architecture) as described here:
https://www.rust-lang.org/tools/install

Start From within the MSYS2-Shell:
    rustup-init.exe
    select: 2) Customize installation
enter the following Host-Triple:           --> x86_64-pc-windows-gnu
Default toolchain?                         --> stable
Profile (which tools and data to install)? --> default
Modify PATH variable? (y/n)                --> y  (only for checking)

after that start the installation with: [return]
    select 1) Proceed with installation (default)

Now it's time to configure the environment within the shell.
In one of the dot-files which get executed at start of the shell (in login-mode),
maybe in .bash_profile, add the location of the Rust binaries to the PATH-variable (now in the unix notation):
      export PATH="/c/Users/<your username>/.cargo/bin:$PATH"

------------------
An example for a similiar environment is the Asteroids-alike-game rust-belt:
https://github.com/johnthagen/rust-belt

## Cloning this git repository
In the MSys2-Shell
navigate to your intended base development directory,
the clone the files from GitHub by issueing:
    git clone https://github.com/clunion/shardoverse

