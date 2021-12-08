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
        println!("The digits with unique number of segments appear {} times in the output", digit_count);
    }
}

fn parse_line(line: String) -> SignalPattern {
    let parts: Vec<&str> = line.split("|").collect();
    return SignalPattern {
        digits: parts.get(0).unwrap().split(" ").map(|d| Digit { segments: String::from(d) }).collect(),
        result: parts.get(1).unwrap().split(" ").map(|d| Digit { segments: String::from(d) }).collect(),
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


#[derive(Clone)]
struct Digit {
    segments: String,
}