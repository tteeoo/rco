# rco: rusty configurer

A niche tool for managing configuration files, specifically geared towards minimal linux installations.
It lets you give your configuration files a nickname, then quickly edit them by running:

`rco nickname`


I mainly made this to learn rust, although it is genuinely useful, atleast to me.

This is a (better) remake of my project <a href="https://github.com/tteeoo/rco">comma</a> which is writen in C.

# Installation

## AUR

If you are on Arch Linux, rco can be installed via the AUR package <a href="https://aur.archlinux.org/packages/rco/">rco</a>

Also, the PKGBUILD for this project is located at <a href="https://github.com/tteeoo/rco/pkg/arch/PKGBUILD">pkg/arch/PKGBUILD</a>


## Build from source with cargo

### Prerequisites

* <b>cargo</b>, rust's package manager and frontend to the compiler

### To install:

```
$ cargo install --git https://github.com/tteeoo/rco
```

You may need to add `~/.cargo/bin/` to your `PATH` variable in your shell's rc file; this is so you can run `rco` in your shell without specifying the binary's filepath.


To uninstall rco when installed by building from source (not via AUR) run the <a href="https://github.com/tteeoo/rco/scripts/uninstall.sh">scripts/uninstall.sh</a> script.

# Usage

As already mentioned, you can quickly edit your files by running:

`rco nickname`

Edit as root with:

`rco -s nickname`



You can load configuration files for editing like this:

`rco -l nickname /path/to/file "brief description"`



In turn, you can unload, and stop tracking them, like this:

`rco -u nickname`



You can also list all your tracked configuration files by running without cany arguments:

`rco`



# Configuration

rco's configuration file is located at `~/.config/rco/config.csv`

The default file can be located in the repo at <a href="https://github.com/tteeoo/rco/defaults/config.csv">defaults/config.csv</a>

This is where you can change:

- The editor files are opened in

- Wether or not files are list in alternating colors

- The shell your editor is opened with; this will not need to be changed unless you do not have `sh` at `/bin/sh` for some reason


# Roadmap

## Future features include:

* Custom object file location
* Exporting files to a central directory
* Importing files from a git repository, and copying them into your system

See the projects tab at the top for a more verbose view of what is being worked on.


# Project layout

## Directories

* **src/**: all of the source code
* **defaults/**: where the default config and object files are, as well as a non-markdown README
* **scripts/**: various useful scripts
* **pkg/**: package files for different distros

## Branches

* **master**: the latest stable release
* **testing**: for development use only. master will be merged with this once a release is ready


# License

All files are licensced under the MIT License

This can be found in the repo at <a href="https://github.com/tteeoo/rco/LICENSE-MIT">LICENSE-MIT</a>

<hr>

<em>https://github.com/tteeoo/rco</em>
