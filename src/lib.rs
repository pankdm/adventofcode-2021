pub mod day_template;
pub mod days;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};

pub fn read_input(filename: &str) -> Vec<String> {
    let full_name = format!("{}", filename);
    let msg = format!("File {} not found", full_name);
    let file = File::open(full_name).expect(msg.as_str());
    let reader = BufReader::new(file);
    let mut res = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        res.push(line.to_string());
    }
    return res;
}

pub fn to_vv_char(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.to_vec()).collect()
}

pub fn parse_i64(s: &str) -> i64 {
    match s.parse::<i64>() {
        Err(e) => {
            assert!(false, "Error parsing '{}': {}", &s, e);
            unreachable!();
        }
        Ok(value) => {
            return value;
        }
    }
}

pub fn parse_ints(s: &str, pattern: &str) -> Vec<i64> {
    let parts = split_string(s, pattern);
    let mut nums = Vec::new();
    // dbg!(parts.clone());
    for part in parts.iter() {
        nums.push(part.to_i64());
    }
    nums
}

pub fn split_string(s: &str, pattern: &str) -> Vec<String> {
    let mut res = Vec::new();
    for part in s.split(pattern) {
        res.push(part.to_string());
    }
    return res;
}

pub fn to_lines(s: &str) -> Vec<String> {
    split_string(&s.trim().to_string(), "\n")
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector2d {
    pub x: i64,
    pub y: i64,
}

impl Vector2d {
    pub fn new(x: i64, y: i64) -> Self {
        Vector2d { x, y }
    }
    pub fn rotate_left(&self) -> Vector2d {
        Vector2d {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn rotate_right(&self) -> Vector2d {
        Vector2d {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Add for Vector2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i64> for Vector2d {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Vector2d> for i64 {
    type Output = Vector2d;
    fn mul(self, other: Vector2d) -> Vector2d {
        other * self
    }
}

type Grid = Vec<Vec<char>>;

pub trait GridExt {
    fn get(&self, v: Vector2d) -> char;
    fn inside(&self, v: Vector2d) -> bool;
    fn set(&mut self, v: Vector2d, c: char);
}

impl GridExt for Grid {
    fn get(&self, v: Vector2d) -> char {
        self[v.y as usize][v.x as usize]
    }
    fn inside(&self, v: Vector2d) -> bool {
        0 <= v.y && v.y < self.len() as i64 && 0 <= v.x && v.x < self[v.y as usize].len() as i64
    }
    fn set(&mut self, v: Vector2d, c: char) {
        self[v.y as usize][v.x as usize] = c;
    }
}

pub fn neighbours8() -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    let d: Vec<i64> = vec![-1, 0, 1];
    for dx in d.iter() {
        for dy in d.iter() {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            res.push((*dx, *dy));
        }
    }
    res
}

pub trait ToI64 {
    fn to_i64(&self) -> i64;
}

impl ToI64 for str {
    fn to_i64(&self) -> i64 {
        parse_i64(self)
    }
}

pub trait ToVec {
    fn to_vec(&self) -> Vec<char>;
}

impl ToVec for str {
    fn to_vec(&self) -> Vec<char> {
        self.chars().collect()
    }
}

pub trait ToStr {
    fn to_str(&self) -> String;
}

impl ToStr for Vec<char> {
    fn to_str(&self) -> String {
        self.iter().collect()
    }
}

// Extended gcd algorithm
// returns (g, x, y) where
//  - gcd(a, b) = g
//  - a * x + b * y = g
pub fn gcd_ext(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (d, x1, y1) = gcd_ext(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    return (d, x, y);
}

// Returns inverse to element a modulo m
// x * a = 1 (mod m)
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _y) = gcd_ext(a, m);
    assert_eq!(g, 1);
    // a * x + m * y == 1
    return x % m;
}

pub trait CollectVec: Iterator + Sized {
    fn cv(self) -> Vec<Self::Item> {
        self.collect()
    }
}
impl<I: Iterator> CollectVec for I {}
