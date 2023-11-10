# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com/) puzzles for years
[2020](https://adventofcode.com/2020), [2021](https://adventofcode.com/2021),
[2022](https://adventofcode.com/2022) and [2023](https://adventofcode.com/2023).

## Usage

Each problem has unit tests that can be run individually with e.g.:

```sh
cargo test y2022::d01
```

Individual puzzle solutions can be obtained with e.g.:

```sh
cargo run -r -- 2022 01 a
```

As this downloads the puzzle input it will require environment variable
`AOC_SESSION` to be set to the session cookie of the website.

Parameters are defined as follows:

```plain
Usage: advent-of-code <YEAR> <DAY> <PART>

Arguments:
  <YEAR>  AoC event year
  <DAY>   Day of puzzle
  <PART>  Part of puzzle [possible values: a, b]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
