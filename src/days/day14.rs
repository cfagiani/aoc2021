use std::collections::HashMap;

use itertools::Itertools;

use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day14 {}

impl Day for Day14 {
    fn part1(&self, input_root: &str) {
        let (template, pair_insertions) = parse_input(input_root);
        let chain = (0..10).fold(template, |chain, _| insert_char(chain, &pair_insertions));
        let counts = get_char_freq(&chain);
        let (max, min) = get_max_min(&counts);
        println!("After 10 rounds, difference between most frequent and least frequent characters is: {}", (max - min) / 2);
    }

    fn part2(&self, input_root: &str) {
        let (template, pair_insertions) = parse_input(input_root);
        let chain = (0..40).fold(template, |chain, _| insert_char(chain, &pair_insertions));
        let counts = get_char_freq(&chain);
        let (max, min) = get_max_min(&counts);
        println!("After 40 rounds, difference between most frequent and least frequent characters is: {}", (max - min) / 2);
    }
}

/// Counts the frequency of each character in the output and returns them as a hashmap of character
/// frequencies.
fn get_char_freq(chain: &HashMap<(u8, u8), usize>) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();
    for ((c1, c2), count) in chain {
        *counts.entry(*c1).or_insert(0) += count;
        *counts.entry(*c2).or_insert(0) += count;
    }
    // don't count our boundary marker
    counts.remove(&b'z');
    return counts;
}

/// Gets a tuple containing the max and min frequencies from the count map.
fn get_max_min(freqs: &HashMap<u8, usize>) -> (usize, usize) {
    let mut freq_vec: Vec<(&u8, &usize)> = freqs.iter().collect();
    freq_vec.sort_by(|a, b| a.1.cmp(b.1));
    return (*freq_vec[freq_vec.len() - 1].1, *freq_vec[0].1);
}

/// parses input into 2 maps: a map containing the current counts of 2 letter pairs and a map of polymer insertion rules
fn parse_input(input_root: &str) -> (HashMap<(u8, u8), usize>, HashMap<(u8, u8), u8>) {
    let lines = get_data_from_file(input_root, "day14.txt", |s| s);
    let pair_insertions: HashMap<(u8, u8), u8> = lines[2..]
        .iter()
        .filter_map(|line| line.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(ls, rs)| ((ls.as_bytes()[0], ls.as_bytes()[1]), rs.as_bytes()[0]))
        .collect();
    let template = format!("z{}z", &lines[0]).bytes().tuple_windows().counts();
    return (template, pair_insertions);
}

fn insert_char(
    cur_chain: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new_chain: HashMap<(u8, u8), usize> = HashMap::new();
    for ((c1, c2), count) in cur_chain {
        if let Some(&to_insert) = rules.get(&(c1, c2)) {
            // if there was a rule matching this pair, then we need to insert the product P of
            // applying the rule AB-> P. Insert 2 tuples: one for AP and another for PB.
            *new_chain.entry((c1, to_insert)).or_insert(0) += count;
            *new_chain.entry((to_insert, c2)).or_insert(0) += count;
        } else {
            // no rule matched so just count the pair of characters without any insertion
            *new_chain.entry((c1, c2)).or_insert(0) += count;
        }
    }
    return new_chain;
}
