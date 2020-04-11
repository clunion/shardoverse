![shardoverse](./assets/images/shardoverse-title.png)

# shardoverse

[![GitHub Releases][github-release-svg]][github-release]

                        Shardoverse

                        A Rougelike
                   Peer-to-Peer Multi Player
                     Dungeon Explorer
                    and Fortres Builder (? maybe later)
                    Game written in Rust

## Idea and story

The world is shattered to pieces, leaving the survivers each on his own shard of the universe.
Now one has to run, to explore and to gather what is left, what is needed to survive, in the depth of the shard.

Set in a broken fantasy world with a bit of steam, the player starts on his own in an unfmiliar place.
Is he alone, are there other survivers, other shards?

## This is a learning project!

We are one software developer (expierienced in writing business & database applications)
and two newcomers, all interested in game programming.

Goal of this project is to learn:
- the Rust programming language and
- a bit of game programming.

We chose to write a little Roguelike game and maybe add some extras.

## Current state
The project has not really started yet.
Currently, we are setting up the development environment, select the tools and libraries we will use,
choose some basic assets for grafics and sound and decide on the license.


## Decisions made

| What                                      | Why                                                                                      |
|-------------------------------------------|------------------------------------------------------------------------------------------|
| [`Rougelike`](http://www.roguebasin.com/) | Programming a rougelike should be simple enough and sound like fun!                      |
| [`Rust`](https://www.rust-lang.org/)      | Rust is said to be fast and reliable at the same time, so it should be perfect for games |
| [`MSys2`](https://www.msys2.org)          | To develop somewhat independend of the underlying system platform                        |
| [`MinGW64`](http://mingw.org/)            | MinGW goes together with MSys2, the decision here is to use 64-Bit libs for simplicity   |
| [`SDL2`](https://wiki.libsdl.org/)        | A well probed and platform independend set of libraries                                  |

## Considerations

| What                                                  | Why                                                                                      |
|-------------------------------------------------------|------------------------------------------------------------------------------------------|
| [`amethyst`](https://crates.io/crates/amethyst)       | Hm, data-driven development looks like a good idea, maybe a bit strange to grasp. What is the relation to SDL2? Is Amethyst a replacement for SDL2 or can both be integrated? |
| [`bracket-lib`](https://crates.io/crates/bracket-lib) | Formerly known as RLTK/rltk_rs. Wow, that looks cool, and a mountain of docu is available! But that may be a bit much, and our goal is lerning how to do it in Rust. On the other hand, may be we could integrate some things. |


## Setting up the development environment

### Install MSYS2 + MinGW64
As described here:  [`MSys2-install`](https://www.msys2.org/wiki/MSYS2-installation/)
(and possibly this may help:  [`unix-linux-environment-windows`](https://www.booleanworld.com/get-unix-linux-environment-windows-msys2/)

Now there should be an Icon on the desktop to start the MSys2-Shell.
For the further steps, open this shell.
May be it is now the right time to configure this shell/window for your liking.

### Updating the Msys2- and MinGW64-Packages and -Repositories
    pacman -Syuu

(repeat this until there is nothing more to update)

### Install the development toolchain
    pacman -S mingw-w64-x86_64-toolchain

### Add several necessary development tools
    pacman -S base-devel
    pacman -S msys2-devel

### Install git for Msys2
as described at: [`git: Install-inside-MSYS2-proper`](https://github.com/git-for-windows/git/wiki/Install-inside-MSYS2-proper)

### Just for fun, we'll not really need this
    pacman -S mingw-w64-x86_64-vulkan-devel

### Installing the SDL2-Library:
to chek which SDL2-Pakets are available for Mingw64:

    pacman -Ss mingw-w64-x86_64-SDL2

#### Then at least install the following:
    pacman -S mingw-w64-x86_64-SDL2_image
    pacman -S mingw-w64-x86_64-SDL2_ttf
    pacman -S mingw-w64-x86_64-SDL2_mixer
    pacman -S mingw-w64-x86_64-SDL2_gfx

### Generate the manpages for the installed Msys2-Tools
    /usr/bin/mandb --quiet

------------------
## Install Rust
If not already done, add a Variable named 'HOME' to the Windows Users Environmentvariables:

    HOME=C:\<your own homedir>

This directory is the location from which all the dot-files will be read when the shell is started, in Windows notation.
(one description of how to do this can be seen here: [`environment-variables-windows-10`](https://www.techjunkie.com/environment-variables-windows-10/)

Add the location of the Mingw64 binaries to the Windows Users Environmentvariables PATH, also in Windows notation:

    PATH=C:\msys64\mingw64\bin

Then get the initial Rust-Install (take the one for the 64-Bit architecture)
as described here:  [`Install Rust`](https://www.rust-lang.org/tools/install)

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

### Configure the environment within the MSys2-Shell

In one of the dot-files which get executed at start of the shell (in login-mode), maybe in .bash_profile,
add the location of the Rust binaries to the PATH-variable (now in the unix notation):

      export PATH="/c/Users/<your username>/.cargo/bin:$PATH"

------------------
> An example for a similiar environment is the Asteroids-alike-game [`rust-belt`](https://github.com/johnthagen/rust-belt)!

## Cloning this git repository
In the MSys2-Shell
navigate to your intended base development directory,
then clone the files from GitHub by issueing:

    git clone https://github.com/clunion/shardoverse

That creates a new sub directory with all the necessary files in it.

## Build and Run

To build:

    cargo build


To run:

    cargo run --release


# Maintainer
* [@clunion](https://github.com/clunion)



