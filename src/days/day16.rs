// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
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
    read_input(&format!("input/day16/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 974);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 180616437720);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

// 110100101111111000101000
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC

pub fn read_bits(pos: usize, bits: usize, d: &Vec<i64>) -> i64 {
    let mut res = 0;
    for i in pos..pos + bits {
        res *= 2;
        res += d[i];
    }
    // println!("  read from p = {}, {} bits -> {}", pos, bits, res);
    res
}

// returns new pos
pub fn read_literal(mut pos: usize, d: &Vec<i64>) -> (usize, i64) {
    let mut value = 0;
    loop {
        let run = d[pos];
        // println!("reading literal at pos = {}, run = {}", pos, run);
        pos += 1;
        let num = read_bits(pos, 4, d);
        pos += 4;

        value *= 16;
        value += num;

        if run == 0 {
            break;
        }
    }
    (pos, value)
}

#[derive(Default)]
struct Parser {
    pub versions: Vec<i64>,
}

impl Parser {
    pub fn parse_packet(&mut self, mut pos: usize, d: &Vec<i64>) -> (usize, i64) {
        // println!("parsing packet from pos={}, d = {:?}", pos, to_str(&d[pos..d.len()]));

        let v = read_bits(pos, 3, d);
        pos += 3;
        self.versions.push(v);

        let t = read_bits(pos, 3, d);
        pos += 3;
        // println!("type = {}", t);

        if t == 4 {
            let (pos, value) = read_literal(pos, d);
            return (pos, value);
        }
        let len_t = d[pos];
        let mut packets = Vec::new();

        pos += 1;
        if len_t == 0 {
            let length = read_bits(pos, 15, d);
            pos += 15;
            let end = pos + length as usize;
            while pos < end {
                let (pos_, value) = self.parse_packet(pos, d);
                pos = pos_;
                packets.push(value);
            }
        } else {
            let num = read_bits(pos, 11, d);
            pos += 11;
            for i in 0..num {
                let (pos_, value) = self.parse_packet(pos, d);
                pos = pos_;
                packets.push(value);
            }
        }

        let res = match t {
            0 => packets.iter().sum(),
            1 => packets.iter().product(),
            2 => *packets.iter().min().unwrap(),
            3 => *packets.iter().max().unwrap(),
            5 => {
                if packets[0] > packets[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if packets[0] < packets[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if packets[0] == packets[1] {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
        (pos, res)
    }
}

pub fn to_str(d: &[i64]) -> String {
    d.iter()
        .map(|x| (*x as u8 + '0' as u8) as char)
        .collect::<String>()
}

pub fn parse(lines: &Vec<String>) -> Vec<i64> {
    let mut d = Vec::new();
    for c in lines[0].chars() {
        let digit = c.to_digit(16).unwrap();
        let s = format!("{:04b}", digit);
        let digits = s.chars().map(|x| x.to_digit(2).unwrap() as i64).cv();
        d.extend(digits);
    }
    d
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let d = parse(lines);
    let mut p = Parser::default();
    let (pos, value) = p.parse_packet(0, &d);

    p.versions.iter().sum()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let d = parse(lines);
    let mut p = Parser::default();
    let (pos, value) = p.parse_packet(0, &d);

    value
}
