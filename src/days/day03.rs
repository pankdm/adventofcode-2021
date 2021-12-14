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
    let input = to_vv_char(lines);
    let n = input[0].len();
    let mut eps = 0;
    let mut gamma = 0;
    for i in 0..n {
        let n0 = input.iter().filter(|s| s[i] == '0').count();
        let n1 = input.iter().filter(|s| s[i] == '1').count();
        eps *= 2;
        gamma *= 2;
        if n1 > n0 {
            eps += 1;
        } else {
            gamma += 1;
        }
    }
    eps * gamma
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut nums = to_vv_char(lines);

    let n = nums[0].len();

    let mut v0 = nums.clone();
    for i in 0..n {
        if v0.len() == 1 {
            break;
        }
        let n0 = v0.iter().filter(|s| s[i] == '0').count();
        let n1 = v0.iter().filter(|s| s[i] == '1').count();
        let bit = if n1 >= n0 { '1' } else { '0' };
        v0.retain(|x| x[i] == bit);
    }
    let res0 = i64::from_str_radix(&v0[0].to_str(), 2).unwrap();

    let mut v1 = nums.clone();
    for i in 0..n {
        if v1.len() == 1 {
            break;
        }

        let n0 = v1.iter().filter(|s| s[i] == '0').count();
        let n1 = v1.iter().filter(|s| s[i] == '1').count();
        let bit = if n0 <= n1 { '0' } else { '1' };
        v1.retain(|x| x[i] == bit);
    }
    let res1 = i64::from_str_radix(&v1[0].to_str(), 2).unwrap();

    res0 * res1
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day03/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 1307354);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 482500);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
