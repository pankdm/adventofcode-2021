// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::*;

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day15/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 429);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2844);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut map = Vec::new();
    for line in lines {
        let row = line.chars().map(|x| (x as u8 - '0' as u8) as i32).cv();
        map.push(row);
    }
    map
}

pub fn dijkstra(map: &Vec<Vec<i32>>) -> i64 {
    let h = map.len();
    let w = map[0].len();

    let mut visited = vec![vec![false; w]; h];

    let last = ((w - 1) as i32, (h - 1) as i32);

    let mut dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));

    loop {
        let (d, x0, y0) = heap.pop().unwrap();
        if (x0, y0) == last {
            return d.0 as i64;
        }
        if visited[y0 as usize][x0 as usize] {
            continue;
        }
        visited[y0 as usize][x0 as usize] = true;

        for (dx, dy) in dirs.iter() {
            let x = x0 + dx;
            let y = y0 + dy;
            if x >= 0 && y >= 0 && y < map.len() as i32 && x < map[0].len() as i32 {
                let next_d = map[y as usize][x as usize] + d.0;
                heap.push((Reverse(next_d), x, y));
            }
        }
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let map = parse(lines);
    dijkstra(&map)
}

pub fn mod9(mut x: i32) -> i32 {
    while x > 9 {
        x -= 9;
    }
    x
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let small = parse(lines);

    let height = small.len();
    let width = small[0].len();

    let ymax = 5 * height;
    let xmax = 5 * width;
    let mut map = vec![vec![0 as i32; xmax]; ymax];

    for h in 0..5 {
        for w in 0..5 {
            for y in 0..height {
                for x in 0..width {
                    map[y + h * height][x + w * width] = mod9(small[y][x] + h as i32 + w as i32);
                }
            }
        }
    }

    dijkstra(&map)
}
