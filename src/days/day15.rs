use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::days::Day;
use crate::input::{get_data_from_file, parse_as_digits};

pub struct Day15 {}

impl Day for Day15 {
    fn part1(&self, input_root: &str) {
        let risk_map = build_map(input_root);
        println!("Lowest Risk Path: {}", risk_map.dijkstra(1).unwrap());
    }

    fn part2(&self, input_root: &str) {
        let risk_map = build_map(input_root);
        println!("Lowest Risk Path: {}", risk_map.dijkstra(5).unwrap());
    }
}

fn build_map(input_root: &str) -> RiskMap {
    let digits = get_data_from_file(input_root, "day15.txt", parse_as_digits);
    let mut risk_map = RiskMap::new(digits.len(), digits.get(0).unwrap().len());
    for i in 0..digits.len() {
        for j in 0..digits.get(i).unwrap().len() {
            risk_map.risk.insert(
                Vertex::new(i.try_into().unwrap(), j.try_into().unwrap()),
                *(digits.get(i).unwrap().get(j).unwrap()),
            );
        }
    }
    return risk_map;
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Vertex {
    i: i32,
    j: i32,
}

impl Vertex {
    pub fn new(i: i32, j: i32) -> Vertex {
        Vertex { i, j }
    }
}

struct RiskMap {
    risk: HashMap<Vertex, u32>,
    width: i32,
    height: i32,
}

impl RiskMap {
    pub fn new(h: usize, w: usize) -> RiskMap {
        RiskMap {
            risk: HashMap::new(),
            width: w.try_into().unwrap(),
            height: h.try_into().unwrap(),
        }
    }

    pub fn get_neighbor_risk(&self, loc: &Vertex, size_factor: i32) -> Vec<(Vertex, u32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| Vertex::new(loc.i + y, loc.j + x))
            .filter(|l| {
                l.j >= 0
                    && l.i >= 0
                    && l.i < (self.height * size_factor)
                    && l.j < (self.width * size_factor)
            })
            .map(|l| {
                let base_cost = *(self
                    .risk
                    .get(&Vertex::new(l.i % self.height, l.j % self.width))
                    .unwrap());
                let x_tile_num = (l.j / self.width) as u32;
                let y_tile_num = (l.i / self.height) as u32;
                // costs wrap around to 1 not 0
                let new_cost = (((base_cost + x_tile_num + y_tile_num) - 1) % 9) + 1;
                (l, new_cost)
            })
            .collect()
    }

    pub fn dijkstra(&self, size_factor: i32) -> Option<u32> {
        let source: Vertex = Vertex::new(0, 0);
        let mut dist_map: HashMap<Vertex, u32> = HashMap::new();
        dist_map.insert(source, 0);
        let mut heap = BinaryHeap::new();

        heap.push(HeapNode {
            cost: 0,
            loc: source,
        });

        while let Some(HeapNode { cost, loc }) = heap.pop() {
            // stop when we get to the destination (bottom-right grid position)
            if loc.j == (self.width * size_factor - 1) as i32
                && loc.i == (self.height * size_factor - 1) as i32
            {
                return Some(cost);
            }

            if let Some(old_cost) = dist_map.get(&loc) {
                // if we have already seen the node and got there via a shorter path, skip
                if cost > *old_cost {
                    continue;
                }
            }

            // get risk for each adjacent location
            for (point, risk) in self.get_neighbor_risk(&loc, size_factor) {
                let next = HeapNode {
                    cost: cost + risk,
                    loc: point,
                };

                if next.cost < *dist_map.get(&next.loc).unwrap_or(&(next.cost + 1)) {
                    heap.push(next);
                    dist_map.insert(next.loc, next.cost);
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    cost: u32,
    loc: Vertex,
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (self.loc.i + self.loc.j).cmp(&(other.loc.i + other.loc.j)))
            .then_with(|| {
                (other.loc.i - other.loc.j)
                    .abs()
                    .cmp(&(self.loc.i - self.loc.j).abs())
            })
    }
}
