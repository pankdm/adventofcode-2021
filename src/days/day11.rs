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
    let mut state = Vec::new();
    for line in lines {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
        state.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let rx = state.len();
    let cx = state[0].len();

    let mut res = 0;
    for step in 0..100 {
        let mut next = state.clone();
        for row in next.iter_mut() {
            for value in row.iter_mut() {
                *value += 1;
            }
        }
        let mut flashed = vec![vec![false; cx]; rx];
        state = next.clone();

        // flashes
        let mut index = 0;
        loop {
            next = state.clone();
            let mut changed = false;
            for r in 0..rx {
                for c in 0..cx {
                    if state[r][c] > 9 && !flashed[r][c] {
                        flashed[r][c] = true;
                        changed = true;
                        state[r][c] = 0;
                        res += 1;
                        for delta in neighbours8() {
                            let pt = Vector2d::new(c as i64, r as i64) + delta;
                            if next.inside(pt) {
                                let current = next.get(pt);
                                next.set(pt, current + 1);
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
            state = next.clone();
            index += 1;
        }
        for r in 0..rx {
            for c in 0..cx {
                if state[r][c] > 9 {
                    state[r][c] = 0;
                }
            }
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut state = Vec::new();
    for line in lines {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
        state.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let rx = state.len();
    let cx = state[0].len();

    let mut res = 0;
    for step in 0..100000 {
        let mut next = state.clone();
        for row in next.iter_mut() {
            for value in row.iter_mut() {
                *value += 1;
            }
        }
        let mut flashed = vec![vec![false; cx]; rx];
        state = next.clone();

        // flashes
        let mut index = 0;
        let mut num_flashed = 0;
        loop {
            next = state.clone();
            let mut changed = false;
            for r in 0..rx {
                for c in 0..cx {
                    if state[r][c] > 9 && !flashed[r][c] {
                        flashed[r][c] = true;
                        changed = true;
                        state[r][c] = 0;
                        num_flashed += 1;
                        res += 1;
                        for delta in neighbours8() {
                            let pt = Vector2d::new(c as i64, r as i64) + delta;
                            if next.inside(pt) {
                                let current = next.get(pt);
                                next.set(pt, current + 1);
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
            state = next.clone();
            index += 1;
        }
        // println!("day = {}, flashed = {}", step, num_flashed);
        if num_flashed == rx * cx {
            return step + 1;
        }

        for r in 0..rx {
            for c in 0..cx {
                if state[r][c] > 9 {
                    state[r][c] = 0;
                }
            }
        }
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
