// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn debug_print(m: &HashSet<(i64, i64)>) {
    let mx = m.iter().map(|v| v.0).max().unwrap() as usize;
    let my = m.iter().map(|v| v.1).max().unwrap() as usize;

    let mut grid = vec![vec!['.'; mx + 1]; my + 1];
    for v in m.iter() {
        grid[v.1 as usize][v.0 as usize] = '#';
    }
    for g in grid.iter() {
        let row: String = g.iter().collect();
        println!("{}", row);
    }
}

pub fn fold(m: &mut HashSet<(i64, i64)>, input: &String) {
    let p = split_string(&input, "=");
    let v = p[1].to_i64();
    let copy = m.clone();
    if p[0] == "x" {
        for (x, y) in copy {
            if x <= v {
                continue;
            }
            let delta = x - v;
            let new_x = v - delta;
            m.remove(&(x, y));
            m.insert((new_x, y));
        }
    } else if p[0] == "y" {
        for (x, y) in copy {
            if y <= v {
                continue;
            }
            let delta = y - v;
            let new_y = v - delta;
            m.remove(&(x, y));
            m.insert((x, new_y));
        }
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut m = HashSet::new();
    for line in lines {
        if line.contains(",") {
            let parts = split_string(line, ",");
            let x = parts[0].to_i64();
            let y = parts[1].to_i64();
            m.insert((x, y));
        } else if line.starts_with("fold") {
            let parts = split_string(line, " ");
            fold(&mut m, &parts[2]);
            // debug_print(&m);
            break;
        }
    }
    m.len() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut m = HashSet::new();
    for line in lines {
        if line.contains(",") {
            let parts = split_string(line, ",");
            let x = parts[0].to_i64();
            let y = parts[1].to_i64();
            m.insert((x, y));
        } else if line.starts_with("fold") {
            let parts = split_string(line, " ");
            fold(&mut m, &parts[2]);
            // debug_print(&m);
            // break;
        }
    }
    // m.len() as i64
    debug_print(&m);
    m.len() as i64
}

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day13/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 731);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 93);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
