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
    let mut v = Vec::new();
    for line in lines {
        let x = parse_i64(line);
        v.push(x);
    }
    let mut res = 0;
    for i in 1..v.len() {
        if v[i] > v[i - 1] {
            res += 1;
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut v = Vec::new();
    for line in lines {
        let x = parse_i64(line);
        v.push(x);
    }
    let mut res = 0;
    let mut w = Vec::new();
    for i in 2..v.len() {
        w.push(v[i] + v[i - 1] + v[i - 2]);
    }
    let mut res = 0;
    for i in 1..w.len() {
        if w[i] > w[i - 1] {
            res += 1;
        }
    }
    res
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day01/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 1393);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 1359);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
