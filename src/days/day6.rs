use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day6 {}

impl Day for Day6 {
    /// Calculates how many lanternfish would exist after 80 days
    fn part1(&self, input_root: &str) {
        println!("After 80 days, there are {} fish", run_generations(input_root, 80));
    }

    /// Calculates how many lanternfish would exist after 256 days
    fn part2(&self, input_root: &str) {
        println!("After 256 days, there are {} fish", run_generations(input_root, 256));
    }
}

fn run_generations(input_root: &str, generations: i32) -> usize {
    let fish_string = get_data_from_file(input_root, "test.txt", |s| s);
    let mut fish_ages: Vec<i32> = fish_string.get(0).unwrap().split(",").map(|c| c.parse().unwrap()).collect();
    for _ in 0..generations {
        let mut new_fish: Vec<i32> = Vec::new();
        for fish in fish_ages {
            if fish == 0 {
                new_fish.push(6);
                new_fish.push(8);
            } else {
                new_fish.push(fish - 1);
            }
        }
        fish_ages = new_fish
    }
    return fish_ages.len();
}