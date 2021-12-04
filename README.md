This project contains my solutions to the 2021 edition of the [Advent of Code](https://adventofcode.com/2021)

This year, I'm trying to solve the puzzles in Rust as a means to explore the language. This is my first foray into Rust
so please excuse any non-idiomatic code.

The main function for this project will use the command-line arguments to determine what it should run. At a
minimum, you must specify which day to run. You can then optionally specify a part (1 or 2) to run. If no part is
supplied, both parts will be run.

Usage:
aoc2021 <inputRoot> <dayNum> [partNum]
  where inputRoot is the path to the directory containing input files, dayNum is an integer in the range (1,25) and partNum is a integer in the range (1,2).
  Input files are assuumed to be named "dayX.txt" where X is the day number.

