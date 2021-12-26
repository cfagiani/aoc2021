use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day18 {}

impl Day for Day18 {
    fn part1(&self, input_root: &str) {
        let numbers = get_data_from_file(input_root, "day18.txt", parse_snailfish_numbers);
        let mut sums = add_all(&numbers);
        println!("Magnitude after sum: {}", get_magnitude(&mut sums));
    }

    fn part2(&self, input_root: &str) {
        let numbers = get_data_from_file(input_root, "day18.txt", parse_snailfish_numbers);
        println!("Largest magnitude: {}", find_largest_magnitude(&numbers));
    }
}

fn find_largest_magnitude(nums: &Vec<Vec<SailfishNum>>) -> i64 {
    let mut largest_mag = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let mut sum = do_sum(nums.get(i), nums.get(j));
            reduce(&mut sum);
            let cur_mag = get_magnitude(&mut sum);
            if cur_mag > largest_mag {
                largest_mag = cur_mag;
            }
        }
    }
    return largest_mag;
}

fn add_all(nums: &Vec<Vec<SailfishNum>>) -> Vec<SailfishNum> {
    let mut prev_sum = do_sum(nums.get(0), nums.get(1));
    reduce(&mut prev_sum);
    let mut i = 2;
    while i < nums.len() {
        prev_sum = do_sum(Option::Some(&prev_sum), nums.get(i));
        reduce(&mut prev_sum);
        i += 1;
    }
    return prev_sum;
}

fn reduce<'a>(nums: &'a mut Vec<SailfishNum>) {
    let mut action_occurred = true;
    while action_occurred {
        action_occurred = explode(nums);
        if action_occurred {
            continue;
        }
        action_occurred = split(nums);
    }
}

fn parse_snailfish_numbers(line: String) -> Vec<SailfishNum> {
    let mut stack = Vec::new();
    let mut depth = -1;
    for character in line.chars() {
        match character {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            _ => {
                let val = character.to_digit(10).unwrap() as i64;
                stack.push(SailfishNum { depth: depth, value: val });
            }
        }
    }
    return stack;
}

#[derive(Copy, Clone)]
struct SailfishNum {
    depth: i32,
    value: i64,
}

fn do_sum<'a>(a: Option<&'a Vec<SailfishNum>>, b: Option<&'a Vec<SailfishNum>>) -> Vec<SailfishNum> {
    if !a.is_some() {
        return b.unwrap().clone();
    }
    let mut result = a.unwrap().clone();
    result.append(&mut (b.unwrap().clone()));
    for mut v in result.iter_mut() {
        v.depth += 1;
    }
    return result;
}

fn explode(nums: &mut Vec<SailfishNum>) -> bool {
    for i in 0..nums.len() {
        if nums[i].depth != 4 {
            continue;
        }

        let new_depth = nums[i].depth - 1;
        if i != 0 {
            nums[i - 1].value += nums[i].value;
        }
        if i < nums.len() - 2 {
            nums[i + 2].value += nums[i + 1].value;
        }
        nums.remove(i + 1);
        nums.remove(i);
        nums.insert(i, SailfishNum { depth: new_depth, value: 0 });
        return true;
    }

    return false;
}

fn split(nums: &mut Vec<SailfishNum>) -> bool {
    for i in 0..nums.len() {
        if nums[i].value > 9 {
            let target = nums[i];
            let split_depth = target.depth + 1;
            let new_val = (target.value as f64) / 2.0;
            nums.remove(i);
            nums.insert(i, SailfishNum { value: new_val.floor() as i64, depth: split_depth });
            nums.insert(i + 1, SailfishNum { value: new_val.ceil() as i64, depth: split_depth });
            return true;
        }
    }
    return false;
}

fn get_magnitude(nums: &mut Vec<SailfishNum>) -> i64 {
    let mut depth = 3;
    while depth >= 0 {
        let mut action_occurred = true;
        while action_occurred {
            action_occurred = false;
            for j in 0..nums.len() - 1 {
                let left = nums[j];
                let right = nums[j + 1];
                if left.depth != depth {
                    continue;
                }
                nums.remove(j + 1);
                nums.remove(j);
                nums.insert(j, SailfishNum { depth: depth - 1, value: 3 * left.value + 2 * right.value });
                action_occurred = true;
                break;
            }
        }
        depth -= 1;
    }
    return nums[0].value;
}