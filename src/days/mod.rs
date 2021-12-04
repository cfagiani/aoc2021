use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;

mod day1;
mod day2;
mod day3;
mod day4;


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


pub fn run_day(input_root: &str, day_num: &str, part_num: &str) {
    let day: &dyn Day = match day_num {
        "1" => &Day1 {},
        "2" => &Day2 {},
        "3" => &Day3 {},
        "4" => &Day4 {},
        _ => &UnknownDay {}
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