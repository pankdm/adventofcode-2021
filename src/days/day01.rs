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
    let v = lines.iter().map(|l| l.to_i64());
    v.tuple_windows().filter(|(a, b)| a < b).count() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let v = lines.iter().map(|l| l.to_i64());
    let sums = v.tuple_windows().map(|(a, b, c)| a + b + c);
    sums.tuple_windows().filter(|(a, b)| a < b).count() as i64
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
