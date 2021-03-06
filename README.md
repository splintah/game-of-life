# Game of Life [![Build Status](https://travis-ci.org/splintah/game-of-life.svg?branch=master)](https://travis-ci.org/splintah/game-of-life)

A Game of Life emulation with support for several file formats.

![Window Demo](./window-demo.png)

## File support

This program supports the following file formats:

- [Life 1.06 file](http://www.conwaylife.com/wiki/Life_1.06)
- [Life 1.05 file](http://www.conwaylife.com/wiki/Life_1.05)
- [Run Length Encoded file](http://www.conwaylife.com/wiki/Run_Length_Encoded)
- [Plaintext file](http://www.conwaylife.com/wiki/Plaintext)

You can find a lot of patterns, and their files, on the [LifeWiki](http://www.conwaylife.com/wiki).
There is a list of patterns [here](http://www.conwaylife.com/wiki/Category:Patterns).
To use a file, pass it to the `--file` flag (see [below](#usage) for more flags).

## Installation

[Install Rust](https://www.rust-lang.org/en-US/). Then run the following in your terminal:

```bash
# Install from GitHub with cargo.
cargo install --git https://github.com/splintah/game-of-life
```

This will install `game-of-life` into `$HOME/.cargo/bin/`, which should be in your `PATH` variable when you have installed Rust via [`rustup`](https://rustup.rs).

## Installation from source

[Install Rust](https://www.rust-lang.org/en-US/). Then run the following in your terminal:

```bash
# Clone the code.
git clone https://github.com/splintah/game-of-life.git
# Change your directory to `game-of-life`.
cd game-of-life
# Install the binaries.
cargo install
```

## Usage

```text
game-of-life 0.3.0
Splinter Suidman (splintah) & Sybrand Aarnoutse (syberant)
game-of-life emulates John Conway's game of life.
Press Escape to exit, press C to toggle cursor capture and press Space or a mouse button to reinitialise grid.
Pressing ctrl and scrolling will zoom in on the cells.
A file can be dropped on the screen to load that file. When the grid is reinitialised, it will be reinitialised with
that file.

USAGE:
    game-of-life [FLAGS] [OPTIONS]

FLAGS:
        --border     Display the border.
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --background <background>    Change the background colour.
                                     The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white,
                                     000000 for black.
                                     Default: FFFFFF.
    -c, --cell-width <cell-width>    Change width of a cell (in pixels).
                                     Default: 10.
    -l, --chance <chance>            Chance for randomly initialising board.
                                     Example: with '--chance 50' passed, cells will have a 50% chance of living.
                                     Default: 15.
    -f, --file <file>                The file that contains the board.
                                     If this flag is passed, the board will be initialised with the board in the given
                                     file.
                                     Default: None.
        --foreground <foreground>    Change the foreground colour of the cells.
                                     The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white,
                                     000000 for black.
                                     Default: 000000.
        --fps <fps>                  The amount of updates and frames that should be performed per second.
                                     This is the maximum frames per second; that is, the actual fps could be less.
                                     A frame rate of 0 fps will result in no fps limit.
                                     Default: 24.
    -h, --height <height>            Change the height of the board (in cells).
                                     Default: 50.
    -w, --width <width>              Change the width of the board (in cells).
                                     Default: 50.

```
