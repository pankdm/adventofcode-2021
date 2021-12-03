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
    let n = lines[0].len();
    let mut eps = 0;
    let mut gamma = 0;
    for i in 0..n {
        let mut n0 = 0;
        let mut n1 = 0;
        for line in lines {
            let s = to_v_char(line);
            if s[i] == '0' {
                n0 += 1;
            } else {
                n1 += 1;
            }
        }
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

pub fn to_bin(s: &Vec<char>) -> i64 {
    let mut res = 0;
    for i in 0..s.len() {
        res *= 2;
        if s[i] == '1' {
            res += 1;
        }
    }
    res
}


pub fn part2(lines: &Vec<String>) -> i64 {
    let n = lines[0].len();
    let mut nums: Vec<_> = lines.iter().map(|x| to_v_char(x)).collect();

    let mut v0 = nums.clone();
    for i in 0..n {
        let mut n0 = 0;
        let mut n1 = 0;
        for s in v0.iter() {
            if s[i] == '0' {
                n0 += 1;
            } else {
                n1 += 1;
            }
        }
        if v0.len() == 1 {
            break;
        }
        let bit = if n1 >= n0 { '1' } else { '0' };
        let next: Vec<_> = v0.iter().cloned().filter(|x| x[i] == bit).collect();
        v0 = next.clone();
        if v0.len() == 1 {
            break;
        }
    }
    let res0 = to_bin(&v0[0]);

    let mut v1 = nums.clone();
    for i in 0..n {
        let mut n0 = 0;
        let mut n1 = 0;
        for s in v1.iter() {
            if s[i] == '0' {
                n0 += 1;
            } else {
                n1 += 1;
            }
        }
        if v1.len() == 1 {
            break;
        }
        let bit = if n0 <= n1 { '0' } else { '1' };
        let next: Vec<_> = v1.iter().cloned().filter(|x| x[i] == bit).collect();
        v1 = next.clone();
        if v1.len() == 1 {
            break;
        }
    }
    let res1 = to_bin(&v1[0]);


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
