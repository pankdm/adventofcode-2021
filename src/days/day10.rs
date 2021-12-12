// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn process_line(line: &String, unmatched: &mut Vec<i64>, incomplete: &mut Vec<i64>) {
    let chars = line.to_vec();
    let data = vec![
        ('(', ')', 3, 1),
        ('[', ']', 57, 2),
        ('{', '}', 1197, 3),
        ('<', '>', 25137, 4),
    ];
    let mut opening = HashSet::new();
    let mut scores1 = HashMap::new();
    let mut scores2 = HashMap::new();
    let mut matching = HashMap::new();

    for (open, close, v1, v2) in data.iter() {
        matching.insert(close, open);
        opening.insert(open);
        scores1.insert(close, v1);
        scores2.insert(open, v2);
    }

    let mut stack = Vec::new();
    for ch in chars.iter() {
        if opening.contains(ch) {
            stack.push(ch);
        } else {
            let score = *scores1[ch];
            if stack.is_empty() {
                unmatched.push(score);
                return;
            }
            let expected = stack.pop().unwrap();
            if matching[ch] != expected {
                unmatched.push(score);
                return;
            }
        }
    }

    let mut res = 0;
    for ch in stack.iter().rev() {
        res *= 5;
        res += scores2[ch];
    }
    incomplete.push(res);
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut unmatched = Vec::new();
    let mut incomplete = Vec::new();
    for line in lines {
        process_line(line, &mut unmatched, &mut incomplete);
    }
    unmatched.iter().sum()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut unmatched = Vec::new();
    let mut incomplete = Vec::new();
    for line in lines {
        process_line(line, &mut unmatched, &mut incomplete);
    }
    incomplete.sort();
    let len = incomplete.len();
    incomplete[len / 2]
}

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day10/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 290691);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2768166558);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
