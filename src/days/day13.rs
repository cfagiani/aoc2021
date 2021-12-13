use std::collections::HashSet;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day13 {}

impl Day for Day13 {
    fn part1(&self, input_root: &str) {
        let (dots, folds) = parse_input(input_root, "day13.txt");
        let after_fold = fold(&dots, &folds[0]);
        println!(
            "After one fold, there are {} visible dots",
            after_fold.len()
        );
    }

    fn part2(&self, input_root: &str) {
        let (dots, folds) = parse_input(input_root, "day13.txt");
        let mut after_fold = dots;
        for f in folds {
            after_fold = fold(&after_fold, &f);
        }
        println!("\n\nAfter all folds, output is\n\n");
        print_grid(40, 7, &after_fold);
    }
}

fn print_grid(x_dim: i32, y_dim: i32, grid: &HashSet<Point>) {
    for i in 0..y_dim {
        for j in 0..x_dim {
            let val = grid.get(&Point { x: j, y: i });
            if val.is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn fold(dots: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut result = HashSet::new();
    for dot in dots {
        if fold.dir == "y" && dot.y > fold.val {
            result.insert(Point {
                x: dot.x,
                y: fold.val - (dot.y - fold.val),
            });
        } else if fold.dir == "x" && dot.x > fold.val {
            result.insert(Point {
                x: fold.val - (dot.x - fold.val),
                y: dot.y,
            });
        } else {
            result.insert(Point { x: dot.x, y: dot.y });
        }
    }
    return result;
}

fn parse_input(input_root: &str, file_name: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    let lines = get_data_from_file(input_root, file_name, |s| s);
    for line in lines {
        if line.starts_with("fold") {
            let string_vec: Vec<String> = line
                .replace("fold along ", "")
                .split("=")
                .map(String::from)
                .collect(); //line.trim().split('-').map(String::from).collect();
                            //let (dir, loc) = line.replace("fold along ", "").split_once("=").unwrap();
            folds.push(Fold {
                dir: string_vec[0].clone(),
                val: string_vec[1].parse().unwrap(),
            });
        } else if line.trim().len() > 0 {
            dots.insert(point_from_string(&line));
        }
    }
    return (dots, folds);
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Fold {
    val: i32,
    dir: String,
}

/// Parses a Point from a string in the form "x,y"
fn point_from_string(input: &str) -> Point {
    let coords: Vec<i32> = input.split(",").map(|c| c.parse().unwrap()).collect();

    Point {
        x: *coords.get(0).unwrap(),
        y: *coords.get(1).unwrap(),
    }
}
