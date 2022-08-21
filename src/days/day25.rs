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
    read_input(&format!("input/day25/{}", file))
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

type SeaMap = Vec<Vec<char>>;

pub fn move_all(m: &mut SeaMap, t: char) -> bool {
    let height = m.len();
    let width = m[0].len();
    let mut old = m.clone();
    let mut should_move = vec![vec![false; width]; height];
    let mut moved = false;
    for h in 0..height {
        for w in 0..width {
            let mut h1 = h;
            let mut w1 = w;
            if old[h][w] != t {
                continue;
            }
            if t == '>' {
                w1 += 1;
                w1 %= width;
            } else {
                h1 += 1;
                h1 %= height;
            }
            if old[h1][w1] == '.' {
                m[h1][w1] = t;
                m[h][w] = '.';
                moved = true;
            }
        }
    }
    moved
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut m = to_vv_char(lines);

    let mut count = 0;
    loop {
        println!("\niter = {}", count);
        // if count > 100 {
        //     break;
        // }
        let mut moved = false;
        moved |= move_all(&mut m, '>');
        moved |= move_all(&mut m, 'v');

        // for row in m.iter() {
        //     let s: String = row.iter().collect();
        //     println!("{}", s);
        // }

        count += 1;
        if !moved {
            return count;
        }
    }
    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    -1
}
