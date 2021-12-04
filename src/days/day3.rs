use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day3 {}

impl Day for Day3 {
    /// Computes the power consumption by taking the product of the gamma and epsilon values
    /// which are computed by counting the most and least common bit in each position (respectively)
    fn part1(&self, input_root: &str) {
        let bit_strings = get_data_from_file(input_root, "day3.txt", |r| r);
        let mut gamma = String::from("");
        let mut epsilon = String::from("");
        let string_len = bit_strings.get(0).unwrap().chars().count();
        for i in 0..string_len {
            let most_common = get_most_common_val_at_position(&bit_strings, i);
            gamma.push(most_common);
            match most_common {
                '1' => epsilon.push('0'),
                '0' => epsilon.push('1'),
                _ => ()
            }
        }
        let gamma_int = isize::from_str_radix(gamma.as_str(), 2).unwrap();
        let epsilon_int = isize::from_str_radix(epsilon.as_str(), 2).unwrap();
        println!("Energy use {}", gamma_int * epsilon_int);
    }

    /// Calculates life support rating
    fn part2(&self, input_root: &str) {
        let bit_strings = get_data_from_file(input_root, "day3.txt", |r| r);
        let oxygen_rating = find_rating(&bit_strings, |r: char| r);
        let co2_rating = find_rating(&bit_strings, |r: char| {
            return if r == '1' {
                '0'
            } else {
                '1'
            };
        });
        println!("Life support rating {}", oxygen_rating * co2_rating);
    }
}

fn find_rating(bit_strings: &Vec<String>, condition: fn(char) -> char) -> isize {
    let string_len = bit_strings.get(0).unwrap().chars().count();
    let mut candidates = bit_strings.clone();
    for i in 0..string_len {
        let most_common = get_most_common_val_at_position(&candidates, i);
        let filter_char = condition(most_common);
        candidates = filter_list(&candidates, i, filter_char);
        if candidates.len() == 1 {
            break;
        }
    }
    return isize::from_str_radix(candidates.get(0).unwrap().as_str(), 2).unwrap();
}

/// Returns the most common value among all candidates at position pos within the string.
fn get_most_common_val_at_position(candidates: &Vec<String>, pos: usize) -> char {
    let mut one_count = 0;
    let mut zero_count = 0;
    for bit_string in candidates {
        if bit_string.chars().nth(pos).unwrap() == '1' {
            one_count += 1
        } else {
            zero_count += 1;
        }
    }

    return if one_count >= zero_count {
        '1'
    } else {
        '0'
    };
}

/// Filters a list of Strings based to return a new Vector containing just those strings that have
/// a character matching val in position pos.
fn filter_list(candidates: &Vec<String>, pos: usize, val: char) -> Vec<String> {
    let mut filtered = Vec::new();
    for candidate in candidates {
        if candidate.chars().nth(pos).unwrap() == val {
            filtered.push(String::from(candidate));
        }
    }
    return filtered;
}



