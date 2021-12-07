// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let crabs = parse_ints(&lines[0], ",");

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut best_cost = -1;
    for pos in min..=max {
        let mut cost = 0;
        for c in crabs.iter() {
            cost += (c - pos).abs();
        }
        if best_cost == -1 || cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let crabs = parse_ints(&lines[0], ",");

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut best_cost = -1;
    for pos in min..=max {
        let mut cost = 0;
        for c in crabs.iter() {
            let d = (c - pos).abs();
            cost += d * (d + 1) / 2;
        }
        if best_cost == -1 || cost < best_cost {
            best_cost = cost;
        }
    }
    best_cost
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day07/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
