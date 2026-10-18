# Advent of Code 2019

This repository contains solutions to Advent of Code 2019.

## Adding a solution for a new day

Run this command to stub out a new day:

```sh
cargo xtask scaffold <number> # <number> is the day you'd like to add
```

This will create two files:
- `src/days/day<number>.rs`: write your solution here.
- `input/day<number>.txt>`: paste your input here.

## Run solutions

Run solutions with:

```sh
cargo run -- -s <day>
```
