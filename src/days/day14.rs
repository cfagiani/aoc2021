use std::collections::HashMap;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day14 {}

impl Day for Day14 {
    fn part1(&self, input_root: &str) {
        let (template, pair_insertions) = parse_input(input_root);

        //TODO: use string_with_capacity?
        let mut cur_val = template.clone();
        for _ in 0..10 {
            let mut next_val = String::from("");

            for i in 0..cur_val.len() - 1 {
                let mut str_iter = cur_val.chars();
                let mut pair = String::from(str_iter.nth(i).unwrap());
                pair.push(str_iter.next().unwrap());

                let mut pair_iter = pair.chars();
                let insert_char = pair_insertions.get(&pair).unwrap();
                if i == 0 {
                    next_val.push(pair_iter.next().unwrap());
                } else {
                    pair_iter.next();
                }
                next_val.push(*insert_char);
                next_val.push(pair_iter.next().unwrap());
            }
            cur_val = next_val;
        }
        let freqs = get_character_frequency(cur_val);
        let mut freq_vec: Vec<(&char, &i32)> = freqs.iter().collect();
        freq_vec.sort_by(|a, b| a.1.cmp(b.1));
        println!("After 10 steps, difference between most and least frequent element is {}", freq_vec[freq_vec.len() - 1].1 - freq_vec[0].1);
    }
}

fn get_character_frequency(input: String) -> HashMap<char, i32> {
    return input.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
}

fn parse_input(input_root: &str) -> (String, HashMap<String, char>) {
    let mut template = String::from("");
    let mut pair_insertions = HashMap::new();
    let lines = get_data_from_file(input_root, "day14.txt", |s| s);
    for line in lines {
        if line.trim().len() > 0 {
            if line.contains("->") {
                let (a, b) = line.split_once(" -> ").unwrap();
                pair_insertions.insert(String::from(a), b.chars().next().unwrap());
            } else {
                template = line;
            }
        }
    }
    return (String::from(template), pair_insertions);
}
