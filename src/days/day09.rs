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
    let mut map = Vec::new();
    for line in lines {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
        map.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut res = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let mut ok = true;
            for (dr, dc) in dirs.iter() {
                let rr = r as i32 + dr;
                let cc = c as i32 + dc;
                if rr >= 0 && rr < map.len() as i32 && cc >= 0 && cc < map[r].len() as i32 {
                    if map[rr as usize][cc as usize] <= map[r][c] {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                // println!("({}, {}) -> {}", r, c, map[r][c]);
                res += map[r][c] as i64 + 1;
            }
        }
    }
    res
}

pub fn dfs(r: usize, c: usize, map: &Vec<Vec<u8>>, comp: &mut Vec<Vec<i32>>, index: i32) {
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    comp[r][c] = index;
    for (dr, dc) in dirs.iter() {
        let rr = r as i32 + dr;
        let cc = c as i32 + dc;
        if rr >= 0 && rr < map.len() as i32 && cc >= 0 && cc < map[r].len() as i32 {
            let r1 = rr as usize;
            let c1 = cc as usize;
            if map[r1][c1] != 9 && comp[r1][c1] == 0 {
                dfs(r1, c1, map, comp, index);
            }
        }
    }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut map = Vec::new();
    for line in lines {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
        map.push(row);
    }
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut comp = vec![vec![0; map[0].len()]; map.len()];
    let mut index = 0;

    let mut res = 0;

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] < 9 && comp[r][c] == 0 {
                index += 1;
                dfs(r, c, &map, &mut comp, index);
            }
        }
    }

    let mut counts = HashMap::new();
    for row in comp.iter() {
        for v in row.iter() {
            if *v > 0 {
                *counts.entry(v).or_insert(0) += 1;
            }
        }
    }

    let mut flat: Vec<_> = counts.iter().map(|(k, v)| *v as i64).collect();
    flat.sort();
    flat.reverse();
    flat[0] * flat[1] * flat[2]
}

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let file = if args.len() < 2 {
        "in.txt".to_string()
    } else {
        args[1].to_string()
    };
    read_input(&format!("input/day09/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 436);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 1317792);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
