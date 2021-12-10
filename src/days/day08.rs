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
    let mut res = 0;
    for line in lines.iter() {
        let parts = split_string(line, " | ");
        let words = split_string(&parts[1], " ");
        for word in words {
            let l = word.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                res += 1;
            }
        }
    }
    res
}

// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

pub fn sorted_digit(s: &str) -> String {
    s.chars().sorted().collect()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    // println!("len = {}", lines.len());
    let mut d = vec![""; 10];
    d[0] = "abcefg";
    d[1] = "cf";
    d[2] = "acdeg";
    d[3] = "acdfg";
    d[4] = "bcdf";
    d[5] = "abdfg";
    d[6] = "abdefg";
    d[7] = "acf";
    d[8] = "abcdefg";
    d[9] = "abcdfg";

    let mut valid: HashMap<String, usize> = HashMap::new();
    for (index, value) in d.iter().enumerate() {
        let key: String = value.to_string();
        valid.insert(key, index);
    }

    let mut rows = Vec::new();
    let mut outputs = Vec::new();

    for line in lines.iter() {
        let parts = split_string(line, " | ");
        let words0 = split_string(&parts[0], " ");
        let words1 = split_string(&parts[1], " ");

        let mut row = Vec::new();
        for word in words0 {
            row.push(word.clone());
        }

        let mut output = Vec::new();
        for word in words1 {
            // row.push(word.clone());
            output.push(word);
        }

        rows.push(row);
        outputs.push(output);
    }

    let mut res = 0;

    for (index, row) in rows.iter().enumerate() {
        let range: Vec<_> = (0..7).collect();
        for p in range.iter().permutations(range.len()) {
            let mut ok = true;
            let mut mapping: HashMap<String, _> = HashMap::new();
            for digit in row.iter() {
                let digit = sorted_digit(digit);
                let mapped: String = digit
                    .chars()
                    .map(|c| (p[c as usize - 'a' as usize] + 'a' as u8) as char)
                    .collect();
                let mapped = sorted_digit(&mapped);
                if !valid.contains_key(&mapped) {
                    // println!("not ok: digit = {}, mapped = {}", digit, mapped);
                    ok = false;
                    break;
                }
                mapping.insert(digit.clone(), valid[&mapped]);
            }
            // println!("p = {:?}, ok = {}", p, ok);
            if ok {
                let mut number = 0;
                for digit in outputs[index].iter() {
                    let digit = sorted_digit(digit);
                    number *= 10;
                    number += mapping[&digit];
                }
                // println!("decoded {}", number);
                res += number as i64;
                break;
            }
        }
    }

    res
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day08/in.txt")
    // read_input("input/day08/t0.txt")
    // read_input("input/day08/t1.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 239);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 946346);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
