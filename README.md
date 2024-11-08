# Advent of Code

My personal solutions for [Advent of Code](https://adventofcode.com/), written in [Rust](https://www.rust-lang.org/).

## Prequesites

For most of the endpoints you need to be authenticated. Authentication is accomplished via OAtuh/Session Cookies.
As mentioned on the site your session cookies is valid for about a month.
So simply extract your session token from a browser of your choice and set it as env var `SESSION_TOKEN`.

## Usage

```shell
cargo run -p <package> -- <day>
```
