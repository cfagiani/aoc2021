use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day1 {}

impl Day for Day1 {
    /// Counts the number of times the integer in the input increases from the previous value
    fn part1(&self, input_root: &str) {
        let int_list = get_data_from_file(input_root, "day1.txt", |r| r.parse().unwrap());
        let mut last_val = -1;
        let mut increase_counter = 0;
        for cur_val in int_list {
            if last_val >= 0 && cur_val > last_val {
                increase_counter += 1;
            }
            last_val = cur_val;
        }
        println!("Depth increased {} times", increase_counter);
    }

    /// Counts the number of times a sliding window of values increases from previous
    fn part2(&self, input_root: &str) {
        let int_list = get_data_from_file(input_root, "day1.txt", |r| r.parse().unwrap());
        let window_size = 3;
        let mut last_window = -1;
        let mut increase_counter = 0;

        for i in 0..int_list.len() - window_size + 1 {
            let mut cur_window = 0;
            for j in i..i + window_size {
                cur_window += int_list.get(j).unwrap();
            }
            if last_window >= 0 && cur_window > last_window {
                increase_counter += 1;
            }
            last_window = cur_window;
        }
        println!("Depth increased {} times", increase_counter);
    }
}