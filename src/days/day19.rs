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
        file = args[1].to_string();
    }
    println!("Using file: {}", file);
    read_input(&format!("input/day19/{}", file))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_rotations() {
        let beacons = [
            [-1, -1, 1],
            [-2, -2, 2],
            [-3, -3, 3],
            [-2, -3, 1],
            [5, 6, -4],
            [8, 0, 7],
        ];
        for r in all_rotations().iter() {
            println!("");
            let mapped = beacons.iter().map(|x| map_vector(x, r)).cv();
            for a in mapped.iter() {
                println!("{:?}", a);
            }
        }
    }

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

struct Rotation {
    axis: Vec<usize>,
    signs: Vec<i32>,
}

type Vec3d = [i32; 3];

fn cross(a: &Vec3d, b: &Vec3d) -> Vec3d {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn add(a: &Vec3d, b: &Vec3d) -> Vec3d {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn neg(a: &Vec3d) -> Vec3d {
    [-a[0], -a[1], -a[2]]
}

fn map_vector(a: &Vec3d, r: &Rotation) -> Vec3d {
    let mut res = [0; 3];
    for i in 0..3 {
        res[r.axis[i]] = a[i] * r.signs[i];
    }
    res
}

fn all_rotations() -> Vec<Rotation> {
    let mut res = Vec::new();

    let all_axis = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 1, 0],
        [2, 0, 1],
    ];
    let mut all_signs = Vec::new();
    for x in [-1, 1] {
        for y in [-1, 1] {
            for z in [-1, 1] {
                all_signs.push([x, y, z]);
            }
        }
    }
    for axis in all_axis.iter() {
        for signs in all_signs.iter() {
            let r = Rotation {
                axis: axis.to_vec(),
                signs: signs.to_vec(),
            };
            let x = map_vector(&[1, 0, 0], &r);
            let y = map_vector(&[0, 1, 0], &r);
            let z = map_vector(&[0, 0, 1], &r);
            if cross(&x, &y) == z {
                res.push(r);
            }
        }
    }
    res
}

fn in_bounds(center: &Vec3d, point: &Vec3d) -> bool {
    for i in 0..3 {
        if (center[i] - point[i]).abs() > 1000 {
            return false;
        }
    }
    true
}

fn is_matching(beacons: &Vec<Vec3d>, other_beacons: &Vec<Vec3d>) -> (bool, Vec3d) {
    let origin = [0, 0, 0];
    for a in beacons.iter() {
        // if a != &[-618, -824, -621] {
        //     continue;
        // }
        for b in other_beacons.iter() {
            let other_center = add(a, &neg(b));
            // println!(
            //     "  matching {:?} with {:?}, center = {:?}",
            //     a, b, other_center
            // );
            let points = beacons
                .iter()
                .filter(|p| in_bounds(&other_center, p))
                .cloned()
                .sorted()
                .cv();
            let other_points = other_beacons
                .iter()
                .map(|x| add(&other_center, x))
                .filter(|p| in_bounds(&origin, p))
                .sorted()
                .cv();

            // println!(" points ({}): {:#?}", points.len(), points);
            // println!(
            //     " other points ({}): {:#?}",
            //     other_points.len(),
            //     other_points
            // );

            if points.len() == other_points.len() && points.len() >= 12 {
                if points == other_points {
                    return (true, other_center);
                }
            }
        }
    }
    (false, [0, 0, 0])
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut scanners = Vec::new();
    let mut cur = Vec::new();
    for line in lines.iter() {
        if line.starts_with("--- scanner") {
            continue;
        }
        if line.is_empty() {
            scanners.push(cur.clone());
            cur = Vec::new();
            continue;
        }
        let parts = split_string(line, ",")
            .iter()
            .map(|x| x.to_i64() as i32)
            .cv();
        let point: [i32; 3] = parts.try_into().unwrap();
        cur.push(point);
    }
    if !cur.is_empty() {
        scanners.push(cur.clone());
    }
    println!("total = {}", scanners.len());
    let rotations = all_rotations();
    // let rotations = vec![Rotation {
    //     axis: vec![0, 1, 2],
    //     signs: vec![-1, 1, -1],
    // }];

    println!("rotations = {}", rotations.len());
    // println!("{:?}", scanners);

    let mut offsets = vec![[0, 0, 0]; scanners.len()];
    let mut matched = vec![false; scanners.len()];

    matched[0] = true;
    let mut counter = 1;
    loop {
        let mut find_matched = || {
            // loop through matched
            for i1 in (0..scanners.len()).filter(|index| matched[*index] == true) {
                // loop through unmatched
                let beacons = &scanners[i1];
                let center = offsets[i1];
                for i2 in (0..scanners.len()).filter(|index| matched[*index] == false) {
                    for r in rotations.iter() {
                        let other_beacons = scanners[i2].iter().map(|x| map_vector(x, r)).cv();
                        let (res, relative_center) = is_matching(beacons, &other_beacons);
                        if res == true {
                            // update matched beacons with new orientation
                            scanners[i2] = other_beacons;
                            return (i1, i2, add(&center, &relative_center));
                        }
                    }
                }
            }
            // everything was matched
            (0, 0, [0, 0, 0])
        };
        let (from, to, absolute_center) = find_matched();
        if to == 0 {
            println!("nothing matched, exiting");
            break;
        } else {
            counter += 1;
            println!(
                "[{}/{}] matched {} with {}, center = {:?}",
                counter,
                scanners.len(),
                from,
                to,
                absolute_center
            );
            matched[to] = true;
            offsets[to] = absolute_center;
        }
    }

    println!("");
    for i in 0..offsets.len() {
        println!("{:?} -> {:?}", matched[i], offsets[i]);
    }

    let mut all_beacons = HashSet::new();
    for i in 0..scanners.len() {
        assert_eq!(matched[i], true);
        for beacon in scanners[i].iter().map(|x| add(&offsets[i], x)) {
            all_beacons.insert(beacon);
        }
    }
    all_beacons.len() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let offsets = [
        [0, 0, 0],
        [29, 1057, 105],
        [-1140, -135, 1377],
        [53, -117, -3543],
        [1191, 2329, 1375],
        [86, -127, -1153],
        [-1085, 1120, 47],
        [45, 2280, -1019],
        [1216, 1120, 2416],
        [1160, 2231, -1123],
        [-31, -2493, -2288],
        [-2447, -99, 1322],
        [129, -2430, -1044],
        [-2325, -2564, 142],
        [-1080, -2479, 57],
        [88, -81, 1292],
        [118, 2270, 157],
        [-3, -3739, -1197],
        [-1117, 2390, 133],
        [14, -113, -2259],
        [-1150, -2511, -2339],
        [1325, 1205, 103],
        [18, -1198, 75],
        [111, -2492, 12],
        [127, -2412, -3436],
        [-1152, -1337, 1293],
        [109, -2416, 1228],
        [78, 1134, 1190],
        [-2303, -3720, 146],
        [2409, 2252, 1199],
        [18, -4915, -1074],
        [1191, 6, 1319],
        [1149, 1171, 1360],
    ];

    let mut dists = Vec::new();
    for a in offsets.iter() {
        for b in offsets.iter() {
            let dist: i64 = (0..3).map(|i| ((a[i] - b[i]) as i64).abs()).sum();
            dists.push(dist);
        }
    }
    *dists.iter().max().unwrap()
}
