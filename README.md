# A Rust and game programming learning project

![Shardoverse](https://raw.githubusercontent.com/clunion/shardoverse/master/assets/images/Shardoverse-title.png)

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

|CPU     | Operating System             |Rust            | Development Environment| CI System      | Status                                                                                                    |  
|:-------|:-----------------------------|:--------------:|:----------------------:|:--------------:|:----------------------------------------------------------------------------------------------------------|  
| x86_64 | Ubuntu-latest, MacOS-latest  |stable          | gnu                    | GitHub Actions | ![shardoverse](https://github.com/clunion/shardoverse/workflows/shardoverse/badge.svg)                    |
| x86_64 | Linux, FreeBSD, Apple-Darwin |stable, nightly | gnu                    | Travis-CI      | ![Travis-CI](https://travis-ci.com/clunion/shardoverse.svg?branch=master)                                 |
| x86_64 | MS-Windows                   |stable, nightly | MSys2+MinGW64          | AppVeyor       | ![AppVeyor](https://ci.appveyor.com/api/projects/status/github/clunion/shardoverse?branch=master&svg=true)|

## What is this?

A learning project to get into the coding of:

* __A Roguelike__
* __Peer-to-Peer Multi Player__
* __Role Playing Game__
* __written in Rust.__

When it gets near completion a  __fortress building__ scenario might get added.

## Idea and story

The world is shattered into pieces, leaving the survivors each on his own shard of the universe.

Now one has to run, to explore and to gather what is left, what is needed to survive, in the depth of the shard.

Set in a broken fantasy world with a bit of steam, the player starts on his own in an unfamiliar place.
Is he alone, are there other survivors, other shards?

## This is a learning project!

We are one software developer (somewhat experienced in writing business software & database applications, not games) and two newcomers, all interested in game programming.

Goal of this project is to learn:

* the Rust programming language,
* a bit of game programming and
* the network-stuff for peer-to-peer coupling.

We chose to write a little roguelike game and perhaps add some extras.

## Current state

Most things of the development environment are set up in a reasonable way now, including logging and a bit of configuration file handling.
Git, some Continuous Integration pipelining and the generating of source documentation is working.  
Several basic assets for graphics and sound are selected.

Currently, we start with creating the first easy data structures (for creatures and items), expand the project structure while applying some design principles.

Using an Entity Component System (ECS) model is in consideration, perhaps [`Specs or Legion`](https://csherratt.github.io/blog/posts/specs-and-legion/) or the ECS Bevy will be used.

After some experimentation with SDL2, the handling of windows turned out not to be satisfactory:
In the Rust-environment it seems not to be possible to get continued redraws and events while moving or resizing windows. So we switched to winit for the window-handling (which includes input and event handling) and are about to try bevy engine for drawing and ECS (and perhaps more).

## Software Design Principles

Design principles for a small learning project, for a small computer game? Yes, definitely.

* [`KISS - Keep It Simple, Stupid!`](https://en.wikipedia.org/wiki/KISS_principle) (preferred interpretation: Keep it simple and smart)
* [`TDD - Test Driven Design`](https://en.wikipedia.org/wiki/Test-driven_development) (Test first, Test much, Integrate early, ...)
* [`Secure by design`](https://en.wikipedia.org/wiki/Secure_by_design) (at least for everything regarding the networking)
* [`SOLID`](https://en.wikipedia.org/wiki/SOLID) (Well, a bit much, but good to have at least one in the team who knows what it means.)

Even though this is just a small game project it is regarded as a good idea to have at least some principles in mind when designing the architecture of the program.
It is surely not necessary to follow every detail of these principles, but the intent of most principles should always be clear.

## Decisions made

| What                                                            | Why                                                                                                                                        |
|-----------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| [`Roguelike`](http://www.roguebasin.com/)                       | Programming a roguelike should be simple enough and sound like fun!                                                                        |
| [`Rust`](https://www.rust-lang.org/)                            | Rust is said to be fast and reliable at the same time, so it should be perfect for games                                                   |
| [`MSys2`](https://www.msys2.org)                                | To develop somewhat independent of the underlying system platform                                                                          |
| [`MinGW64`](http://mingw.org/)                                  | MinGW goes together with MSys2, the decision here is to use 64-Bit libs for simplicity                                                     |
| [`git`](https://https://git-scm.com/)                           | subversion would have done since this is just for learning, but let's do it right from the start.                                          |
| [`GitHub`](https://github.com/)                                 | Er, ok, we are already here. Rust is here, some libs are here, some other games, and so we are too.                                        |
| [`MS-Windows`](https://en.wikipedia.org/wiki/Microsoft_Windows) | Not really a choice, that's what most already have. Even though, the environment and tools used here are meant to be platform independent. |
| [`AreWeIDEyet`](https://areweideyet.com/)                       | Ah, yes. A bit. Somehow. After gaining first experiences with Rust, it seems a debugger may get helpful for understanding how things work, when they work. VisualStudioCode is chosen for a start. More info on IDEs for Rust can be found here: [whats-the-best-ide-for-developing-in-rust](https://medium.com/cloud-native-the-gathering/whats-the-best-ide-for-developing-in-rust-5087d46006f5). |
| [`winit`](https://github.com/rust-windowing/winit)              | Cross-platform window creation and management. Has quite some features, looks a bit strange at first, but works reasonably well. Seems to be the standard for window handling in rust. |

## In Consideration

| What                                                  | Why                                                                                      |
|-------------------------------------------------------|------------------------------------------------------------------------------------------|
| [`Bevy engine`](https://github.com/bevyengine/bevy)   | Promising new engine, should be fast, comes with an own variant of an ECS                |
| [`amethyst`](https://crates.io/crates/amethyst)       | Hm, data-driven development looks like a good idea, maybe a bit strange to grasp.        |
| [`bracket-lib`](https://crates.io/crates/bracket-lib) | Formerly known as RLTK/rltk_rs. Wow, that looks cool, and a mountain of docs is available! But that may be a bit much, since our goal is to learn how to do it in Rust by ourselves. On the other hand, may be we could integrate some things. |
| [`Legion (ECS)`](https://github.com/TomGillen/legion) | A newer ECS, aimed for high performance. Looks tempting, currently not as well documented as Specs. So for learning purposes Specs looks better suited. |
| [`Specs (ECS)`](https://github.com/amethyst/specs)    | Using an [`Entity Component System (ECS)`](https://en.wikipedia.org/wiki/Entity_component_system) seems to be the way to go. There are several available, **Specs** is used in the [`Roguelike-Tutorial`](https://bfnightly.bracketproductions.com/rustbook/), in [`Rust Sokoban`](https://sokoban.iolivia.me/) and (currently) in Amethyst (though [`Amethyst considers changing to Legion`](https://github.com/amethyst/rfcs/issues/22)). |

## Dropped

| What                                                  | Why                                                                                      |
|-------------------------------------------------------|------------------------------------------------------------------------------------------|
| [`SDL2`](https://wiki.libsdl.org/)                    | window handling: no redraws and events while moving or resizing windows.                 |

## Setting up the development environment

### Install MSYS2 + MinGW64

From here: [`Msys2.org`](https://www.msys2.org/), as described here:  [`MSys2-install`](https://www.msys2.org/wiki/MSYS2-installation/)
(and possibly this may help:  [`unix-linux-environment-windows`](https://www.booleanworld.com/get-unix-linux-environment-windows-msys2/) )

Now there should be an Icon on the desktop to start the MSys2-Shell.

For the further steps, open this shell.

Perhaps it is now the right time to configure this shell/window for your liking.

### Updating the MSys2- and MinGW64-Packages and -Repositories

    pacman -Syuu

(repeat this until there is nothing more to update)

### Install the development toolchain

    pacman -S mingw-w64-x86_64-toolchain

### Add several necessary development tools

    pacman -S base-devel
    pacman -S msys2-devel

### Needed for the Bevy fast-compile configuration (linking does not work yet with this MSys2 settings)

    pacman -S mingw-w64-x86_64-clang
    pacman -S mingw-w64-x86_64-lld

### Just for fun, we'll not really need this:

    pacman -S mingw-w64-x86_64-vulkan-devel

#### Generate the manpages for the installed Msys2-Tools

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

Now it's time to configure the windows-environment and the environment within the shell.

### Configure the environment within the MSys2-Shell

In one of the dot-files which get executed at start of the shell (in login-mode), perhaps in the *.bash_profile*, add the location of the Rust binaries to the PATH-variable (now in the Unix notation):

      export PATH="/c/Users/<your username>/.cargo/bin:$PATH"

#### Tell Rust where to find Libraries!

In the MSys2 environment on MS-Windows (and Apple-Darwin), Rust (or, more precisely, cargo) looks in the path stored in the shell variable named LIBRARY_PATH.
This Variable has to be set in one of the MSys2-Shell startup scripts (.bash_profile or the like) to the correct path containing libs.
The path might be /mingw64/lib, then set the variable this way:

    export LIBRARY_PATH=/mingw64/lib

On Unix-like systems (Linux, BSD, ...), the same mechanism happens, but the Shell variable is called *LD_LIBRARY_PATH* and the correct path may look like /usr/local/lib/. Then it also has to be set in the start up scripts, in this case like this:

    export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH

## Convenient build and run aliases

In the `bin` directory of the shardoverse repository is a set of scripts which may be used to start the different build and run variants Conveniently.

Currently, they do simple calls to cargo, but that may change.

A set of this build- and run-scripts are meant to be located in every project's bin folder, thus the method of building and running a project is always the same, regardless if it is an C or C++ or, like this time, a Rust project.

To execute these local scripts, a set of aliases is used.

The reason behind this is, that these scripts then do not have to be found through the PATH-variable (which is not possible to do right when working on several projects in parallel), instead the scripts are found because they are always relative to the current project's base-directory in the same way, and thus can be reached via these aliases.

The naming of the aliases is out of historical reasons. `md` means `make debug`, `sr` is for `start release` and so on.

The naming of the scripts here is changed to the terms used by Rust (build and run), but my fingers have already wired 'md' into them, so I am not really willing to change that.

    alias md="./bin/build_debug.sh"
    alias mr="./bin/build_release.sh"
    alias sd="./bin/run_debug.sh"
    alias sr="./bin/run_release.sh"
    alias st="./bin/run_test.sh"

The more _rusty_ naming would be:

    alias cbd="./bin/build_debug.sh"
    alias cbr="./bin/build_release.sh"
    alias crd="./bin/run_debug.sh"
    alias crt="./bin/run_test.sh"
    alias crr="./bin/run_release.sh"

    alias sdt="./bin/start_debug_tool.sh"
    alias sdtt="./bin/start_debug_tool_tests.sh"
    alias sdti="./bin/start_debug_tool_integration.sh"
 
    alias gcp="./bin/git_commit-push.sh"

Currently, the start_debug_tool[*] scripts start _gdb_ as the debug tool.

start_debug_tool_tests.sh feeds the unit-tests-executable into the throat of gdb,

start_debug_tool_integration.sh uses the integration-test-executable.

The build scripts use [`clippy`](https://github.com/rust-lang/rust-clippy/), the Rust linter (since the 2018 edition in stable).
If it is not installed yet, this can be done in the MSys2-Shell via:

    rustup component add clippy

## Reading Material <-- start here

Now Rust compiler and some tools and libraries should be ready to use.

To get into Rust itself, the following resources can be used.

While *The Rust Programming Language* is the **THE BOOK** and is written in a way that encourages to read it from start to end, it is also possible to just read the first few chapters, and then start picking topics when they are needed.

### Books on Rust itself

* [`The Rust Programming Language`](https://doc.rust-lang.org/book/) Here we start: The source of truth regarding Rust, for learning step by step, or in a selective manner. It is probably a good idea to read at least this book completely prior to naming oneself a new Rustacean.
* [`Rust By Example`](https://doc.rust-lang.org/stable/rust-by-example/) Concise, many topics to delve into, and well prepared examples showing working solutions.

### Rust and the Rust-Ecosystem

* [`Rust Cookbook`](https://rust-lang-nursery.github.io/rust-cookbook/intro.html) This is a unusual kind of book, in the way it is presented as a community effort on GitHub. It is also focusses on examples, this time for many important crates in the Rust ecosystem.
* [`The rustdoc book`](https://doc.rust-lang.org/rustdoc/) How to docu-comment the code, so that rustdoc can generate some standardized documentation.
* [`The Cargo Book`](https://doc.rust-lang.org/cargo/)  The Rust Package-Manager which also builds, runs and tests our code

### Roguelike programming in Rust

* [`Roguelike Tutorial - In Rust`](https://bfnightly.bracketproductions.com/rustbook/) - A great tutorial of how to write a roguelike in Rust, covering every aspect and using modern techniques like Data Driven Design and Entity Component Systems!

### Install git for Msys2

Git is used for version control, to coordinate the different contributors working in parallel, as a base for continuous integration and, later, for publishing the results.

Git can be set up as described in 9 steps at: [`git: Install-inside-MSYS2-proper`](https://github.com/git-for-windows/git/wiki/Install-inside-MSYS2-proper)

#### In short:

* Edit the pacman configuration file of Msys2: /etc/**pacman.conf** (probably in windows to be found at C:\msys64\etc\pacman.conf)
* add the git-for-windows-**mingw32** for mingw64 in the strangely twisted way described there
* add and accept their **signing-keys**
* sync the new repository via ```pacman -Syyuu```
* update Msys2 repeatedly by using ```pacman -Suu``` until there is nothing more to update
* install git via:
    ```pacman -S mingw-w64-x86_64-{git,git-doc-html,git-doc-man,curl} git-extra```

### Also add ssh-pageant:

This is necessary to get the automated/transparent SSH-Key login to GitHub working.
(It may additionally need a running Putty-Pageant, Keepass-KeeAgent or other SSH-agent, see further below)

    pacman -S ssh-pageant

Then set it up as described here: [`ssh-pageant`](https://github.com/cuviper/ssh-pageant)

Make sure to use the same Socket-File in the setup of ssh-pageant (within the Msys2-Shell)
and in the configuration of Pageant/KeeAgent (outside the Msys2-Shell, that is: in the Windows environment).

Here, a file with Path and name like this is used:

    /e/Temp/msys_cyglockfile

Currently, the variant using the Cygwin compatible socket seems to work with Msys2.
If not, try the other (msysgit) variant.
File path and name used in KeeAgent:

    E:\Temp\msys_cyglockfile

## Clone the git repository of Shardoverse

In the MSys2-Shell:

* navigate to your intended base development directory,
* then clone the files from GitHub by issuing:

    git clone <https://github.com/clunion/shardoverse>

That creates a new sub directory named ```shardoverse``` with all the necessary files in it.

## Build and Run Shardoverse

To build:

    cd shardoverse
    cargo build --release

To run:

    cargo run --release

## Basic Usage

Since there is no game yet, the usage is simple.

After starting shardoverse, the program can be left by closing the window.

| Key           | Action                                                                  |
|---------------|-------------------------------------------------------------------------|
| ESC           | Exit program                                                            |
| Keypad-'+'    | Scale up / Zoom-In by 1 pixel per tile, until max of 255                |
| Keypad-'-'    | Scale down / Zoom out by 1 pixel per tile, until min of 8               |
| Keypad-','    | Reset scaling to the base of 32 (which also is the original tile-size)  |
| Keypad-'*'    | Toggle delay in main-loop on or off                                     |
| Mouse-Wheel   | Zoom in/out with dynamic step size                                      |
| P             | Paint colored pixels in main window (gets immediately painted over now) |

When the program is started, a (currently empty) window should appear on screen.  
If the window fails to appear, it is probably off-screen.
There are some ways to get that fixed:

* start the executable with the parameter --windowreset. This could be done via:

>     cargo run --release -- --windowreset

* __or:__ remove the file `shardoverse.ini`. The window coordinates are then initialized with defaults, which should be on the main screen.
* __or:__ change the content of `shardoverse.ini`, it is an ordinary human readable text, and correct the window position therein.

------------------
> An example for a similar environment is the small and nice Asteroids-alike-game [`rust-belt`](https://github.com/johnthagen/rust-belt).  
> And listen to the Game-Music!

------------------

## Some notes regarding SSH Tools

While using a secure key management software is already a good idea,
it sounds even better to integrate the development tools with that key management.

For a secure login to GitHub, SSH can and should be used.

With the process described in the previous sections, that is already working,
but every time git accesses GitHub (via git clone/pull/push/...), a login-window pops up and asks for the credentials.

To automate the login to GitHub using SSH-Keys with [`KeePass2`](https://keepass.info/) via [`KeeAgent`](https://lechnology.com/software/keeagent/) follow this very fine description (all steps except those regarding Git-Bash, those tools should be already installed by now):
[`Mendhak's keepass-and-keeagent-setup`](https://code.mendhak.com/keepass-and-keeagent-setup/)

### In Short:

* install Git in Msys2 as depicted in the sections above
* transfer your SSH public Key to GitHub
* install [`KeePass2`](https://keepass.info/) (tested here: Version 2.44)
* install the KeePass2-Plugin [`KeeAgent`](https://lechnology.com/software/keeagent/) (Version 0.11.1.0, by David Lechner)
* load and activate the ssh keys into KeePass2
* load the Keys into KeeAgent
* Let KeeAgent create (at least) the 'Cygwin compatible **socket file**'
* take note of the socket file's path and filename (in **windows style**, could be something like: E:\Temp\msys_cyglockfile)
* in an Msys2 startup script like .bash_profile: set and export a shell environment variable named **SSH_AUTH_SOCK** with the **unix-style** path to the socket file like this:
```export SSH_AUTH_SOCK="/c/Temp/cyglockfile"```

Now Git actions involving the GitHub server (like push and pull) should not ask for credentials again.  
Note: Setting this up was a bit shaky first, it did not work right away, so some tinkering around for some time was necessary. It may be the same at your side.

Some additional configuration-Info can be found here:
[`git-for-windows-where-to-find-my-private-rsa-key`](https://serverfault.com/questions/194567/how-do-i-tell-git-for-windows-where-to-find-my-private-rsa-key)

Alternatively, if the configuration of Git-within-Msys2 with ssh-pageant as proxy to KeeAgent still does not work sufficiently,
it is reasonable to resort to using a Windows-Git tool like [`TortoiseGit`](https://tortoisegit.org/) instead.

Originally it was intended not to use TortoiseGit from the start, because one of goals of this project is to learn Git, and that means in the console way first.

## Using Git with Notepad++

There is a small problem integrating Notepad++: in usual configuration NP++ opens a new tab for a new text document,
which will happen every time when Git asks for, say, a commit description.

Then Git waits for the editor to be closed, which is not what we want in our development flow.

Here is a description on how to integrate Notepad++ with Git in MSys2:

[`how-do-i-use-notepad-or-other-with-msysgit`](https://stackoverflow.com/questions/1634161/how-do-i-use-notepad-or-other-with-msysgit)

### In Short:

First, in Notepad++, allow the use of multiple instances.
There is a setting in the Notepad++ configuration called 'Multiple instances', the correct setting should read like 'Open session files in a new instance'.

Add the directory which contains the Notepad++-Executable to the Windows environment variable PATH:
    PATH="C:\Program Files\Notepad++" (probably as one additional line, method is described here: [`environment-variables-windows-10`](https://www.techjunkie.com/environment-variables-windows-10/). )

Create a wrapper shell-script for the call to start Notepad++ from Git within the MSys2-Shell.
The file may be named _npp_git.sh_ and should contain the following two lines:

    #!/bin/sh
    notepad++ -multiInst -nosession -notabbar -noPlugin "$*"

Put this shell script _npp\_git.sh_ in a directory which can be found through your PATH-Environment variable within the MSys2-Shell.
That could be a sub directory named _bin_ in the users HOME dir, or somewhere else, as long as that dir is in the PATH variable.

Set the script _npp\_git.sh_ as the editor in the global config of Git by entering the following in the MSys2-Shell:

    git config --global core.editor npp_git.sh

Next time something is committed via Git in the Msys2-Shell, an additional instance of Notepad++ should be opened, where the commit description can be entered and which can be closed without bothering the Notepad++-instance where the source code is written.

## Coding in Rust with UltraEdit

If you happen to have UltraEdit, it can be used for Git much in a similar way like Notepad++.
There are several configuration settings which relate to session handling,
and thus how to open the Git commit comment in a separate editor-session.  
One is called 'Maintain separate process for each file opened from external application'
and can be found at Menu:Advanced/Configuration/Application layout/Miscellaneous.  

Some more description about this can be found here: [how-to-open-different-windows-of-ultraedit](http://forums.ultraedit.com/how-to-open-different-windows-of-ultraedit-with-fi-t17820.html)

The following content for wrapper shell-script works in UltraEdit64 **Versions 27**:

    #!/bin/sh   
    "C:\Program Files\IDM Computer Solutions\UltraEdit\uedit64.exe" $*   

One additional remark regarding the **syntax coloring of Rust-code in UltraEdit**:
UltraEdit defines the syntax coloring in word-files, for Rust such a file is named rust.uew.
The currently available wordfile defines the single apostrophe (') as one of the string delimiters.
This leads to a funny coloring when the Rust-code is using labeled loops or lifetimes, which also use the single apostrophe.

**Workaround:** when the apostrophe is removed form the String Chars list (it is in the first line of rust.uew),
the coloring looks much better, at least around loop-labels and lifetimes.

## Install an IDE (VSCode, optional)

Start with download of [Visual Studio Code](https://code.visualstudio.com/).  
Follow the instructions shown on the site (Install as usual).

Infos on how to get started with VSCode can be found on the welcome-page within the program,
or here: [VSCode Introvideos](https://code.visualstudio.com/docs/introvideos/basics)

### Add some VSCode Extensions

A minimal setup for developing with Rust in VSCode and to be able to debug:  

* [C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

That should look like this:  

![Plugins](.\doc\IDE\VSCode\Plugins_Initial.jpg)

Setting up the debugging is further described here:  [how-to-debug-rust-with-visual-studio-code](https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/)

### Add Shardoverse to VSCode

To start developing the Shardoverse project with VSCode, open the filesystem folder of shardoverse in VSCode's project Explorer:  
![Plugins](.\doc\IDE\VSCode\Open_Folder.jpg)

### Using the MSys2-Shell in VSCode as the Terminal

Follow the steps as described here:
[How to integrate Git Bash with Visual Studio Code on Windows](https://dev.to/simbo1905/how-to-integrate-git-bash-with-visual-studio-code-on-windows-3217)

First type "Ctrl+Shift+P" to open the command search and type/select "Open User Settings". If this display a settings search page you will need to hit the ”{}” at the top right to get to the raw JSON. Merge the following settings ensuring to use paths that match where you installed the "bash.exe"

Add the following lines to the file _settings.json_ of VSCode:

    "terminal.integrated.shell.windows": "C:\\msys64\\usr\\bin\\bash.exe",
    "terminal.integrated.shellArgs.windows": ["--login"],
    "terminal.integrated.env.windows": {
        "CHERE_INVOKING":"1",
        "MSYSTEM":"MINGW64"
    }

Check if the path to _bash.exe_ is correct, adjust here if not.

With this simple setup, coding is already quite feasible.
For convenience, adding some more VSCode Extensions:

| Extension                                                                                                     | Description                                                            |
|---------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|
|[better-toml](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)                         | Adds some editing supporting features for toml files.                  |
|[code-spell-checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)| Spellchecks while typing, including text in comments, string definitions and more. |
|[crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)                                | Help to manage dependencies while using Cargo.toml.                    |
|[git Lense](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)                               | Cited: "GitLens supercharges the built-in Visual Studio Code Git capabilities. It helps you to visualize code authorship at a glance via inline Git blame annotations and code lens, seamlessly navigate and explore the history of a file or branch, gain valuable insights via powerful comparison commands, and so much more." |
|[gitignore](https://marketplace.visualstudio.com/items?itemName=codezombiech.gitignore)                        | Some assistance in working with .gitignore files, language support and such. |
|[project-manager](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)             | Enables access to projects from within VSCode, no matter where they are located. Define Projects (as Favorites) or choose to auto-detect Git, Mercurial or SVN repositories, VSCode folders, or other folder. |
|[vscode-markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)      | Includes a library of rules to encourage standards and consistency for Markdown files. |
|[github-markdown-preview](https://marketplace.visualstudio.com/items?itemName=bierner.github-markdown-preview) | Changes VS Code's built-in markdown preview to match Github markdown rendering in style and content, adds 4 more Extensions to do that. |
|[vscode-icons](https://marketplace.visualstudio.com/items?itemName=vscode-icons-team.vscode-icons)             | Adds some Icons to the project explorer tree.                          |
|[Rust Test Explorer](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter)    | Rust Test Explorer for VS Code that enables viewing and running your Rust tests from the VS Code Sidebar. |
|[Test Explorer UI](https://marketplace.visualstudio.com/items?itemName=hbenl.vscode-test-explorer)             | An extensible user interface for running tests in VS Code, used by Rust Test Explorer. |

A nice explanation on how to use (an already configured) git inside of VSCode is found here:  [How to use Git inside of VSCode](https://www.youtube.com/watch?v=VOwyH2-VCVY) (from 2017).  
A more detailed tutorial which includes the setting up of git: [How to use Git inside of VSCode - 2020](https://www.youtube.com/watch?v=F2DBSH2VoHQ)

For the key-bindings, i started with Notepad++ keymap,
moved to the Eclipse keymap and ended for now with a mixture of both,
mainly for the Debugging keys F5-F8 (preferred to be like in Visual Studio) and the handling of multi-cursor editing (preferred to be like in UltraEdit and/or Notepad++)
(a newer variant of what was known as column editing).  

## Project structure

* Very small main.rs with parameter checking, initialization of the logging and only minimal logic
* nearly everything goes into module-files to make the functions unit-testable
* for simplicity, we start using a 'central-core' module to bind the parts and layers together
* integration testing will be set up early (when beginning with the network stuff)

## Logging

For logging the standard crate [```log```](https://github.com/rust-lang/log) is used in combination with crate [```flexi_logger```](https://github.com/emabee/flexi_logger) as a backend.

In the current configuration, detailed logfiles are written into the directory ```log```.  
Additionally, colored error and warning-messages are written to the console, using a custom log formatter.  

The common log-level can also be changed at runtime by setting the environment variable RUST_LANG.  

For the code currently in the work, the log level is set to debug, so only for that code file(s) there is output at debug-level.  

In the release variant of Shardoverse, only errors and warnings are
included in the executable, all else are 'compiled out' by cargo options.

## Debugging

An introduction to debugging Rust programs can be found here: [`Where We're Going, We Don't Need Println!`](https://www.joshmatthews.net/debugging-workshop/)

To investigate:

* more ways to debug in this setup

## Continuous Integration (CI) with Rust

This is not needed to build Shardoverse locally, only describes what is done here to get the build CI running with GitHub.

* follow the templates provided by and described on [`trust`](https://github.com/japaric/trust)
* this requires accounts at [`Travis CI`](https://travis-ci.org/) and [`AppVeyor`](https://www.appveyor.com/), for both free accounts for open source projects are available.

## Some Notes on AppVeyor

First idea was to make a very simple and minimal CI-Setup, using only the appveyor.yml file and defining all stages there.
Setting this up in the crude mixture of weird PowerShell and windows-cmd commands was no fun at all. At last it was decided to
put the logic for install, build and test stages in separate windows-cmd scripts and install the necessary libs through Msys2 using pacman, which works quite well.

## Some Notes on TravisCI

The same as for AppVeyor applies, most code moved into scripts. In this case these are bash-scripts, which is fine.
That YAML-stuff is more weirdly in the way than helpful.
Perhaps when a bigger matrix of OSs and Dev-Envs will be used, that YAML gets handy.

## Code Coverage

To investigate:

* Best way to compute the code coverage
* how to use [`tarpaulin`](https://github.com/xd009642/tarpaulin)

## Security Safeguarding

To investigate:

* Hint: [`cargo-audit`](https://rustsec.org/)

## Source Styling

Ok, this is something of a personal matter, and preferred style/formatting is different for nearly everyone.  
The preferred source code formatting for Shardoverse currently can not be achieved in the stable Rust build,  
because some of the preferred formatting options are considered unstable in Rust fmt.

Unstable fmt options can be used in nightly Rust builds, but currently a change of Shardoverse to nightly is not intended.

## Performance Profiling

To investigate:

* How to do performance checking
* Hint: [`criterion.rs`](https://github.com/bheisler/criterion.rs)
* Example for a performance test: [`uhyve`](https://github.com/hermitcore/uhyve)
* probably needs a [`runner`](https://help.github.com/en/actions/hosting-your-own-runners/about-self-hosted-runners) on GitHub to give comparable results.

## License(s)

The game Shardoverse and all originally created parts of it (source code, texts, descriptions and such) are licensed under the MIT license, see the LICENSE.md file.

Parts which are used by the game, namely several assets like graphics, tiles, tilesets, textures, icons, cursors, sounds, music and fonts,
which are not originally created by direct contributors to Shardoverse, are provided under their own licenses.
Those parts are the property of their owners and the use within Shardoverse does not remove those licenses.

For each asset a file is provided in the directory of that asset, describing the origin of the data and, wherever possible, naming the copyright owner and the license.

There is absolutely no intention to infringe any copyrights, trademarks or patents of their respective owners.

If that should happen nevertheless, then that would be an accidental oversight and will be corrected.

## Assets

The following assets are considered to be used, if their creators/owners do not object:

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

    [`DungeonCrawlStoneSoupFull by many`](https://opengameart.org/content/dungeon-crawl-32x32-tiles), can also be found [`here`](https://github.com/crawl/tiles).

* ./assets graphics/2D-isometric/tiles

    [`rltiles-pack by Mitsuhiro Itakura (maintainer)`](https://opengameart.org/content/64x64-isometric-roguelike-tiles)

* ./assets/graphics/3D/meshes

* ./assets/graphics/3D/textures

* ./assets/gui

    [`RPG GUI construction kit v1.0_by_Lamoot`](https://opengameart.org/content/rpg-gui-construction-kit-v10)

* ./assets/images

* ./assets/videos

## Sources for Resources

Some more tilesets can be found here:  
2D Top-Down:  
    [`reddit roguelikedev wiki tilesets`](https://old.reddit.com/r/roguelikedev/wiki/tilesets)  
    [`opengameart: roguelike`](https://opengameart.org/art-search-advanced?field_art_tags_tid=roguelike)  
    [`itch.io: top-down+tileset`](https://itch.io/search?q=top-down+tileset)  

2.5D Isometric:  
    [`Reiner's Tilesets`](https://www.reinerstilesets.de/)

## Worth to digest, honorable to participate

* [`Zero-to-Game`](https://www.zerotoga.me/) - A website in blog-style, taking the reader on a journey from Zero (game programming experience) to Game. Well written and fun to read. In a way, a bit similar to what is intended here with Shardoverse.
* [`New Rustacean`](https://newrustacean.com/) - A well made podcast about learning Rust.
* [`Rust Sokoban`](https://sokoban.iolivia.me/) - A compact course of writing a sokoban game in Rust. Straightly delving into the game, getting quickly to the fun of it.  
* [`Crate publishing guidelines and tips`](https://blog.wnut.pw/2020/03/12/crate-publishing-guidelines-and-tips/#take-a-look-at-the-api-guidelines) - Many useful hints about improving code quality before publishing code.

## Rust game development community

* [`Rust gamedev news/`](https://rust-gamedev.github.io/) - A monthly newsletter
* [`Game Development in Rust server`](https://discord.gg/yNtPTb2) - The Discord server for Rust game development  
* [`r/rustgamedev`](http://reddit.com/r/rust_gamedev) - Subreddit of and for Rust game development  
* [`@rust_gamedev`](https://twitter.com/rust_gamedev) - Twitter channel  

### And a more global entry point is:

* [`Are we game yet?`](https://arewegameyet.rs/)

## Maintainer of Shardoverse

* [@clunion](https://github.com/clunion)

## Communicate

* [Discord-Server Shardoverse](https://discord.gg/PWAtRU)
