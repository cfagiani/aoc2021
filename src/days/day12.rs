use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day12 {}

impl Day for Day12 {
    fn part1(&self, input_root: &str) {
        let cave_map = parse_caves(input_root);
        let path_count = count_all_paths(&cave_map, &mut HashSet::new(), "start");
        println!("There are {} paths", path_count);
    }
}


fn count_all_paths<'a>(cave_map: &HashMap<String, RefCell<Cave>>, visited: &mut HashSet<String>, current: &str) -> usize {
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
    let neighbors = &cave_map.get(current).unwrap().borrow().neighbors;
    for dest in neighbors {
        count += count_all_paths(cave_map, visited, dest.as_str());
    }
    visited.remove(current);
    return count;
}


fn parse_caves(input_root: &str) -> HashMap<String, RefCell<Cave>> {
    let lines = get_data_from_file(input_root, "day12.txt", |s| s);
    let mut cave_map = HashMap::new();
    for line in lines {
        let (cave_a, cave_b) = line.trim().split_once('-').unwrap();
        cave_map.entry(String::from(cave_a)).or_insert_with_key(|name| RefCell::new(Cave { name: name.to_string(), neighbors: HashSet::new() }));
        cave_map.entry(String::from(cave_b)).or_insert_with_key(|name| RefCell::new(Cave { name: name.to_string(), neighbors: HashSet::new() }));

        let side = cave_map.get(cave_a).unwrap();
        let side2 = cave_map.get(cave_b).unwrap();
        side.borrow_mut().neighbors.insert(String::from(cave_b));
        side2.borrow_mut().neighbors.insert(String::from(cave_a));
    }
    return cave_map;
}


#[derive(Clone)]
struct Cave {
    name: String,
    neighbors: HashSet<String>,
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Cave {}

impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}