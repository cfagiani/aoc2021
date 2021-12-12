use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use crate::days::Day;
use crate::input::get_data_from_file;
use crate::input::parse_as_digits;

pub struct Day11 {}

impl Day for Day11 {
    fn part1(&self, input_root: &str) {
        let initial_levels = get_data_from_file(input_root, "day11.txt", &parse_as_digits);
        let mut octopuses = HashSet::new();
        let mut flash_count = 0;
        for i in 0..10 {
            for j in 0..10 {
                octopuses.insert(Octopus { loc: Loc { i, j }, level: initial_levels[i as usize][j as usize] });
            }
        }
        for _ in 0..100 {
            let (next_octs, count) = run_round(&octopuses);
            flash_count += count;
            octopuses = next_octs;
        }
        println!("After 100 steps, there were {} flashes", flash_count);
    }

    fn part2(&self, input_root: &str) {
        let initial_levels = get_data_from_file(input_root, "day11.txt", &parse_as_digits);
        let mut octopuses = HashSet::new();
        let mut flash_count = 0;
        for i in 0..10 {
            for j in 0..10 {
                octopuses.insert(Octopus { loc: Loc { i, j }, level: initial_levels[i as usize][j as usize] });
            }
        }
        let mut round = 0;
        while flash_count != 100 {
            let (next_octs, count) = run_round(&octopuses);
            flash_count = count;
            round += 1;
            octopuses = next_octs;
        }
        println!("All octopuses flash at once on round {}", round);
    }
}


fn run_round(octopuses: &HashSet<Octopus>) -> (HashSet<Octopus>, i32) {
    let mut to_process = VecDeque::new();
    let mut has_flashed = HashSet::new();
    let mut cur_round = HashSet::new();

    // increment level for every octopus by 1
    for octopus in octopuses {
        let mut updated_oct = octopus.clone();
        updated_oct.level += 1;
        if updated_oct.level > 9 {
            to_process.push_front(updated_oct);
            has_flashed.insert(updated_oct);
            updated_oct.level = 0;
            cur_round.insert(updated_oct);
        } else {
            cur_round.insert(updated_oct);
        }
    }

    let neighbor_offsets = [-1, 0, 1];

    while !to_process.is_empty() {
        let octopus = to_process.pop_front().unwrap();
        // update neighbors
        for i in neighbor_offsets.into_iter() {
            for j in neighbor_offsets.into_iter() {
                if i == 0 && j == 0 {
                    // don't update itself
                    continue;
                }
                let n_loc = Loc { i: octopus.loc.i + i, j: octopus.loc.j + j };

                let maybe_neighbor = cur_round.take(&Octopus { loc: n_loc, level: 0 });
                if maybe_neighbor.is_some() {
                    let mut neighbor = maybe_neighbor.unwrap();
                    if has_flashed.contains(&maybe_neighbor.unwrap()) {
                        cur_round.insert(neighbor);
                        continue;
                    }

                    neighbor.level += 1;
                    if neighbor.level > 9 {
                        to_process.push_front(neighbor);
                        has_flashed.insert(neighbor);
                        neighbor.level = 0;
                        cur_round.insert(neighbor);
                    } else {
                        cur_round.insert(neighbor);
                    }
                }
            }
        }
    }
    return (cur_round, has_flashed.len() as i32);
}

#[derive(Clone, Copy)]
struct Octopus {
    loc: Loc,
    level: u32,
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
struct Loc {
    i: i32,
    j: i32,
}

impl PartialEq for Octopus {
    fn eq(&self, other: &Self) -> bool {
        self.loc.eq(&other.loc)
    }
}

impl Eq for Octopus {}

impl Hash for Octopus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
    }
}

