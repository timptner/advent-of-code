# Advent of Code

My personal solutions for [Advent of Code](https://adventofcode.com/) (AoC), written in [Rust](https://www.rust-lang.org/).

## Authentication

To use any of the manager commands you need to be authenticated.
Authentication is accomplished via OAtuh session cookies, which are valid for about a month.
Login to AoC with a browser of your choice and extract the session cookie afterwards.
Set the value of your session cookie as env var `SESSION_TOKEN`.
(You can use a `.env` file in your working directory to persist your token.)

## Usage

Retrieve personal input.

```shell
cargo run input <year> <day>
```

Solve a puzzle and print the solution:

```shell
cargo run solve <year> <day>
```

You can test all solutions against the default input/answer combinations.

```shell
cargo test
```

Submit answer to puzzle platform.

```shell
cargo run answer <year> <day> <level> <value>
```
