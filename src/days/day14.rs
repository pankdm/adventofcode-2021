// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day14/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 3408);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 3724343376942);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn solve(lines: &Vec<String>, steps: usize) -> i64 {
    let mut input = lines[0].to_vec();

    let mut rules = HashMap::new();
    for line in lines[2..lines.len()].iter() {
        let p = split_string(line, " -> ");
        let c = p[1].to_vec()[0];
        rules.insert(p[0].clone(), c);
    }

    let mut pairs: HashMap<String, usize> = input.windows(2).map(|x| x.iter().collect()).counts();

    for step in 0..steps {
        let mut next = HashMap::new();
        for (part, count) in pairs.iter() {
            if rules.contains_key(part) {
                let c = rules[part];
                let v1: String = vec![part.to_vec()[0], c].iter().collect();
                let v2: String = vec![c, part.to_vec()[1]].iter().collect();

                *next.entry(v1).or_insert(0) += count;
                *next.entry(v2).or_insert(0) += count;
            } else {
                *next.entry(part.to_string()).or_insert(0) += count;
            }
        }
        pairs = next;
    }

    let mut counts = HashMap::new();
    // first letter
    counts.insert(input.to_vec()[0], 1);
    for (s, value) in pairs.iter() {
        let c = s.to_vec()[1];
        *counts.entry(c).or_insert(0) += value;
    }

    let mut x = counts.values().cv();
    x.sort();
    (x[x.len() - 1] - x[0]) as i64
}

pub fn part1(lines: &Vec<String>) -> i64 {
    solve(lines, 10)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    solve(lines, 40)
}
