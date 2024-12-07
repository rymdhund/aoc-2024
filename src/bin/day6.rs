use std::{collections::HashSet, fs};

use aoc2024::coord::{coord_iter, CoordMap, Coord, Dir};
use itertools::Itertools;


fn parse(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename).unwrap().lines().map(|line|
        line.chars().collect_vec()
    ).collect()
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut pos = find_in_map(map, '^')[0];
    let mut dir = Dir::Up;

    let mut path: HashSet<Coord> = HashSet::new();
    path.insert(pos);

    while map.contains(pos) {
        let next = pos.go(dir);
        if !map.contains(next) {
            break;
        }
        if *map.at(next) == '#' {
            dir = dir.turn_right();
        } else {
            pos = next;
            path.insert(pos);
        }
    }
    path.len()
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut pos = find_in_map(map, '^')[0];
    let mut dir = Dir::Up;

    let mut path: HashSet<Coord> = HashSet::new();
    path.insert(pos);

    let mut obsts: HashSet<Coord> = HashSet::new();

    while map.contains(pos) {
        let next = pos.go(dir);
        if !map.contains(next) {
            break;
        }
        if *map.at(next) == '#' {
            dir = dir.turn_right();
        } else {
            if !path.contains(&next) && is_loop(map, pos, dir, next) {
                obsts.insert(next);
            }
            pos = next;
            path.insert(pos);
        }

    }
    obsts.len()
}

fn find_in_map(map: &Vec<Vec<char>>, c: char) -> Vec<Coord> {
    coord_iter(map).filter(|&p| {
        map.at(p) == &c
    }).collect_vec()
}

fn is_loop(map: &Vec<Vec<char>>, pos: Coord, dir: Dir, obst: Coord) -> bool {
    let mut path: HashSet<(Coord, Dir)> = HashSet::new();
    path.insert((pos, dir));

    let mut pos = pos;
    let mut dir = dir;

    loop {
        let next = pos.go(dir);
        if !map.contains(next) {
            return false;
        }
        if path.contains(&(next, dir)) {
            return true;
        }
        if *map.at(next) == '#' || next == obst {
            dir = dir.turn_right();
        } else {
            pos = next;
            path.insert((pos, dir));
        }
    }
}

fn main() {
    let ex = parse("inputs/day6_example.txt");
    let inp = parse("inputs/day6.txt");
    println!("ex1: {} == 41", solve1(&ex));
    println!("ex2: {} == 6", solve2(&ex));
    println!("sol1: {}", solve1(&inp));
    println!("sol1: {}", solve2(&inp));
}