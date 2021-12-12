use std::array::IntoIter;
use std::collections::{HashMap, VecDeque};

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day10 {}

impl Day for Day10 {
    /// calculates syntax error score for corrupted lines
    fn part1(&self, input_root: &str) {
        let lines = get_data_from_file(input_root, "day10.txt", |s| s);

        let char_pairs = HashMap::<_, _>::from_iter(IntoIter::new([
            (')', '('),
            (']', '['),
            ('>', '<'),
            ('}', '{'),
        ]));
        let corrupted_char: Vec<char> = lines
            .iter()
            .map(|v| validate_lines(v, &char_pairs))
            .filter(|v| !v.0)
            .map(|v| v.2.unwrap())
            .collect();

        let mut score = 0;
        for c in corrupted_char {
            match c {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => println!("Invalid illegal char {}", c),
            }
        }
        println!("Syntax error score {}", score);
    }

    /// Computes score for autocomplete
    fn part2(&self, input_root: &str) {
        let lines = get_data_from_file(input_root, "day10.txt", |s| s);
        let char_pairs = HashMap::<_, _>::from_iter(IntoIter::new([
            (')', '('),
            (']', '['),
            ('>', '<'),
            ('}', '{'),
        ]));
        let incomplete_lines: Vec<VecDeque<char>> = lines
            .iter()
            .map(|v| validate_lines(v, &char_pairs))
            .filter(|v| v.0)
            .map(|v| v.1)
            .collect();
        let char_scores =
            HashMap::<_, _>::from_iter(IntoIter::new([('(', 1), ('[', 2), ('{', 3), ('<', 4)]));
        let mut all_scores: Vec<u64> = Vec::new();
        for mut line in incomplete_lines {
            let mut score: u64 = 0;
            while !line.is_empty() {
                score *= 5;
                score += char_scores.get(&line.pop_front().unwrap()).unwrap();
            }
            all_scores.push(score);
        }
        all_scores.sort();

        println!(
            "Autocomplete score is {}",
            all_scores.get(all_scores.len() / 2).unwrap()
        );
    }
}

/// Validates a line of input by checking to see that the brackets contain no invalid characters
/// i.e that the brackets are balanced so far (they may be incomplete but the line is still valid as
/// long as we haven't encountered the wrong type of bracket). This method returns a tuple consisting
/// of a bool indicating if the line is valid (true means valid), the remaining stack of unclosed
/// brackets, and an optional character set tothe first illegal character (if any).

fn validate_lines(
    line: &String,
    char_pairs: &HashMap<char, char>,
) -> (bool, VecDeque<char>, Option<char>) {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        if c == ']' || c == ')' || c == '>' || c == '}' {
            let top = stack.pop_front();
            if top.is_none() {
                return (false, stack, Some(c));
            }
            if top.unwrap() != *char_pairs.get(&c).unwrap() {
                return (false, stack, Some(c));
            }
        } else {
            stack.push_front(c);
        }
    }
    return (true, stack, Option::None);
}
