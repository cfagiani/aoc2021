use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day9 {}

impl Day for Day9 {
    /// calculates sum of the risk levels of each "low point"
    fn part1(&self, input_root: &str) {
        let height_map: Vec<Vec<u32>> = get_data_from_file(input_root, "day9.txt", &parse);
        let mut risk_sum = 0;
        for i in 0..height_map.len() {
            for j in 0..height_map.get(i).unwrap().len() {
                // for each point, check adjacent
                let val = height_map.get(i).unwrap().get(j).unwrap();
                if i > 0 && height_map.get(i - 1).unwrap().get(j).unwrap() <= val {
                    continue;
                }
                if i < height_map.len() - 1 && height_map.get(i + 1).unwrap().get(j).unwrap() <= val {
                    continue;
                }
                if j > 0 && height_map.get(i).unwrap().get(j - 1).unwrap() <= val {
                    continue;
                }
                if j < height_map.get(i).unwrap().len() - 1 && height_map.get(i).unwrap().get(j + 1).unwrap() <= val {
                    continue;
                }
                // if we're here, it's a low point

                risk_sum += val + 1;
            }
        }
        println!("Sum of risk levels is {}", risk_sum);
    }
}

fn parse(line: String) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}