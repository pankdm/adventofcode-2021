// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn flash(mut m: &mut Vec<Vec<u8>>) -> i64 {
    let mut num_flashes = 0;
    let mut flashed = vec![vec![false; m[0].len()]; m.len()];
    loop {
        let mut next = m.clone();
        let mut changed = false;
        for r in 0..m.len() {
            for c in 0..m[0].len() {
                if m[r][c] > 9 && !flashed[r][c] {
                    flashed[r][c] = true;
                    changed = true;
                    m[r][c] = 0;
                    num_flashes += 1;
                    for (dr, dc) in neighbours8() {
                        let rr = r as i64 + dr;
                        let cc = c as i64 + dc;
                        if rr >= 0 && rr < m.len() as i64 && cc >= 0 && cc < m[0].len() as i64 {
                            next[rr as usize][cc as usize] += 1;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
        *m = next.clone();
    }
    num_flashes
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut state = Vec::new();
    for line in lines {
        let row = line.chars().map(|c| c as u8 - '0' as u8).cv();
        state.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let rx = state.len();
    let cx = state[0].len();

    let mut res = 0;
    for step in 0..100 {
        state.iter_mut().flatten().for_each(|x| *x += 1);
        res += flash(&mut state);
        state.iter_mut().flatten().for_each(|x| {
            if *x > 9 {
                *x = 0
            }
        });
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut state = Vec::new();
    for line in lines {
        let row = line.chars().map(|c| c as u8 - '0' as u8).cv();
        state.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let rx = state.len();
    let cx = state[0].len();

    let mut res = 0;
    for step in 0..100000 {
        state.iter_mut().flatten().for_each(|x| *x += 1);
        let num_flashed = flash(&mut state) as usize;
        if num_flashed == rx * cx {
            return step + 1;
        }
        state.iter_mut().flatten().for_each(|x| {
            if *x > 9 {
                *x = 0
            }
        });
    }
    -1
}

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day11/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 1588);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 517);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
