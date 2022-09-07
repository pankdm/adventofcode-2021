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
    read_input(&format!("input/day24/{}", file))
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

struct State {
    vars: Vec<i32>,
}

impl State {
    fn new() -> Self {
        State { vars: vec![0; 4] }
    }

    fn register(&self, a: &str) -> usize {
        let c = a.to_vec()[0];
        (c as u8 - 'w' as u8) as usize
    }

    fn value(&self, b: &str) -> i32 {
        match b {
            "w" | "x" | "y" | "z" => {
                let index = self.register(b);
                self.vars[index]
            }
            _ => b.to_i64() as i32,
        }
    }

    fn apply(&mut self, s: &str) {
        let ss = split_string(s, " ");
        assert_eq!(ss.len(), 3);
        let index = self.register(&ss[1]);
        let value = self.value(&ss[2]);
        match ss[0].as_str() {
            "add" => self.vars[index] += value,
            "mul" => self.vars[index] *= value,
            "div" => self.vars[index] /= value,
            "mod" => self.vars[index] %= value,
            "eql" => self.vars[index] = if self.vars[index] == value { 1 } else { 0 },
            _ => {
                panic!("unknown op: {}", ss[0])
            }
        }
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    part_helper(lines, true)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    part_helper(lines, false)
}

pub fn part_helper(lines: &Vec<String>, largest: bool) -> i64 {
    assert_eq!(5 / 2, 2);
    assert_eq!(-5 / 2, -2);

    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for s in lines.iter() {
        if s.starts_with("inp ") {
            assert_eq!(s, "inp w");
            if !block.is_empty() {
                blocks.push(block.clone());
                block.clear();
            }
        } else {
            block.push(s.clone());
        }
    }
    blocks.push(block);

    let mut var_rows = Vec::new();

    let rows = blocks[0].len();
    for row in 0..rows {
        // println!("row = {}", r);
        let all_equal = blocks.iter().all(|b| b[row] == blocks[0][row]);
        if all_equal {
            // println!("({}) {}", row, blocks[0][row]);
        } else {
            {
                let parts = split_string(&blocks[0][row], " ");
                // print!("({}) {} {} --> ", row, parts[0], parts[1]);
            }
            let mut var_row = Vec::new();
            for i in 0..blocks.len() {
                let parts = split_string(&blocks[i][row], " ");
                // print!(" {:>3}", parts[2].to_i64());
                var_row.push(parts[2].to_i64());
            }
            var_rows.push(var_row);
            // println!("");
        }
        // println!("");
    }
    let a = var_rows[0].clone();
    let b = var_rows[1].clone();
    let c = var_rows[2].clone();

    // println!("");
    // println!("total inputs = {}", blocks.len());
    // println!("a = {:?}", a);
    // println!("b = {:?}", b);
    // println!("c = {:?}", c);

    let mut z = Term::zero();
    let mut equations = Vec::new();
    for i in 0..blocks.len() {
        let mut x = z.mod26();
        x.delta += b[i];
        // println!("");
        // println!("i = {}", i);
        // println!("x = {}", x.format());

        assert!(a[i] == 26 || a[i] == 1);
        if a[i] == 26 {
            z = z.divide26();
            // println!("  divided by 26");
        }
        if x.delta + 1 > 9 || x.delta + 9 < 1 {
            // println!("  solution is not possible for x = {}", x.format());
            let oldz = z.clone();
            z = Term {
                w: i as i32,
                delta: c[i],
                mult: Some(Box::new(oldz)),
            };
        } else {
            // println!("  solution is possible for w{} = {}", i, x.format());
            equations.push((x, i));
        }
        // println!("z = {}", z.format());
    }

    let mut res = vec![0; blocks.len()];

    equations.sort_by_key(|x| x.0.w);
    for (term, w) in equations.iter() {
        // println!("  {} = w{}", term.format(), w);
        // for finding largest using reverse order of digits;
        let digits_order = if largest {
            (1..=9).rev().cv()
        } else {
            (1..=9).cv()
        };
        for digit in digits_order {
            let value = digit + term.delta;
            if 1 <= value && value <= 9 {
                // println!("    best digit = {}", digit);
                res[term.w as usize] = digit;
                res[*w] = value;
                break;
            }
        }
    }
    let final_number = res.iter().map(|x| format!("{}", x)).cv().join("");
    println!("{}", final_number);
    final_number.to_i64()
}

// x = z % 26 + b
// z /= a
// if x == w {
// } else {
//     z *= 26
//     z += (w + c)
// }

#[derive(Clone)]
struct Term {
    w: i32, // -1 if no w terms yet
    delta: i64,
    mult: Option<Box<Term>>,
}

impl Term {
    fn zero() -> Term {
        Term {
            w: -1,
            delta: 0,
            mult: None,
        }
    }

    fn is_zero(&self) -> bool {
        self.w == -1 && self.delta == 0
    }

    fn divide26(&self) -> Term {
        assert!(self.delta > 0);
        assert!(self.delta + 9 < 26, "delta is too big {}", self.delta);

        if self.mult.is_none() {
            Term::zero()
        } else {
            *self.mult.clone().unwrap()
        }
    }

    fn mod26(&self) -> Term {
        Term {
            w: self.w,
            delta: self.delta,
            mult: None,
        }
    }

    fn format(&self) -> String {
        let mut parts = Vec::new();
        if self.w != -1 {
            parts.push(format!("w{}", self.w));
        }
        parts.push(format!("{}", self.delta));
        if self.mult.is_some() {
            let mult_term = self.mult.as_ref().unwrap();
            if !mult_term.is_zero() {
                let other = mult_term.format();
                parts.push(format!("26*({})", other));
            }
        }
        parts.join(" + ")
    }
}
