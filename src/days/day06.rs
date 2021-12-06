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
    let mut nums = parse_ints(&lines[0], ",");
    println!("len = {}", nums.len());
    for day in 0..80 {
        let mut next = nums.clone();
        for i in 0..nums.len() {
            if nums[i] == 0 {
                next[i] = 6;
                next.push(8);
            } else {
                next[i] -= 1;
            }
        }
        nums = next;
    }
    nums.len() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut nums = parse_ints(&lines[0], ",");
    let mut counts = HashMap::new();
    for value in nums.iter() {
        *counts.entry(*value).or_insert(0) += 1;  
    }
    for day in 0..256 {
        let mut next = HashMap::new();
        for (k, v) in counts.iter() {
            if *k == 0 {
                *next.entry(8).or_insert(0) += v;
                *next.entry(6).or_insert(0) += v;
            } else {
                *next.entry(*k - 1).or_insert(0) += v;
            }
        }
        counts = next.clone();
    }
    counts.values().sum()
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
