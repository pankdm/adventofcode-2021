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
    let mut pos = 0;
    let mut depth = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let dir = to_v_char(&parts[0]);
        let dist = parts[1].to_i64();
        match dir[0] {
            'f' => pos += dist,
            'd' => depth += dist,
            'u' => depth -= dist,
            _ => {}
        }
    }
    pos * depth
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let dir = to_v_char(&parts[0]);
        let dist = parts[1].to_i64();
        match dir[0] {
            'f' => {
                pos += dist;
                depth += aim * dist;
            }
            'd' => aim += dist,
            'u' => aim -= dist,
            _ => {}
        }
    }
    pos * depth
}
pub fn read_main_input() -> Vec<String> {
    read_input("input/day02/in.txt")
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
