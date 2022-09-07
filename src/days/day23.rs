// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;

use crate::*;

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    read_input(&format!("input/day23/{}", file))
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

// # # # # # # # # # # # # #
// # 1 2 3 4 5 6 7 8 9 1011 #
// # # #A13#B15#C17#D19# # #
//     #A23#B25#C27#D29#
//     # # # # # # # # #
const desired: [i32; 8] = [3, 3, 5, 5, 7, 7, 9, 9];
const cost: [i32; 8] = [1, 1, 10, 10, 100, 100, 1000, 1000];

const DEBUG: bool = false;

pub fn in_room(pos: i32) -> bool {
    pos >= 13
}

pub fn in_between(pos: i32, a: i32, b: i32) -> bool {
    assert!(pos < 12);
    assert!(a < 12);
    assert!(b < 12);

    if a <= b {
        a <= pos && pos <= b
    } else {
        b <= pos && pos <= a
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    pos: Vec<i32>,
    cost: i32,
    occupied: i64,
    locked: Vec<bool>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn from_pos(pos: Vec<i32>) -> State {
        // let mut pos = vec![13, 23, 15, 25, 6, 27, 19, 29];
        // let mut locked = vec![true; 8];
        // locked[4] = false;

        let locked = vec![false; 8];

        let mut occupied = 0;
        for p in &pos {
            occupied |= 1 << p;
        }
        // very simple example
        // Test configuration
        // #############
        // #...........#
        // ###A#C#B#D###
        //   #A#B#C#D#
        //   #########
        State {
            pos,
            occupied,
            cost: 0,
            locked,
        }
    }

    pub fn is_empty(&self, pos: i32) -> bool {
        (self.occupied & (1 << pos)) == 0
    }

    pub fn initial() -> State {
        // #############
        // #...........#
        // ###D#D#C#B###
        //   #B#A#A#C#
        //   #########
        State::from_pos(vec![25, 27, 23, 19, 17, 29, 13, 15])

        // Test configuration
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        // State::from_pos(vec![23, 29, 13, 17, 15, 27, 19, 25])
    }

    pub fn next_moves(&self) -> Vec<State> {
        let mut res = Vec::new();
        for i in 0..8 {
            if DEBUG {
                println!("adding for i = {}", i);
            }
            if in_room(self.pos[i]) {
                if self.locked[i] {
                    continue;
                }
                // amphiods could that are in rooms can move into any spot in hallway
                let mut cur = self.pos[i];
                let mut ok = true;
                while cur >= 13 {
                    if self.is_empty(cur - 10) {
                        cur -= 10;
                    } else {
                        ok = false;
                        break;
                    }
                }
                if !self.is_empty(cur) {
                    ok = false;
                }

                if !ok {
                    continue;
                }

                for dst in cur + 1..=11 {
                    if self.is_empty(dst) {
                        let moves = self.pos[i] / 10 + (cur - dst).abs();

                        let mut next_state = self.clone();
                        next_state.pos[i] = dst;
                        next_state.cost += moves * cost[i];
                        next_state.occupied -= 1 << self.pos[i];
                        next_state.occupied += 1 << dst;

                        if DEBUG {
                            println!("  Adding next_state {:?}", next_state);
                        }
                        res.push(next_state);
                    } else {
                        break;
                    }
                }

                for dst in (1..=cur - 1).rev() {
                    if self.is_empty(dst) {
                        let moves = self.pos[i] / 10 + (cur - dst).abs();

                        let mut next_state = self.clone();
                        next_state.pos[i] = dst;
                        next_state.cost += moves * cost[i];
                        next_state.occupied -= 1 << self.pos[i];
                        next_state.occupied += 1 << dst;

                        if DEBUG {
                            println!("  Adding next_state {:?}", next_state);
                        }
                        res.push(next_state);
                    } else {
                        break;
                    }
                }
            } else {
                // println!(" checking {} outside the room", i);
                // apmhiods that are in hallway can move into their desired room
                // check that there are no other amphiods in the room
                let mut ok = true;
                // by default it wants to go to the last cell
                let mut dst = desired[i] + 20;
                for other_id in 0..8 {
                    // skip the same one
                    if other_id == i {
                        continue;
                    }

                    let other = self.pos[other_id];
                    if in_room(other) {
                        if other_id / 2 == i / 2 {
                            if self.pos[other_id] == dst {
                                dst -= 10;
                            }
                        } else if self.pos[other_id] % 10 == desired[i] {
                            ok = false;
                            // println!(
                            //     "not OK: other {} at {} is at the same desired {}",
                            //     other_id, other, desired[i]
                            // );
                            break;
                        }
                    } else {
                        if in_between(other, self.pos[i], desired[i]) {
                            // println!(
                            //     "not OK: other {} at {} is between [{}, {}]",
                            //     other_id, other, self.pos[i], desired[i]
                            // );
                            ok = false;
                        }
                    }
                }
                if !self.is_empty(dst) || !self.is_empty(desired[i] + 10) {
                    // println!(
                    //     "not OK: other dst {} or {} are not empty",
                    //     dst,
                    //     desired[i] + 10
                    // );
                    ok = false;
                }

                if ok {
                    let moves = dst / 10 + (self.pos[i] - desired[i]).abs();

                    let mut next_state = self.clone();
                    next_state.pos[i] = dst;
                    next_state.cost += moves * cost[i];
                    next_state.occupied -= 1 << self.pos[i];
                    next_state.occupied += 1 << dst;
                    next_state.locked[i] = true;
                    // println!("  Adding move in state: {:?}", next_state);
                    res.push(next_state);
                }
            }
        }
        res
    }

    pub fn is_in_desired(&self, i: usize) -> bool {
        let x1 = desired[i] + 10;
        let x2 = desired[i] + 20;
        if self.pos[i] == x1 || self.pos[i] == x2 {
            return true;
        }
        return false;
    }

    pub fn is_final(&self) -> bool {
        for i in 0..8 {
            if !self.is_in_desired(i) {
                return false;
            }
        }
        true
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let s = State::initial();

    let mut heap = BinaryHeap::new();
    let mut expanded = HashSet::new();

    heap.push(s);

    let mut best_cost = -1;
    let mut counter = 0;

    while !heap.is_empty() {
        let cur = heap.pop().unwrap();
        let key = (cur.pos.clone(), cur.locked.clone());
        if expanded.contains(&key) {
            // skipping already expanded ones
            continue;
        }

        counter += 1;
        if counter % 100000 == 0 {
            println!("{} -> At {:?}", counter / 1000, cur);
        }

        if cur.is_final() {
            let cur_cost = cur.cost;
            println!("found best = {}", cur_cost);
            best_cost = cur_cost;
            break;
        }

        for next in cur.next_moves() {
            heap.push(next);
        }
        expanded.insert(key);
        // break;
    }

    best_cost as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    -1
}
