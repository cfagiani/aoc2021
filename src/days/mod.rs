use crate::days::day1::Day1;
use crate::days::day10::Day10;
use crate::days::day11::Day11;
use crate::days::day12::Day12;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;
use crate::days::day9::Day9;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

/// Trait for the solution to each Day of the Advent of Code.
/// Each day implements two methods, part1 and part2. The methods will read input (if any) and then
/// output
pub trait Day {
    fn part1(&self, _input_root: &str) {
        println!("part not yet implemented")
    }

    fn part2(&self, _input_root: &str) {
        println!("part not yet implemented")
    }
}

struct UnknownDay {}

impl Day for UnknownDay {
    fn part1(&self, _input_root: &str) {
        println!("Day not yet implemented")
    }

    fn part2(&self, _input_root: &str) {
        println!("Day not yet implemented")
    }
}

/// Runs the solution for a single day based on the value passed in.
pub fn run_day(input_root: &str, day_num: &str, part_num: &str) {
    let day: &dyn Day = match day_num {
        "1" => &Day1 {},
        "2" => &Day2 {},
        "3" => &Day3 {},
        "4" => &Day4 {},
        "5" => &Day5 {},
        "6" => &Day6 {},
        "7" => &Day7 {},
        "8" => &Day8 {},
        "9" => &Day9 {},
        "10" => &Day10 {},
        "11" => &Day11 {},
        "12" => &Day12 {},
        _ => &UnknownDay {},
    };
    match part_num {
        "" => {
            run_part_one(day, input_root);
            run_part_two(day, input_root);
        }
        "1" => run_part_one(day, input_root),
        "2" => run_part_two(day, input_root),
        _ => {
            println!("If specified, part option can only be 1 or 2");
        }
    }
}

fn run_part_one(day: &dyn Day, input_root: &str) {
    println!("\nPART 1\n");
    day.part1(input_root);
}

fn run_part_two(day: &dyn Day, input_root: &str) {
    println!("\n\nPART 2\n");
    day.part2(input_root);
}
