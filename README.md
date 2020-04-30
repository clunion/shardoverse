
![Shardoverse](./assets/images/Shardoverse-title.png)

| Build System    | Status                                                                         |
|-----------------|--------------------------------------------------------------------------------|
| GitHub Workflow |  ![Rust](https://github.com/clunion/shardoverse/workflows/Rust/badge.svg)      |

A Roguelike  
Peer-to-Peer Multi Player  
Dungeon Explorer  
and Fortress Builder (? maybe later)  
Game written in Rust.  

## Idea and story

The world is shattered to pieces, leaving the survivors each on his own shard of the universe.  

Now one has to run, to explore and to gather what is left, what is needed to survive, in the depth of the shard.

Set in a broken fantasy world with a bit of steam, the player starts on his own in an unfamiliar place.  
Is he alone, are there other survivors, other shards?

## This is a learning project!

We are one software developer (somewhat experienced in writing business software & database applications, not games) and two newcomers, all interested in game programming.

Goal of this project is to learn:
* the Rust programming language,
* a bit of game programming and 
* the network-stuff for peer-to-peer coupling.   

We chose to write a little Roguelike game and maybe add some extras.   
 
## Current state
The project has not really started yet.  
Currently, we are setting up the development environment, select the tools and libraries we will use,
choose some basic assets for graphics and sound and such things.


## Decisions made

| What                                      | Why                                                                                                 |
|-------------------------------------------|-----------------------------------------------------------------------------------------------------|
| [`Roguelike`](http://www.roguebasin.com/) | Programming a roguelike should be simple enough and sound like fun!                                 |
| [`Rust`](https://www.rust-lang.org/)      | Rust is said to be fast and reliable at the same time, so it should be perfect for games            |
| [`MSys2`](https://www.msys2.org)          | To develop somewhat independent of the underlying system platform                                   |
| [`MinGW64`](http://mingw.org/)            | MinGW goes together with MSys2, the decision here is to use 64-Bit libs for simplicity              |
| [`SDL2`](https://wiki.libsdl.org/)        | A well probed and platform independent set of libraries                                             |
| [`git`](https://https://git-scm.com/)     | subversion would have done since this is just for learning, but let's do it right from the start.   |
| [`GitHub`](https://github.com/)           | Er, ok, we are already here. Rust is here, some libs are here, some other games, and so we are too. |


## Considerations

| What                                                  | Why                                                                                      |
|-------------------------------------------------------|------------------------------------------------------------------------------------------|
| [`amethyst`](https://crates.io/crates/amethyst)       | Hm, data-driven development looks like a good idea, maybe a bit strange to grasp. What is the relation to SDL2? Is Amethyst a replacement for SDL2 or can both be integrated? |
| [`bracket-lib`](https://crates.io/crates/bracket-lib) | Formerly known as RLTK/rltk_rs. Wow, that looks cool, and a mountain of docs is available! But that may be a bit much, since our goal is to learn how to do it in Rust by ourselves. On the other hand, may be we could integrate some things. |


## Setting up the development environment

### Install MSYS2 + MinGW64
As described here:  [`MSys2-install`](https://www.msys2.org/wiki/MSYS2-installation/)  
(and possibly this may help:  [`unix-linux-environment-windows`](https://www.booleanworld.com/get-unix-linux-environment-windows-msys2/) )

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

#### In short:
- Edit the pacman configuration file of Msys2: pacman.conf
- add the git-for-windows-**mingw32** for mingw64 in the strangely twisted way described there
- add and accept their signing-keys
- update Msys2 repeatedly by using ```pacboy update``` until there is nothing more to update
- install git via:  
    ```pacboy sync git:x git-doc-html:x git-doc-man:x git-extra: curl:x```
  

### Also add ssh-pageant
This is neccessary to get the automated/transparent SSH-Key login to GitHub working,
it needs additionally an running Putty-Pageant or Keepass-KeeAdent, see further below.

    pacman -S ssh-pageant  
(seems to work, but not sure yet)

Then set it up as described here: [`ssh-pageant`](https://github.com/cuviper/ssh-pageant)  

Make sure to use the same Socket-File in the setup auf ssh-pageant (within the Msys2-Shell)  
and in the configuration of Pageant/KeeAgent (outside the Msys2-Shell, that is: in the Windows environment).  

Here, a file with Path and name like this is used:

    /e/Temp/msys_cyglockfile

Currently, the variant using the cygwin compatible socket seems to work with Msys2.  
If not, try the other (msysgit) variant.
Filepath and name used in KeeAgent:  

    E:\Temp\msys_cyglockfile

### Just for fun, we'll not really need this
    pacman -S mingw-w64-x86_64-vulkan-devel

### Installing the SDL2-Library:
To check which SDL2-Packets are available for Mingw64:

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
If not already done, add a Variable named 'HOME' to the Windows users environment variables:

    HOME=C:\<your own homedir>

This directory is the location from which all the dot-files will be read when the shell is started, in Windows notation.
(one description of how to do this can be seen here: [`environment-variables-windows-10`](https://www.techjunkie.com/environment-variables-windows-10/) )

Add the location of the Mingw64 binaries to the Windows users environment variables PATH, also in Windows notation:

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
add the location of the Rust binaries to the PATH-variable (now in the Unix notation):

      export PATH="/c/Users/<your username>/.cargo/bin:$PATH"

------------------
> An example for a similar environment is the small but nice Asteroids-alike-game [`rust-belt`](https://github.com/johnthagen/rust-belt).  
> And listen to the Game-Music!

## Clone the git repository of Shardoverse
In the MSys2-Shell
navigate to your intended base development directory,
then clone the files from GitHub by issuing:

    git clone https://github.com/clunion/shardoverse

That creates a new sub directory named ```shardoverse``` with all the necessary files in it.

## Build and Run

To build:   

    cd shardoverse  
    cargo build --release

To run:

    cargo run --release

## Some notes regarding SSH Tools
While using a secure key management software is already a good idea,   
it sounds even better to integrate the development tools with that key management.

For a secure login to GitHub SSH can and should be used.

With the process described in the previous sections, that is already working, 
but every time git accesses GitHub (via git clone/pull/push/...), a login-window pops up and asks for the credentials.

To automate the login to GitHub using SSH-Keys with [`KeePass2`](https://keepass.info/) via [`KeeAgent`](https://lechnology.com/software/keeagent/).  

Follow this very fine description (all steps except those regarding Git-Bash, those tools should be already installed by now): 
[`Mendhak's keepass-and-keeagent-setup`](https://code.mendhak.com/keepass-and-keeagent-setup/)  

#### In Short:
* install Git in Msys2 as depiceted in the sections above
* transfer your SSH public Key to GitHub
* install [`KeePass2`](https://keepass.info/) (tested here: Version 2.44) 
* install the KeePass2-Plugin [`KeeAgent`](https://lechnology.com/software/keeagent/) (Version 0.11.1.0, by David Lechner)
* load and activate the ssh keys into KeePas2
* load the Keys into KeeAgent
* Let KeeAgent create (at least) the 'cygwin compatible **socket file**' 
* take note of the socket file's path and filename (in **windows style**, could be somthing like: E:\Temp\msys_cyglockfile)
* in an Msys2 startup script like .bash_profile: set and export a shell environment varible named **SSH_AUTH_SOCK** with the **unix-style** path to the socket file like this:  
```export SSH_AUTH_SOCK="/c/Temp7cyglockfile"```   

Now Git actions involving the GitHub server (like push and pull) should not ask for credentials again. 
But setting this up was a bit shaky first, it did not work right away, so some tinkering around for some time was neccessary.   

Some additional configuration-Info can be found here:  
[`git-for-windows-where-to-find-my-private-rsa-key`](https://serverfault.com/questions/194567/how-do-i-tell-git-for-windows-where-to-find-my-private-rsa-key)  


Alternatively, if the configuration of Git-within-Msys2 with ssh-pageant as proxy to KeeAgent still does not work sufficiently,  
it is reasonable to resort to using a Windows-Git tool like [`TortoiseGit`](https://tortoisegit.org/) instead.   

Originally it was intended not to use TortoiseGit from the start, because one of goals of this project is to learn Git, and that means in the console way first.

## Using Git with Notepad++ 

There is a small problem integrating Notepad++: in usual configuration NP++ opens a new tab for a new text document, 
which will happen every time when Git asks for, say, a commit description.   

Then Git waits for the editor to be colsed, which is not what we want in our development flow.   

Here is a description on how to integrate Notepad++ with Git in MSys2:

[`how-do-i-use-notepad-or-other-with-msysgit`](https://stackoverflow.com/questions/1634161/how-do-i-use-notepad-or-other-with-msysgit)


#### In Short:
First, in Notepad++, allow the use of multiple instances. 
There is a setting in the Notepad++ configuration called 'Multiple instances', the correct setting should read like 'Open session files in a new instance'.

Add the directory which contains the Notepad++-Executable to the Windows environment variable PATH:  
    PATH="C:\Program Files\Notepad++" (probably as one additional line, method is described here: [`environment-variables-windows-10`](https://www.techjunkie.com/environment-variables-windows-10/). )

Create a wrapper shell-script for the call to start Notepad++ from Git within the MSys2-Shell.  
The file may be named _npp_git.sh_ and should contain the following two lines:  

    #!/bin/sh
    notepad++ -multiInst -nosession -notabbar -noPlugin "$*"

Put this shell script _npp_git.sh_ in a directory which can be found through your PATH-Environment variable within the MSys2-Shell.
That could be a sub directory named _bin_ in the users HOME dir, or somewhere else, as long as that dir is in the PATH variable.

Set the script _npp_git.sh_ as the editor in the global config of Git by entering the following in the MSys2-Shell:   

    git config --global core.editor npp_git.sh

Next time something is commited via Git in the Msys2-Shell, an additional instance of Notepad++ should be opened, where the commit description can be entered and which can be losed without bothering the Notepad++-instance where the source code is written.

# Convenient build and run aliases
In the `bin` directy is a set of scripts which may be used to start the diffent build and run variants Conveniently.
Currently, they do simple calls to cargo, but that may change.   
Such a set of build scripts can/should be located in every project folder, thus the method of building and running a project 
is always the same, regardless if it is an C or C++ or, like this time, a Rust project.   
To execute these local scripts, a set of aliases is used. 
The reson behind this is, that then these scripts do not have to be found through the PATH-variable 
(which is not possible to do right when working on several projects in parrallel), instead the scripts are because they are always in the same way relative to the current projects base-directory, 
and thus can be reached via the aliases.   

The naming of the aliases is out of historical reasons. `md` means `make debug`, `sr` is for `start release` and so on.   

The naming of the scripts here is changed to the terms used by Rust (build and run), but my fingers have already wired 'md' in them, so i am not really willing to change that.
    alias md="./bin/build_debug.sh"   
    alias mr="./bin/build_release.sh"   
    alias sd="./bin/run_debug.sh"   
    alias sr="./bin/run_release.sh"   
    alias st="./bin/run_test.sh"   
    alias sdt="./bin/run_debug_tool.sh"   

The more _rusty_ naming would be:
    alias cbd="./bin/build_debug.sh"   
    alias cbr="./bin/build_release.sh"   
    alias crd="./bin/run_debug.sh"   
    alias crt="./bin/run_test.sh"   
    alias crr="./bin/run_release.sh"   


# Debugging
To investigate:   
* ways to debug in this setup

# Logging
To investigate: 
* Whats's the Rust aproach to logging?
* Is there a way to use the plixx-logging-libs?

# Project structure
* Very small main.rs with only minimal logic
* nearly everything goes into libs to make the functions unit-testable
* integration testing (when network stuff ge

# License(s)
The game Shardoverse and all originally created parts of it (source code, texts, descriptions and such) are licensed under the MIT license, see the LICENSE.md file.

Parts which are used by the game, namely several assets like graphics, tiles, tilesets, textures, icons, cursors, sounds, music and fonts,
which are not originally created by direct contributors to Shardoverse, are provided under their own licenses.   
Those parts are the property of their owners and the use within Shardoverse does not remove those licenses.   

For each asset a file is provided in the directory of that asset, describing the origin of the data and, wherever possible, naming the copyright owner and the license.

There is absolutely no intention to infringe any copyrights, trademarks or patents of their respective owners.

If that should happen nevertheless, then that would be an accidental oversight and will be corrected.

# Assets

The following Assets are considered to be used, if their creators/owners do not object:

* ./assets/audio/effects  

* ./assets/audio/music  
    [`cave themeb4 from Brandon75689`](https://opengameart.org/content/cave-theme)  

* ./assets/cursors  
    [`Gauntlet Cursor_by_itsmars`](https://opengameart.org/content/gauntlet-cursor)  
    [`Pointers_by_yd`](https://opengameart.org/content/pointers)  
    [`pointers_part_5_by_yd`](https://opengameart.org/content/pointers-part-5)  
    [`Roguelike_RPG Icons_by_Joe-Williamson`](ttps://opengameart.org/content/roguelikerpg-icons)  

* ./assets/fonts  
    [`Dragonfly Font by Rick Mueller`](https://www.fontspace.com/dragonfly-font-f5775)  
    [`FiraSans-Regular.ttf by Mozilla`](https://github.com/mozilla/Fira)  
    [`NugieRomantic Font by cove703`](https://www.fontspace.com/nugie-romantic-font-f33764)  

* ./assets/graphics/2D/tiles  
    [`DungeonCrawlStoneSoupFull by many`](https://github.com/crawl/tiles/tree/master/releases)  

*   ./assets graphics/2D-isometric/tiles  
    [`rltiles-pack by Mitsuhiro Itakura (maintainer)`](https://opengameart.org/content/64x64-isometric-roguelike-tiles)  

* ./assets/graphics/3D/meshes  
* ./assets/graphics/3D/textures  

* ./assets/gui  
    [`RPG GUI construction kit v1.0_by_Lamoot`](https://opengameart.org/content/rpg-gui-construction-kit-v10)  

* ./assets/images  

* ./assets/videos  



# Maintainer
* [@clunion](https://github.com/clunion)



