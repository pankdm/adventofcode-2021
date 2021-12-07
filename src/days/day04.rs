// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn check_board(b: &Vec<Vec<i64>>, index: usize, nums: &Vec<i64>) -> i64 {
    let was: HashSet<_> = nums[0..=index].iter().collect();

    let mut score = 0;
    for row in 0..5 {
        for col in 0..5 {
            let x = b[row][col];
            if !was.contains(&x) {
                score += x;
            }
        }
    }

    for val in 0..5 {
        let good_col = (0..5).map(|i| b[val][i]).all(|x| was.contains(&x));
        let good_row = (0..5).map(|i| b[i][val]).all(|x| was.contains(&x));
        if good_col || good_row {
            println!(
                "at value = {} (id = {}), score = {}",
                nums[index], index, score
            );
            return score * nums[index];
        }
    }

    -1
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut ind = 0;
    let nums = parse_ints(&lines[ind], ",");
    ind += 2;

    let mut boards = Vec::new();

    while ind < lines.len() {
        let mut board = Vec::new();
        for i in 0..5 {
            let ints: Vec<_> = lines[ind].split_whitespace().map(|s| s.to_i64()).collect();
            board.push(ints);
            ind += 1;
        }
        ind += 1;
        boards.push(board);
    }
    for index in 0..nums.len() {
        for board in boards.iter() {
            let score = check_board(&board, index, &nums);
            if score != -1 {
                return score;
            }
        }
    }
    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut ind = 0;
    let nums = parse_ints(&lines[ind], ",");
    ind += 2;

    let mut boards = Vec::new();

    while ind < lines.len() {
        let mut board = Vec::new();
        for i in 0..5 {
            let ints: Vec<_> = lines[ind].split_whitespace().map(|s| s.to_i64()).collect();
            board.push(ints);
            ind += 1;
        }
        ind += 1;
        boards.push(board);
    }
    let mut res = Vec::new();
    for (bi, board) in boards.iter().enumerate() {
        for index in 0..nums.len() {
            let score = check_board(&board, index, &nums);
            if score != -1 {
                res.push((index, score));
                break;
            }
        }
    }
    res.iter().max().unwrap().1
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day04/in.txt")
    // read_input("input/day04/t1.txt")
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
