// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

extern crate aoc;
use aoc::*;
use days::*;

fn main() {
    day08::main();
    // let args: Vec<String> = env::args().collect();
    // let lines = read_input_from_args(&args);

    // dbg!(day15::part1(&lines));
    // dbg!(day15::part2(&lines));
}
