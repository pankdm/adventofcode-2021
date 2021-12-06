// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn solve(lines: &Vec<String>, days: usize) -> i64 {
    let mut nums = parse_ints(&lines[0], ",");
    let max = (*nums.iter().max().unwrap()).max(9);
    let mut counts = vec![0; max as usize];
    for value in nums.iter() {
        counts[*value as usize] += 1;
    }
    for day in 0..days {
        let mut next = vec![0; max as usize];
        for (index, value) in counts.iter().enumerate() {
            if index == 0 {
                next[8] += value;
                next[6] += value;
            } else {
                next[index - 1] += value;
            }
        }
        counts = next.clone();
    }
    counts.iter().sum()
}

pub fn part1(lines: &Vec<String>) -> i64 {
    solve(lines, 80)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    solve(lines, 256)
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day06/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 380758);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 1710623015163);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
