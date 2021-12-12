use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

use crate::days::Day;
use crate::input::get_data_from_file;
use crate::input::parse_as_digits;

pub struct Day9 {}

impl Day for Day9 {
    /// calculates sum of the risk levels of each "low point"
    fn part1(&self, input_root: &str) {
        let height_map: Vec<Vec<u32>> =
            get_data_from_file(input_root, "day9.txt", &parse_as_digits);
        let low_points = find_low_points(&height_map);
        println!(
            "Sum of risk levels is {}",
            low_points.iter().map(|p| p.val + 1).sum::<u32>()
        );
    }

    /// finds product of the sizes of the 3 largest "Basins".
    fn part2(&self, input_root: &str) {
        let height_map: Vec<Vec<u32>> =
            get_data_from_file(input_root, "day9.txt", &parse_as_digits);
        let mut low_points = find_low_points(&height_map);
        // each low point belongs to its own basin. Just need to calculate the size of the basin
        for point in low_points.iter_mut() {
            point.basin_size = get_basin_size(&height_map, point);
        }
        low_points.sort();
        let mut product = 1;
        for i in low_points.len() - 3..low_points.len() {
            product *= low_points.get(i).unwrap().basin_size;
        }
        println!("Product of the 3 largest basins is {}", product);
    }
}

/// Returns a vector of LowPoint structs. Low Points are locations in the height_map that are lower
/// than all their adjacent locations (only considering immediate, non-diagonal neighbors).
fn find_low_points(height_map: &Vec<Vec<u32>>) -> Vec<LowPoint> {
    let mut low_points = Vec::new();
    for i in 0..height_map.len() {
        for j in 0..height_map.get(i).unwrap().len() {
            if check_neighbors(&height_map, i, j, |neighbor, val| neighbor <= val) {
                low_points.push(LowPoint {
                    loc: Point { i, j },
                    val: *height_map.get(i).unwrap().get(j).unwrap(),
                    basin_size: 0,
                })
            }
        }
    }
    return low_points;
}

fn get_basin_size(height_map: &Vec<Vec<u32>>, low_point: &LowPoint) -> usize {
    let mut horizon = VecDeque::from([low_point.loc.clone()]);
    let mut visited: HashSet<Point> = HashSet::new();

    while !horizon.is_empty() {
        // for each position, add any adjacent locs that are not 9 to horizon
        let loc = horizon.pop_front().unwrap();
        visited.insert(loc.clone());
        let candidate_locs =
            get_candidates(&loc, height_map.len(), height_map.get(loc.i).unwrap().len());
        let to_add = candidate_locs.difference(&mut visited);
        for point in to_add {
            // if the candidate is a 9 don't add it
            if *height_map.get(point.i).unwrap().get(point.j).unwrap() < 9 {
                horizon.push_back(point.clone());
            }
        }
    }

    return visited.len();
}

fn get_candidates(loc: &Point, i_bound: usize, j_bound: usize) -> HashSet<Point> {
    let mut candidates = HashSet::new();
    if loc.i > 0 {
        candidates.insert(Point {
            i: loc.i - 1,
            j: loc.j,
        });
    }
    if loc.i < i_bound - 1 {
        candidates.insert(Point {
            i: loc.i + 1,
            j: loc.j,
        });
    }
    if loc.j > 0 {
        candidates.insert(Point {
            i: loc.i,
            j: loc.j - 1,
        });
    }
    if loc.j < j_bound - 1 {
        candidates.insert(Point {
            i: loc.i,
            j: loc.j + 1,
        });
    }
    return candidates;
}

/// Applies the condition passed in to the neighbors of the position indicated by i and j. If
/// the condition is true for any of the neighbors, this method returns false, otherwise it returns
/// true.
fn check_neighbors<F>(height_map: &Vec<Vec<u32>>, i: usize, j: usize, condition: F) -> bool
where
    F: Fn(&u32, &u32) -> bool,
{
    // for each point, check adjacent
    let val = height_map.get(i).unwrap().get(j).unwrap();
    if i > 0 && condition(height_map.get(i - 1).unwrap().get(j).unwrap(), val) {
        return false;
    }
    if i < height_map.len() - 1 && height_map.get(i + 1).unwrap().get(j).unwrap() <= val {
        return false;
    }
    if j > 0 && height_map.get(i).unwrap().get(j - 1).unwrap() <= val {
        return false;
    }
    if j < height_map.get(i).unwrap().len() - 1
        && height_map.get(i).unwrap().get(j + 1).unwrap() <= val
    {
        return false;
    }
    return true;
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct LowPoint {
    loc: Point,
    val: u32,
    basin_size: usize,
}

impl PartialOrd for LowPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LowPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.basin_size.cmp(&other.basin_size)
    }
}
