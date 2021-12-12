use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day7 {}

impl Day for Day7 {
    /// Calculates optimum horizontal position on which to align the crabs.
    fn part1(&self, input_root: &str) {
        let nested_crabs = get_data_from_file(input_root, "day7.txt", &parse);
        let mut crab_vec: Vec<&Crab> = nested_crabs.iter().flatten().collect();
        // sort by position ascending
        crab_vec.sort_by(|a, b| a.pos.cmp(&b.pos));
        // get middle position
        let destination = crab_vec.get(crab_vec.len() / 2).unwrap().pos;
        let crab_iter = crab_vec.iter();
        let mut fuel_used = 0;
        for crab in crab_iter {
            let diff = crab.pos - destination;
            fuel_used += diff.abs()
        }
        println!(
            "Total fuel used to move all crabs to {} is {}",
            destination, fuel_used
        );
    }

    fn part2(&self, input_root: &str) {
        let nested_crabs = get_data_from_file(input_root, "day7.txt", &parse);
        let mut crab_vec: Vec<&Crab> = nested_crabs.iter().flatten().collect();
        crab_vec.sort_by(|a, b| a.pos.cmp(&b.pos));
        // get average position
        let mut sum = 0.0;
        for crab in crab_vec.iter() {
            sum = sum + (crab.pos as f64);
        }
        let avg: f64 = sum / (crab_vec.len() as f64);
        // avg is likely between positions. get the cost of using the result of rounding down
        // and compare to result of rounding up and then use "cheaper" option
        let mut destination = avg as i32;
        let mut cost = get_fuel_to_destination(&crab_vec, &destination);
        let round_up_cost = get_fuel_to_destination(&crab_vec, &(destination + 1));
        if round_up_cost < cost {
            cost = round_up_cost;
            destination += 1;
        }

        println!(
            "Total fuel used to move all crabs to {} is {}",
            destination, cost
        );
    }
}

/// Computes fuel needed to move crab to destination. Each step toward the destination costs 1
/// more fuel than the previous.
fn get_fuel_to_destination(crabs: &Vec<&Crab>, destination: &i32) -> i32 {
    let mut fuel_used = 0;
    for crab in crabs.iter() {
        let diff = crab.pos - destination;
        let n = diff.abs();
        fuel_used += n * (n + 1) / 2;
    }
    return fuel_used;
}

/// Returns a vector of vector of crabs since all input is in 1 line. Flatten to use as a
/// vector of crabs.
fn parse(line: String) -> Vec<Crab> {
    return line
        .split(",")
        .map(|v| Crab {
            pos: v.parse().unwrap(),
        })
        .collect();
}

/// crab structure not really needed since we're just using ints. I was anticipating that part 2
/// would need more information tracked.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Crab {
    pos: i32,
}
