use std::fs;

use aoc2024::coord::{Coord, CoordMap};
use itertools::Itertools;

fn parse(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        line.chars().collect_vec()
    }).collect()
}

fn solve1(lines: &Vec<Vec<char>>) -> i32 {
    let dirs = vec![Coord::right(), Coord::down(), Coord::new(1, 1), Coord::new(-1, 1)];
    let mut cnt = 0;
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            let p = Coord::new_u(col, row);
            for dir in &dirs {
                if is_string(lines, p, *dir, "XMAS") {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

fn solve2(lines: &Vec<Vec<char>>) -> i32 {
    let dr = Coord::new(1, 1);
    let dl = Coord::new(-1, 1);
    let mut cnt = 0;
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            let p = Coord::new_u(col, row);
            if is_string(lines, p-dr, dr, "MAS") && is_string(lines, p-dl, dl, "MAS"){
                cnt += 1;
            }
        }
    }
    cnt
}

fn get_iter(x: &Vec<Vec<char>>) -> impl Iterator<Item = Coord> + '_ {
    let x = (0..x.len()).flat_map(|r| (0..x[0].len()).map(move |c| Coord::new_u(c, r)));
    return x;
}

fn is_string(source: &Vec<Vec<char>>, pos: Coord, dir: Coord, s: &str) -> bool {
    let s1: String = source.take_in_dir(pos, dir, s.len()).into_iter().collect();
    s1 == s || s1.chars().rev().collect::<String>() == s
}

fn main() {
    let ex = parse("inputs/day4_example.txt");
    let inp = parse("inputs/day4.txt");
    assert!(solve1(&ex) == 18);
    assert!(solve2(&ex) == 9);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}