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

    let (min, max) = crabs.iter().minmax().into_option().unwrap();
    (*min..=*max)
        .map(|pos| crabs.iter().map(|c| (c - pos).abs()).sum())
        .min()
        .unwrap()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let crabs = parse_ints(&lines[0], ",");

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    (min..=max)
        .map(|pos| {
            crabs
                .iter()
                .map(|c| {
                    let d = (c - pos).abs();
                    d * (d + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
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
        assert_eq!(part1(&lines), 340056);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 96592275);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
