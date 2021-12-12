use std::collections::HashSet;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day8 {}

impl Day for Day8 {
    /// Counts the number of times digits with a unique pattern of 7-segment representations appear
    /// in the output
    fn part1(&self, input_root: &str) {
        let signals = get_data_from_file(input_root, "day8.txt", &parse_line);

        let mut digit_count = 0;
        for s in signals {
            digit_count += s.result.iter().filter(&unique_patterns_filter).count();
        }
        println!(
            "The digits with unique number of segments appear {} times in the output",
            digit_count
        );
    }

    fn part2(&self, input_root: &str) {
        let signals = get_data_from_file(input_root, "day8.txt", &parse_line);
        let mut sum = 0;
        for signal in signals {
            let digits = decode_signal(signal.digits);
            // now we can represent the digits of each of the output
            sum += decode_output(digits, signal.result);
        }
        println!("Sum of outputs is {}", sum);
    }
}

fn decode_output(digits: [Digit; 10], output: Vec<Digit>) -> u32 {
    let mut out_val = 0;

    for i in 0..4 {
        let dig_idx = digits
            .iter()
            .enumerate()
            .find(|(_, v)| output.get(i).unwrap().eq(v))
            .unwrap();
        out_val += 10_u32.pow(3 - i as u32) * (dig_idx.0 as u32)
    }

    return out_val;
}

fn decode_signal(signal: Vec<Digit>) -> [Digit; 10] {
    let mut digits: Vec<Digit> = signal.clone();
    // handle the "easy" digits that we can identify just by length
    let one = find_and_remove(&mut digits, |d| d.segments.len() == 2);
    let four = find_and_remove(&mut digits, |d| d.segments.len() == 4);
    let seven = find_and_remove(&mut digits, |d| d.segments.len() == 3);
    let eight = find_and_remove(&mut digits, |d| d.segments.len() == 7);

    // 3 has 5 segments and MUST have all of one's segments

    let three = find_and_remove(&mut digits, |d| {
        d.segments.len() == 5 && d.segments.is_superset(&one.segments)
    });

    // 6 has 6 segments and is the only 6 segment digit that doesn't have ALL of 1's segments
    let six = find_and_remove(&mut digits, |d| {
        d.segments.len() == 6 && !d.segments.is_superset(&one.segments)
    });

    // all segments in 8 that are not in 6
    let top_right = eight.segments.difference(&six.segments).next().unwrap();

    // 2 has 5 segments (at this point, only remaining 5 segment numbers are 2 and 3) and it's the only one
    // that has the top_right segment

    let two = find_and_remove(&mut digits, |d| {
        d.segments.len() == 5 && d.segments.contains(top_right)
    });

    // 5 is the only 5 segment number now
    let five = find_and_remove(&mut digits, |d| d.segments.len() == 5);

    // bottom_left is what you get when you look at segments in 2 that aren't in 3
    let bottom_left = two.segments.difference(&three.segments).next().unwrap();

    // 9 and 0 are left. 9 lacks bottom_left
    let nine = find_and_remove(&mut digits, |d| !d.segments.contains(bottom_left));

    // zero is last digit standing
    let zero = digits.remove(0);

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

fn parse_line(line: String) -> SignalPattern {
    let parts: Vec<&str> = line.split(" | ").collect();
    return SignalPattern {
        digits: parts
            .get(0)
            .unwrap()
            .split(" ")
            .map(|d| Digit::new(d))
            .collect(),
        result: parts
            .get(1)
            .unwrap()
            .split(" ")
            .map(|d| Digit::new(d))
            .collect(),
    };
}

fn unique_patterns_filter(digit: &&Digit) -> bool {
    let seg_len = digit.segments.len();
    // segments in 1 (2), 4 (4), 7 (3), and 8 (7)
    return seg_len >= 2 && seg_len <= 4 || seg_len == 7;
}

struct SignalPattern {
    digits: Vec<Digit>,
    result: Vec<Digit>,
}

#[derive(Clone, PartialEq)]
struct Digit {
    segments: HashSet<char>,
}

impl Digit {
    fn new(s: &str) -> Self {
        Digit {
            segments: s.chars().collect(),
        }
    }
}

/// removes an item from the vector and returns it. This will blow up if it's not there.
fn find_and_remove<F, I>(v: &mut Vec<I>, find: F) -> I
where
    F: FnMut(&I) -> bool,
{
    let idx = v.iter().position(find).unwrap();
    v.remove(idx)
}
