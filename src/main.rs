use std::env;
use std::process::exit;

use crate::days::run_day;

mod days;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part_num = "";
    if args.len() < 3 {
        println!("You must specify the input root and the day to run.\nUsage: aoc2021 <inputRoot> <dayNum> [partNum]\n");
        exit(1);
    }
    if args.len() > 3 {
        part_num = &args[3]
    }
    let input_root = &args[1];
    let day_num = &args[2];
    run_day(input_root, day_num, part_num)
}
