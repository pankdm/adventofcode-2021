// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn find_first(line: &String) -> i64 {
    let chars = to_v_char(line);
    let brackets = vec![
        ('(', ')', 3),
        ('[', ']', 57),
        ('{', '}', 1197),
        ('<', '>', 25137),
    ];
    let mut opening = HashSet::new();
    let mut scores = HashMap::new();

    let mut matching = HashMap::new();
    for (open, close, value) in brackets.iter() {
        matching.insert(close, open);
        opening.insert(open);
        scores.insert(close, value);
    }

    let mut stack = Vec::new();
    for ch in chars.iter() {
        if opening.contains(ch) {
            stack.push(ch);
        } else {
            let score = *scores[ch];
            if stack.is_empty() {
                return score;
            }
            let expected = stack.pop().unwrap();
            if matching[ch] != expected {
                return score;
            }
        }
    }
    0
}

pub fn find_incomplete(line: &String) -> i64 {
    let chars = to_v_char(line);
    let brackets = vec![
        ('(', ')', 1),
        ('[', ']', 2),
        ('{', '}', 3),
        ('<', '>', 4),
    ];
    let mut opening = HashSet::new();
    let mut scores = HashMap::new();

    let mut matching = HashMap::new();
    for (open, close, value) in brackets.iter() {
        matching.insert(close, open);
        opening.insert(open);
        scores.insert(open, value);
    }

    let mut stack = Vec::new();
    for ch in chars.iter() {
        if opening.contains(ch) {
            stack.push(ch);
        } else {
            if stack.is_empty() {
                return 0;
            }
            let expected = stack.pop().unwrap();
            if matching[ch] != expected {
                return 0;
            }
        }
    }

    let mut res = 0;
    for ch in stack.iter().rev() {
        res *= 5;
        res += scores[ch];
    }
    let out: String = stack.iter().rev().cloned().collect();
    println!("  {} -> {:?} {}", line, out, res);

    res
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line in lines {
        let score = find_first(line);
        res += score;
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut scores = Vec::new();
    for line in lines {
        let score = find_incomplete(line);
        if score > 0 {
            println!("{} -> {}", line, score);
            scores.push(score);
        }
    }
    scores.sort();
    let len = scores.len();
    println!("len = {}", len);
    scores[ len / 2]
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day10/in.txt")
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
