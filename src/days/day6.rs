use std::collections::HashMap;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day6 {}

impl Day for Day6 {
    /// Calculates how many lanternfish would exist after 80 days
    fn part1(&self, input_root: &str) {
        println!(
            "After 80 days, there are {} fish",
            run_generations(input_root, 80)
        );
    }

    /// Calculates how many lanternfish would exist after 256 days
    fn part2(&self, input_root: &str) {
        println!(
            "After 256 days, there are {} fish",
            run_generations(input_root, 256)
        );
    }
}

fn run_generations(input_root: &str, generations: i32) -> u64 {
    let fish_string = get_data_from_file(input_root, "day6.txt", |s| s);

    let fish_ages: Vec<u8> = fish_string
        .get(0)
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    let mut age_map: HashMap<u8, u64> = HashMap::new();
    // create map of number of fish with age days left in their generation time
    for age in fish_ages {
        *age_map.entry(age).or_insert(0) += 1
    }

    for _ in 0..generations {
        let mut next_gen: HashMap<u8, u64> = HashMap::new();
        for age in age_map.clone().keys() {
            let count = *age_map.get(age).unwrap();
            if *age == 0 {
                // for all with age of 0, spawn new fish and reset the existing fish age
                *next_gen.entry(6).or_insert(0) += count;
                *next_gen.entry(8).or_insert(0) = count;
            } else {
                // for all with more time in generation, decrement
                *next_gen.entry(age - 1).or_insert(0) += count;
            }
        }
        age_map = next_gen.clone();
    }

    return age_map
        .into_values()
        .collect::<Vec<u64>>()
        .iter()
        .sum::<u64>();
}
