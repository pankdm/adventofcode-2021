// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
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
    read_input(&format!("input/day22/{}", file))
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

pub fn intersect(x0: i64, x1: i64) -> Vec<i64> {
    let mut res = Vec::new();
    if x1 < -50 || x0 > 50 {
        return res;
    }
    for i in x0.max(-50)..=x1.min(50) {
        res.push(i);
    }
    res
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let sz = 101;
    let mut cubes = vec![vec![vec![false; sz]; sz]; sz];

    let mut counter = 0;
    for line in lines {
        // on x=-20..26,y=-36..17,z=-47..7
        let re =
            Regex::new(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
                .unwrap();
        let cap = re.captures(line).unwrap();
        let flag = if &cap[1] == "off" { false } else { true };
        let v = (2..=7).map(|i| cap[i].to_i64()).cv();
        println!("{}/{} {:?}", counter, lines.len(), v);
        counter += 1;

        for x in intersect(v[0], v[1]) {
            for y in intersect(v[2], v[3]) {
                for z in intersect(v[4], v[5]) {
                    cubes[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = flag;
                }
            }
        }
    }
    cubes
        .iter()
        .flatten()
        .flatten()
        .filter(|x| **x == true)
        .count() as i64
}

type Vec3 = [i64; 3];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    min: Vec3,
    max: Vec3,
}

impl Rect {
    pub fn mid(&self) -> Vec3 {
        assert!(!self.is_final());
        let mut mid = [0; 3];
        for id in 0..3 {
            mid[id] = self.min[id] + (self.max[id] - self.min[id]) / 2;
        }
        mid
    }

    pub fn is_final(&self) -> bool {
        self.min == self.max
    }

    pub fn split(&self) -> Vec<Rect> {
        let mid = self.mid();
        // println!("mid = {:?}", mid);
        let mut res = Vec::new();
        self.split_impl([0; 3], [0; 3], mid, 0, &mut res);
        res
    }

    pub fn split_impl(self, min: Vec3, max: Vec3, mid: Vec3, id: usize, res: &mut Vec<Rect>) {
        if id >= 3 {
            res.push(Rect { min, max });
            return;
        }
        let mut ranges = vec![(self.min[id], mid[id])];
        if mid[id] + 1 <= self.max[id] {
            ranges.push((mid[id] + 1, self.max[id]));
        }

        for (a, b) in ranges {
            let mut next_min = min;
            let mut next_max = max;
            next_min[id] = a;
            next_max[id] = b;
            self.split_impl(next_min, next_max, mid, id + 1, res);
        }
    }

    pub fn volume(&self) -> i64 {
        (0..3).map(|id| self.max[id] - self.min[id] + 1).product()
    }
}

struct Node {
    value: i8,
    rect: Rect,
    children: Vec<Node>,
}

impl Node {
    pub fn update(&mut self, rect: Rect, value: i8) {
        if let Some(clipped_rect) = intersect_rects(self.rect, rect) {
            println!("updating {:?} at {:?}", rect, self.rect);
            if !self.children.is_empty() {
                for child in self.children.iter_mut() {
                    child.update(clipped_rect, value);
                }
                return;
            }
            if self.rect == clipped_rect {
                self.value = value;
                return;
            }
            assert!(!self.rect.is_final());

            // split into children
            let rects = self.rect.split();
            println!("   splitting {:?} at {:?}", clipped_rect, self.rect);
            println!("      ({}) res = {:?}", rects.len(), rects);
            assert!(rects.len() > 1);
            for next_rect in rects {
                self.children.push(Node {
                    rect: next_rect,
                    value: self.value,
                    children: Vec::new(),
                });
            }
            for child in self.children.iter_mut() {
                child.update(clipped_rect, value);
            }
        }
    }

    pub fn count_ones(&self) -> i64 {
        if self.children.is_empty() {
            if self.value == 1 {
                return self.rect.volume();
            } else {
                return 0;
            }
        } else {
            self.children.iter().map(|child| child.count_ones()).sum()
        }
    }

    pub fn count_nodes(&self) -> i64 {
        1 + self
            .children
            .iter()
            .map(|child| child.count_nodes())
            .sum::<i64>()
    }
}

pub fn intersect_rects(rect: Rect, other: Rect) -> Option<Rect> {
    for id in 0..3 {
        if rect.max[id] < other.min[id] || rect.min[id] > other.max[id] {
            return None;
        }
    }
    let mut min = [0; 3];
    let mut max = [0; 3];
    for id in 0..3 {
        min[id] = rect.min[id].max(other.min[id]);
        max[id] = rect.max[id].min(other.max[id]);
    }
    Some(Rect { min, max })
}

struct KDTree {
    root: Node,
}

impl KDTree {
    pub fn new(rect: Rect) -> KDTree {
        KDTree {
            root: Node {
                value: 0,
                rect,
                children: Vec::new(),
            },
        }
    }

    pub fn update(&mut self, rect: Rect, value: i8) {
        self.root.update(rect, value);
    }

    pub fn count_ones(&self) -> i64 {
        self.root.count_ones()
    }
}

pub fn parse_input(lines: &Vec<String>) -> Vec<(Rect, i8)> {
    let mut res = Vec::new();
    for line in lines {
        // on x=-20..26,y=-36..17,z=-47..7
        let re =
            Regex::new(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
                .unwrap();
        let cap = re.captures(line).unwrap();
        let flag = if &cap[1] == "off" { 0 } else { 1 };
        let v = (2..=7).map(|i| cap[i].to_i64()).cv();
        let rect = Rect {
            min: [v[0], v[2], v[4]],
            max: [v[1], v[3], v[5]],
        };
        res.push((rect, flag));
    }
    res
}

pub fn part1_smart(lines: &Vec<String>) -> i64 {
    let bounds = Rect {
        // min: [-50, -50, -50],
        // max: [50, 50, 50],
        min: [-5, -5, -5],
        max: [5, 5, 5],
    };

    let mut kd_tree = KDTree::new(bounds);

    let mut counter = 0;
    for (rect, flag) in parse_input(lines) {
        println!("{}/{} {:?}", counter, lines.len(), rect);
        counter += 1;
        kd_tree.update(rect, flag);
        println!("nodes = {}", kd_tree.root.count_nodes());
        break;
    }
    kd_tree.count_ones()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let input = parse_input(lines);
    let mut min = [0; 3];
    let mut max = [0; 3];
    for id in 0..3 {
        min[id] = input.iter().map(|v| v.0.min[id]).min().unwrap();
        max[id] = input.iter().map(|v| v.0.max[id]).max().unwrap();
    }

    let bounds = Rect { min, max };

    let mut kd_tree = KDTree::new(bounds);

    let mut counter = 0;
    for (rect, flag) in parse_input(lines) {
        println!("{}/{} {:?}", counter, lines.len(), rect);
        counter += 1;
        kd_tree.update(rect, flag);
        break;
    }
    kd_tree.count_ones()
}

pub fn part2_smart(lines: &Vec<String>) -> i64 {
    let input = parse_input(lines);

    let mut points = vec![Vec::new(); 3];
    let mut mappings = vec![HashMap::new(); 3];

    for id in 0..3 {
        let mut uniq_points = HashSet::new();
        for (rect, _) in input.iter() {
            uniq_points.insert(rect.min[id]);
            uniq_points.insert(rect.max[id] + 1);
        }
        let sorted_points = uniq_points.iter().sorted().cloned().cv();
        let mut mapping: HashMap<i64, usize> = HashMap::new();
        for (index, value) in sorted_points.iter().enumerate() {
            mapping.insert(*value, index);
        }
        points[id] = sorted_points;
        mappings[id] = mapping;
    }
    let mx = points[0].len();
    let my = points[1].len();
    let mz = points[2].len();
    println!("total size = {} * {} * {} = {}", mx, my, mz, mx * my * mz);
    let mut state = vec![vec![vec![0; mz]; my]; mx];

    let mut counter = 0;
    for (rect, flag) in input.iter() {
        println!("{}/{} {:?}", counter, lines.len(), rect);
        counter += 1;

        let mut imin = [0; 3];
        let mut imax = [0; 3];
        for id in 0..3 {
            imin[id] = mappings[id][&rect.min[id]];
            imax[id] = mappings[id][&(rect.max[id] + 1)];
        }
        for ix in imin[0]..imax[0] {
            for iy in imin[1]..imax[1] {
                for iz in imin[2]..imax[2] {
                    state[ix][iy][iz] = *flag;
                }
            }
        }
    }

    let mut res = 0;

    for ix in 0..mx - 1 {
        for iy in 0..my - 1 {
            for iz in 0..mz - 1 {
                if state[ix][iy][iz] == 1 {
                    let xsize = points[0][ix + 1] - points[0][ix];
                    let ysize = points[1][iy + 1] - points[1][iy];
                    let zsize = points[2][iz + 1] - points[2][iz];
                    let size = xsize * ysize * zsize;
                    res += size;
                }
            }
        }
    }

    res
}

pub fn main() {
    let lines = read_main_input();

    // println!("part1 = {}", part1(&lines));
    println!("part2_smart = {}", part2_smart(&lines));
    // println!("part2 = {}", part2(&lines));
}
