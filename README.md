# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com/) puzzles for years
[2021](https://adventofcode.com/2021) and [2022](https://adventofcode.com/2022).

## Usage

Each problem has unit tests that can be run as usual with:

```sh
cargo test
```

Individual puzzle solutions can be obtained with e.g.:

```sh
cargo run -- 2022 01 a
```

As this downloads the puzzle input it will require environment variable 
`AOC_SESSION` to be set to the session cookie of the website.

Parameters are defined as follows:

```plain
USAGE:
    advent-of-code <YEAR> <DAY> <PART>

ARGS:
    <YEAR>    AoC event year
    <DAY>     Day of puzzle
    <PART>    Part of puzzle [possible values: a, b]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```
