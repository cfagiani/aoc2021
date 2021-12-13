use std::collections::{HashMap, HashSet};

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day12 {}

impl Day for Day12 {
    fn part1(&self, input_root: &str) {
        let cave_map = parse_caves(input_root);
        let path_count = count_all_paths(&cave_map, &mut HashSet::new(), "start");
        println!("There are {} paths", path_count);
    }

    fn part2(&self, input_root: &str) {
        let cave_map = parse_caves(input_root);
        let lowercase_caves = cave_map
            .keys()
            .filter(|&v| v != "start" && v != "end" && v.chars().next().unwrap().is_lowercase())
            .map(|x| x.as_str())
            .map(Some);
        let mut paths = HashSet::new();
        let mut visited = HashSet::new();
        for cave in lowercase_caves {
            get_paths_with_revisit(&cave_map, &mut visited, &["start"], &mut paths, cave);
        }
        let ans = paths.len();
        println!("{}", ans);
    }
}

fn get_paths_with_revisit<'a>(
    cave_map: &'a HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    current: &[&'a str],
    paths: &mut HashSet<Vec<&'a str>>,
    mut allow_twice: Option<&'a str>,
) {
    let origin = current[current.len() - 1];

    if origin == "end" {
        paths.insert(current.to_vec());
        return;
    }

    if visited.contains(origin) {
        return;
    }

    if origin.chars().next().unwrap().is_lowercase() {
        if let Some(v) = allow_twice {
            if v == origin {
                allow_twice = None;
            } else {
                visited.insert(origin.to_string());
            }
        } else {
            visited.insert(origin.to_string());
        }
    }

    if let Some(neighbors) = cave_map.get(origin) {
        for destination in neighbors {
            get_paths_with_revisit(
                cave_map,
                visited,
                &[current, &[destination]].concat(),
                paths,
                allow_twice,
            );
        }
    }

    visited.remove(origin);
}

fn count_all_paths<'a>(
    cave_map: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    current: &str,
) -> usize {
    if visited.contains(current) {
        return 0;
    }
    if current == "end" {
        return 1;
    }

    if current.chars().next().unwrap().is_lowercase() {
        visited.insert(String::from(current));
    }

    let mut count = 0;
    let neighbors = cave_map.get(current).unwrap();
    for dest in neighbors {
        count += count_all_paths(cave_map, visited, dest);
    }
    visited.remove(current);
    return count;
}

fn parse_caves(input_root: &str) -> HashMap<String, HashSet<String>> {
    let lines = get_data_from_file(input_root, "day12.txt", |s| s);
    let mut cave_map = HashMap::new();
    for line in lines {
        let string_vec: Vec<String> = line.trim().split('-').map(String::from).collect();

        cave_map
            .entry(string_vec[0].clone())
            .or_insert_with(HashSet::new)
            .insert(string_vec[1].clone());
        cave_map
            .entry(string_vec[1].clone())
            .or_insert_with(HashSet::new)
            .insert(string_vec[0].clone());
    }
    return cave_map;
}
