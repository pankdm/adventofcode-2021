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

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day17/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 5050);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2223);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn simulate(mut vx: i64, mut vy: i64, v: &Vec<i64>) -> (bool, i64) {
    let mut x = 0;
    let mut y = 0;

    let mut max_y = y;
    loop {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
        max_y = max_y.max(y);
        if x >= v[0] && x <= v[1] && y >= v[2] && y <= v[3] {
            return (true, max_y);
        }
        if y < v[2] {
            return (false, max_y);
        }
    }
}

pub fn parse(lines: &Vec<String>) -> Vec<i64> {
    let s = lines[0].to_string();
    let re = Regex::new(r"x=([\-\d]+)..([\-\d]+), y=([\-\d]+)..([\-\d]+)").unwrap();
    let cap = re.captures(&s).unwrap();
    let v = (1..=4).map(|i| cap[i].to_i64()).cv();
    println!("{:?}", v);
    v
}

pub fn process(v: &Vec<i64>) -> (i64, usize) {
    let limit = 1000;
    let mut count = 0;

    let mut best = 0;
    for vx in 0..limit {
        for vy in -limit..limit {
            let (ok, my) = simulate(vx, vy, &v);
            if ok {
                best = best.max(my);
                count += 1;
            }
        }
    }
    (best, count)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let v = parse(lines);
    let (best, _) = process(&v);
    best
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let v = parse(lines);
    let (_, count) = process(&v);
    count as i64
}
