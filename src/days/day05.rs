// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vents = Vec::new();
    for line in lines {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let cap = re.captures(line).unwrap();
        let x1 = cap.get(1).unwrap().as_str().to_i64();
        let y1 = cap.get(2).unwrap().as_str().to_i64();
        let x2 = cap.get(3).unwrap().as_str().to_i64();
        let y2 = cap.get(4).unwrap().as_str().to_i64();
        vents.push((x1, y1, x2, y2));
    }
    let mut points = HashMap::new();
    for (x1, y1, x2, y2) in vents.iter().cloned() {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        if dx != 0 && dy != 0 {
            continue;
        }
        let steps = (x1 - x2).abs().max((y1 - y2).abs());
        for step in 0..=steps {
            let x = x1 + dx * step;
            let y = y1 + dy * step;
            *points.entry((x, y)).or_insert(0) += 1;
        }
    }
    points.values().filter(|x| **x >= 2).count() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vents = Vec::new();
    for line in lines {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let cap = re.captures(line).unwrap();
        let v = (1..=4).map(|i| cap[i].to_i64()).cv();
        vents.push((v[0], v[1], v[2], v[3]));
    }
    let mut points = HashMap::new();
    for (x1, y1, x2, y2) in vents.iter().cloned() {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let steps = (x1 - x2).abs().max((y1 - y2).abs());
        for step in 0..=steps {
            let x = x1 + dx * step;
            let y = y1 + dy * step;
            *points.entry((x, y)).or_insert(0) += 1;
        }
    }
    points.values().filter(|x| **x >= 2).count() as i64
}

pub fn read_main_input() -> Vec<String> {
    // read_input("input/day05/t1.txt")
    read_input("input/day05/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 7473);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 24164);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
