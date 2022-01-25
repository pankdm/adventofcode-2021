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
    read_input(&format!("input/day21/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 921585);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 911090395997650);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut pos1 = split_string(&lines[0], " ").last().unwrap().to_i64();
    let mut pos2 = split_string(&lines[1], " ").last().unwrap().to_i64();

    let mut score1 = 0;
    let mut score2 = 0;

    let mut dice = 1;
    let mut count = 0;

    loop {
        // p1
        for _ in 0..3 {
            pos1 += dice;

            dice += 1;
            count += 1;
            if dice > 100 {
                dice -= 100;
            }
        }
        pos1 = (pos1 - 1) % 10 + 1;
        score1 += pos1;

        if score1 >= 1000 {
            println!("player 1 wins, {} {}", score2, count);
            return score2 * count;
        }

        // p2
        for _ in 0..3 {
            pos2 += dice;

            dice += 1;
            count += 1;
            if dice > 100 {
                dice -= 100;
            }
        }
        pos2 = (pos2 - 1) % 10 + 1;
        score2 += pos2;

        if score2 >= 1000 {
            println!("player 2 wins, {} {}", score1, count);
            return score1 * count;
        }
    }
}

type Key = (i64, i64, i64, i64, bool);
type Cache = HashMap<Key, (i64, i64)>;

pub fn dfs(
    cache: &mut Cache,
    pos1: i64,
    score1: i64,
    pos2: i64,
    score2: i64,
    turn1: bool,
) -> (i64, i64) {
    let key = (pos1, score1, pos2, score2, turn1);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut wins1 = 0;
    let mut wins2 = 0;

    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let s = d1 + d2 + d3;

                let mut new_pos1 = pos1;
                let mut new_score1 = score1;
                let mut new_pos2 = pos2;
                let mut new_score2 = score2;

                if turn1 {
                    new_pos1 += s;
                    new_pos1 = (new_pos1 - 1) % 10 + 1;
                    new_score1 += new_pos1;
                    if new_score1 >= 21 {
                        wins1 += 1;
                        continue;
                    }
                } else {
                    new_pos2 += s;
                    new_pos2 = (new_pos2 - 1) % 10 + 1;
                    new_score2 += new_pos2;
                    if new_score2 >= 21 {
                        wins2 += 1;
                        continue;
                    }
                }
                let (new_wins1, new_wins2) =
                    dfs(cache, new_pos1, new_score1, new_pos2, new_score2, !turn1);
                wins1 += new_wins1;
                wins2 += new_wins2;
            }
        }
    }
    cache.insert(key, (wins1, wins2));
    (wins1, wins2)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut pos1 = split_string(&lines[0], " ").last().unwrap().to_i64();
    let mut pos2 = split_string(&lines[1], " ").last().unwrap().to_i64();

    let mut cache = Cache::new();
    let (wins1, wins2) = dfs(&mut cache, pos1, 0, pos2, 0, true);
    println!("{} {}", wins1, wins2);
    wins1.max(wins2)
}
