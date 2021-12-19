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
    read_input(&format!("input/day18/{}", file))
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

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    Open,
    Close,
    Value(i64),
}

impl Token {
    pub fn is_value(&self) -> bool {
        if let Token::Value(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_value(&self) -> i64 {
        if let Token::Value(x) = self {
            return *x;
        }
        unreachable!();       
    }

    pub fn as_value_mut(&mut self) -> &mut i64 {
        if let Token::Value(x) = self {
            return x;
        }
        unreachable!();       
    }

    pub fn to_str(&self) -> String {
        match &self {
            Token::Open => "[".to_string(),
            Token::Close => "]".to_string(),
            Token::Value(x) => format!("{}", x),
        }
    }
}

// Example:
// Here is the process of finding the reduced result of [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]:

// after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
// after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]

// reducing [ [ [ [ [ 4 3 ] 4 ] 4 ] [ 7 [ [ 8 4 ] 9 ] ] ] [ 1 1 ] ]
// reducing [ [ [ [ 0 7 ] 4 ] [ 7 [ [ 8 4 ] 9 ] ] ] [ 1 1 ] ]
// reducing [ [ [ [ 0 7 ] 4 ] [ 15 [ 0 13 ] ] ] [ 1 1 ] ]

type Number = Vec<Token>;

pub fn parse(line: &str) -> Number {
    let mut res = Vec::new();
    for ch in line.chars() {
        let tok = match ch {
            '[' => Token::Open,
            ']' => Token::Close,
            ',' => continue,
            _ => Token::Value(ch.to_digit(10).unwrap() as i64),
        };
        res.push(tok);
    }
    res
}

pub fn add(now: &mut Number, other: &Number) {
    now.insert(0, Token::Open);
    now.extend_from_slice(other);
    now.push(Token::Close);
}

pub fn try_explode(now: &mut Number) -> bool {
    let mut depth = 0;
    for index in 0..now.len() {
        let tok = now[index];
        match tok {
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            Token::Value(a) => {
                if depth >= 5 {
                    let b = now[index + 1].as_value();
                    assert!(now[index - 1] == Token::Open);
                    assert!(now[index + 2] == Token::Close);
                    if let Some(next) = now[index + 3..].iter_mut().find(|x| x.is_value()) {
                        *next.as_value_mut() += b;
                    }
                    if let Some(prev) = now[..index].iter_mut().rev().find(|x| x.is_value()) {
                        *prev.as_value_mut() += a;
                    }
                    // replace the pair with 0
                    let replace = [Token::Value(0)];
                    now.splice(index - 1..=(index + 2), replace);
                    return true;
                }
            }
        }
    }
    false
}

pub fn try_split(now: &mut Number) -> bool {
    for index in 0..now.len() {
        let tok = now[index];
        if let Token::Value(x) = tok {
            if x >= 10 {
                let a = x / 2;
                let b = (x + 1) / 2;
                let replace = [Token::Open, Token::Value(a), Token::Value(b), Token::Close];
                now.splice(index..=index, replace);
                return true;
            }
        }
    }
    false
}

pub fn reduce(now: &mut Number) -> bool {
    // explode
    if try_explode(now) {
        return true;
    }
    if try_split(now) {
        return true;
    }
    false
}



pub fn mag(v: &Number, pos: usize) -> (i64, usize) {
    match v[pos] {
        Token::Open => {
            let (left, left_end) = mag(v, pos + 1);
            let (right, right_end) = mag(v, left_end + 1);
            assert!(v[right_end + 1] == Token::Close);
            (3 * left + 2 * right, right_end + 1)
        }
        Token::Value(x) => {
            (x, pos)
        }
        _ => unreachable!()
    }
}

pub fn to_str(now: &Number) -> String {
    now.iter().map(|x| x.to_str()).cv().join(" ")
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut nums = Vec::new();
    for line in lines {
        nums.push(parse(line));
    }

    let mut now = nums[0].clone();
    let mut cnt = 0;
    for next in nums[1..].iter() {
        println!("adding {}/{}", cnt, nums.len());
        cnt += 1;

        add(&mut now, next);
        // for step in 0..10 {
        loop {
            // println!("   reducing {}", to_str(&now));
            if !reduce(&mut now) {
                break;
            }
        }
    }

    mag(&now, 0).0
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut nums = Vec::new();
    for line in lines {
        nums.push(parse(line));
    }

    let mut res = Vec::new();
    for ia in 0..nums.len() {
        for ib in 0..nums.len() {
            if ia == ib {
                continue;
            }
            let mut a = nums[ia].clone();
            let b = nums[ib].clone();
            add(&mut a, &b);
            loop {
                // println!("   reducing {}", to_str(&now));
                if !reduce(&mut a) {
                    break;
                }
            }
            let m = mag(&mut a, 0).0;
            res.push(m);
        }
    }
    *res.iter().max().unwrap()
}
