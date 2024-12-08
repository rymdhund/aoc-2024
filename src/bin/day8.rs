use std::fs;
use std::collections::{HashMap, HashSet};

use aoc2024::coord::{coord_iter, CoordMap, Coord};

use itertools::Itertools;
use num::integer::gcd;


fn parse(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        line.chars().collect_vec()
    }).collect()
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let antennas = find_antennas(map);
    let mut antinodes: HashSet<Coord> = HashSet::new();
    antennas.values().for_each(|coords| {
        for i in 0..coords.len() {
            for j in 0..i {
                let a = coords[i];
                let b = coords[j];
                let diff = a - b;
                if map.contains(b - diff) {
                    antinodes.insert(b - diff);
                }
                if map.contains(a + diff) {
                    antinodes.insert(a + diff);
                }
            }
        }
    });
    antinodes.len()
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let antennas = find_antennas(map);

    let mut antinodes: HashSet<Coord> = HashSet::new();
    antennas.values().for_each(|coords| {
        for i in 0..coords.len() {
            for j in 0..i {
                let a = coords[i];
                let b = coords[j];

                // gcd the diff
                let diff = a - b;
                let div = gcd(diff.x, diff.y);
                let diff = Coord::new(diff.x / div, diff.y / div);

                let mut p = b;
                while map.contains(p) {
                    antinodes.insert(p);
                    p = p - diff;
                }
                let mut p = b + diff;
                while map.contains(p) {
                    antinodes.insert(p);
                    p = p + diff;
                }
            }
        }
    });
    antinodes.len()
}

fn find_antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Coord>> {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

    coord_iter(map).for_each(|coord| {
        let c = *map.at(coord);
        if c != '.' {
            if !antennas.contains_key(&c) {
                antennas.insert(c, Vec::new());
            }
            antennas.get_mut(&c).unwrap().push(coord);
        }
    });
    antennas
}

fn main() {
    let ex = parse("inputs/day8_example.txt");
    let inp = parse("inputs/day8.txt");
    println!("ex1: {} == 14", solve1(&ex));
    println!("ex2: {} == 34", solve2(&ex));
    println!("sol1: {}", solve1(&inp));
    println!("sol1: {}", solve2(&inp));
}