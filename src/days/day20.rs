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
    read_input(&format!("input/day20/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 5498);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 16014);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

type Img = HashMap<(i64, i64), char>;

pub fn collect(row: i64, col: i64, img: &Img, default: char) -> i64 {
    let mut res = 0;
    for r in row - 1..=row + 1 {
        for c in col - 1..=col + 1 {
            res *= 2;
            if img.get(&(r, c)).unwrap_or_else(|| &default) == &'#' {
                res += 1;
            }
        }
    }
    res
}

pub fn print_img(img: &Img) {
    let (rmin, rmax) = img.keys().map(|x| x.0).minmax().into_option().unwrap();
    let (cmin, cmax) = img.keys().map(|x| x.1).minmax().into_option().unwrap();
    for r in rmin..=rmax {
        for c in cmin..=cmax {
            print!("{}", img.get(&(r, c)).unwrap_or_else(|| &'.'));
        }
        println!("");
    }
}

pub fn enhance(img: &mut Img, algo: &Vec<char>, default: char) -> Img {
    let (rmin, rmax) = img.keys().map(|x| x.0).minmax().into_option().unwrap();
    let (cmin, cmax) = img.keys().map(|x| x.1).minmax().into_option().unwrap();

    let mut new_img: Img = HashMap::new();
    for r in rmin - 1..=rmax + 1 {
        for c in cmin - 1..=cmax + 1 {
            let index = collect(r, c, img, default) as usize;
            new_img.insert((r, c), algo[index]);
        }
    }
    new_img
}

pub fn solve_impl(lines: &Vec<String>, steps: i32) -> i64 {
    let algo = lines[0].clone().to_vec();

    let mut img: Img = HashMap::new();

    let rows = lines[2..].iter().cloned().cv();
    for (ri, row) in rows.iter().enumerate() {
        for (ci, ch) in row.to_vec().iter().enumerate() {
            assert!(*ch == '.' || *ch == '#');
            img.insert((ri as i64, ci as i64), *ch);
        }
    }

    let mut default = '.';
    for step in 0..steps {
        img = enhance(&mut img, &algo, default);
        if default == '.' {
            default = algo[0];
        } else {
            default = algo[511];
        }
    }
    img.values().filter(|x| **x == '#').count() as i64
}

pub fn part1(lines: &Vec<String>) -> i64 {
    solve_impl(lines, 2)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    solve_impl(lines, 50)
}
