use std::fs;
use std::collections::HashSet;

use aoc2024::coord::{coord_iter, Dir, Coord, CoordMap};
use itertools::Itertools;

fn parse(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path).unwrap().trim().lines().map(|line| line.chars().collect_vec()).collect()
}

fn solve1(inp: &Vec<Vec<char>>) -> usize {
    let mut used: HashSet<Coord> = HashSet::new();
    let mut acc = 0;

    coord_iter(inp).for_each(|p| {
        if used.contains(&p) {
            return;
        }

        let mut seen: HashSet<Coord> = HashSet::new();
        let mut xs = vec![p];
        let mut border = 0;
        while let Some(x) = xs.pop() {
            if seen.contains(&x) {
                continue;
            }
            seen.insert(x);
            used.insert(x);

            for &d in Dir::iter() {
                let y = x.go(d);
                if !inp.contains(y) {
                    border += 1;
                } else if inp.at(p) == inp.at(y) {
                    xs.push(y);
                } else {
                    border += 1;
                }
            }
        }
        acc += border * seen.len();
    });
    acc
}

fn solve2(inp: &Vec<Vec<char>>) -> usize {
    let mut used: HashSet<Coord> = HashSet::new();
    let mut acc = 0;

    coord_iter(inp).for_each(|p| {
        if used.contains(&p) {
            return;
        }

        let mut seen: HashSet<Coord> = HashSet::new();
        let mut xs = vec![p];
        while let Some(x) = xs.pop() {
            if seen.contains(&x) {
                continue;
            }
            seen.insert(x);
            used.insert(x);

            for &d in Dir::iter() {
                let y = x.go(d);
                if inp.contains(y) && inp.at(p) == inp.at(y) {
                    xs.push(y);
                }
            }
        }
        acc += sides(&seen) * seen.len();
    });
    acc
}

fn sides(shape: &HashSet<Coord>) -> usize {
    let minx = shape.iter().map(|p| p.x).min().unwrap();
    let miny = shape.iter().map(|p| p.y).min().unwrap();
    let maxx = shape.iter().map(|p| p.x).max().unwrap() + 2;
    let maxy = shape.iter().map(|p| p.y).max().unwrap() + 2;

    let mut cnt = 0;

    // go through each column
    for x in minx..maxx {
        let mut prev_border_dir = None;
        for y in miny..maxy {
            let p = Coord::new(x, y);

            let border_dir = match (shape.contains(&p), shape.contains(&p.go(Dir::Left))) {
                (true, false) => Some(Dir::Left),
                (false, true) => Some(Dir::Right),
                _ => None,
            };
            if border_dir != prev_border_dir {
                cnt += 1
            }
            prev_border_dir = border_dir;
        }
    }
    for y in miny..maxy {
        let mut prev_border_dir = None;
        for x in minx..maxx {
            let p = Coord::new(x, y);

            let border_dir = match (shape.contains(&p), shape.contains(&p.go(Dir::Up))) {
                (true, false) => Some(Dir::Up),
                (false, true) => Some(Dir::Down),
                _ => None,
            };
            if border_dir != prev_border_dir {
                cnt += 1
            }
            prev_border_dir = border_dir;
        }
    }
    cnt
}

fn main() {
    let ex = parse("inputs/day12_example.txt");
    let inp = parse("inputs/day12.txt");
    println!("ex1: {} == 1930", solve1(&ex));
    println!("ex2: {} == 1206", solve2(&ex));
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}