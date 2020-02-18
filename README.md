# rco: rusty configurer

A niche tool for managing configuration files, specifically geared towards minimal linux installations.

I mainly made this to learn rust, although it is genuinely useful, atleast to me.

This is a (better) remake of my project <a href="https://github.com/tteeoo/rco">comma</a> which is writen in C.

As of now, rco only runs on Linux, and has been tested (working) on Arch and Ubuntu. It might work on macOS, I'm not sure; it definetly won't work on Windows though.

# Installation

## Build from source (only current installation method)

### Prerequisites

* <b>cargo</b>, rust's package manager and frontend to the compiler
* <b>rustc</b>, the actual rust compiler, which you most likely already have if you have cargo

### Commands

```
$ git clone https://github.com/tteeoo/rco
$ cd rco
$ ./configure.sh
# ./install.sh
```
Note: `$ ./configure.sh` is optional

Also, you <i>must</i> be in the repo's root directory when running these scripts

<br>

To uninstall, run 

```
# ./uninstall.sh
```

# Usage

rco is a basic program and has four main functions:

*Listing* - rco will list out all tracked files when it has no arguments

`rco`

*Loading* - rco will start tracking a given file, giving it a description and a nickname to easily edit it with

`rco -l nickname /path/to/file "brief description"`


*Editing* - rco will attempt to open the file with the given nickname in your editor of choice, specified in its config.csv file

`rco nickname`


*Unloading* - rco will stop tracking a file with a specified nickname

`rco -u nickname`


# Configuration

rco's configuration file is located at `~/.config/rco/config.csv`

The default file can be located in the repo at `/defaults/config.csv`

This is where you can change:

- The editor files are opened in

- Wether or not files are list in alternating colors

- The shell your editor is opened with; this will not need to be changed unless you do not have `sh` at `/bin/sh` for some reason


# License

All files are licensced under the MIT License

This can be found in the repo at /LICENSE
