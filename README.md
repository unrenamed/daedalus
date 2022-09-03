[![](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![](https://github.com/unrenamed/daedalus/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/unrenamed/daedalus/actions/workflows/rust.yml)

# Daedalus

Daedalus is a maze generator for the terminal written in Rust. It can create mazes using different algorithms and show the process step-by-step. Simple as that.

![Demo](https://raw.githubusercontent.com/unrenamed/daedalus/main/.github/images/demo.gif)

### Reference

Daedalus was an inventor, craftsman, architect and artist in Greek mythology, who had two sons, Icarus and Iapyx.

He is best known as the creator of the Labyrinth, a huge maze located under the court of King Minos of Crete, where the Minotaur, a half-man half-bull creature dwelt. According to the myth, the king of Athens was forced to pay tribute to King Minos by sending seven young men and seven young women each year to Crete, in order to be sacrificed to the Minotaur.

Source: https://www.greekmythology.com/Myths/Mortals/Daedalus/daedalus.html

### Installation

1. Download the latest [binary](https://github.com/unrenamed/daedalus/releases) for your OS.
1. `cd` to the file you just downloaded and install it

### CLI
The binary is named `daedalus`.

Running `daedalus` with no arguments will bring up the UI with default values. Use `daedalus --help` to learn more about available arguments.

`--width` (`-w`) and `--height` (`-h`) arguments allow you to customize the grid width and height.

To run the app in 60 FPS mode, use `--tick-rate 1` or `-t 1`.

Example:

```bash
daedalus -w 20 -h 15 -t 1
```
