// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn go(
    now: &String,
    graph: &HashMap<String, Vec<String>>,
    path: Vec<String>,
    can2: bool,
) -> i64 {
    let mut res = 0;
    for next in graph[now].iter() {
        if next == "end" {
            res += 1;
            continue;
        }
        if next == "start" {
            continue;
        }

        let mut got2 = false;
        if next.chars().all(|c| c.is_lowercase()) {
            if path.contains(&next) {
                if !can2 {
                    continue;
                }
                got2 = true;
            }
        }
        let mut next_path = path.clone();
        next_path.push(next.clone());
        if got2 {
            res += go(next, graph, next_path, false);
        } else {
            res += go(next, graph, next_path, can2);
        }
    }
    res
}

pub fn solve(lines: &Vec<String>, can2: bool) -> i64 {
    let mut graph = HashMap::new();

    for line in lines {
        let parts = split_string(line, "-");
        let a = parts[0].clone();
        let b = parts[1].clone();
        graph.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
        graph.entry(b).or_insert(Vec::new()).push(a);
    }

    let start = "start".to_string();
    let path = vec![start.clone()];

    go(&start, &graph, path, can2)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    solve(lines, false)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    solve(lines, true)
}

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day12/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 4970);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 137948);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
