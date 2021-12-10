use std::collections::HashMap;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day5 {}

impl Day for Day5 {
    /// Calculates the number of positions in a 2-D grid where 2 ore more "lines" overlap.
    /// Only considering horizontal and vertical lines
    fn part1(&self, input_root: &str) {
        println!("{} points crossed by more than one line", compute_crossed_lines(input_root, false));
    }

    /// Calculates the number of positions in a 2-D grid where 2 ore more "lines" overlap.
    /// Considers diagonals, horizontal, and vertical lines
    fn part2(&self, input_root: &str) {
        println!("{} points crossed by more than one line", compute_crossed_lines(input_root, true));
    }
}

/// Reads in the input and builds a list of point tuples, each representing the endpoints of "lines".
/// For each line, compute the positions it crosses and keep track of how many lines cross each
/// point in the coordinate plane. Returns the number of positions covered by 2 or more lines.
fn compute_crossed_lines(input_root: &str, include_diagonal: bool) -> i32 {
    let lines = get_data_from_file(input_root, "day5.txt", &parse_line);
    let mut covered_positions = HashMap::new();
    for line in lines {
        let x_step = if line.0.x == line.1.x { 0 } else if line.0.x < line.1.x { 1 } else { -1 };
        let y_step = if line.0.y == line.1.y { 0 } else if line.0.y < line.1.y { 1 } else { -1 };

        //if we're excluding diagonals, skip if not horizontal or vertical
        if x_step != 0 && y_step != 0 && !include_diagonal {
            continue;
        }


        let mut x = line.0.x;
        let mut y = line.0.y;
        while x != line.1.x || y != line.1.y {
            record_pos(x, y, &mut covered_positions);
            x += x_step;
            y += y_step;
        }
        // include the ending point
        record_pos(line.1.x, line.1.y, &mut covered_positions);
    }

    let mut count = 0;
    for entry in &covered_positions {
        if entry.1 > &1 {
            count += 1;
        }
    }

    return count;
}

/// Records that a line passed through x,y by incrementing the count in the positions map.
fn record_pos(x: i32, y: i32, positions: &mut HashMap<Point, i32>) {
    let p = Point { x, y };
    // use remove instead of get to avoid holding onto the borrow on the hashmap
    // we'll put the value back below
    let existing_val = positions.remove(&p);
    if existing_val.is_none() {
        positions.insert(p, 1);
    } else {
        positions.insert(p, existing_val.unwrap() + 1);
    }
}

/// Prints the grid of results. Used for debugging.
#[allow(dead_code)]
fn print_grid(x_dim: i32, y_dim: i32, covered_positions: &HashMap<Point, i32>) {
    for i in 0..y_dim {
        for j in 0..x_dim {
            let val = covered_positions.get(&Point { x: j, y: i });
            if val.is_some() {
                print!("{} ", val.unwrap());
            } else {
                print!(". ");
            }
        }
        print!("\n");
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}


/// Parses a line of input in the form "368,823 -> 302,823" into a tuple consisting of two Point
/// structs
fn parse_line(line: String) -> (Point, Point) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    (point_from_string(parts.get(0).unwrap()), point_from_string(parts.get(1).unwrap()))
}

/// Parses a Point from a string in the form "x,y"
fn point_from_string(input: &str) -> Point {
    let coords: Vec<i32> = input.split(",").map(|c| c.parse().unwrap()).collect();
    Point { x: *coords.get(0).unwrap(), y: *coords.get(1).unwrap() }
}