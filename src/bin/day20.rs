use std::fs;

use aoc2024::coord::{ CoordMap, coord_iter, Coord, Dir };
use itertools::Itertools;


fn parse(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path).unwrap().lines().map(|line| {
        line.chars().collect_vec()
    }).collect()
}

fn solve(m: &Vec<Vec<char>>, max_cheat: usize, limit: usize) -> usize {
    let mut p = coord_iter(&m).find(|&p| *m.at(p) == 'S').unwrap();

    // Build path
    let mut path: Vec<Coord> = vec![p];
    while *m.at(p) != 'E' {
        p = Dir::iter().map(|&dir| p.go(dir))
            .find(|&p2| 
                *m.at(p2) != '#' && !(path.len() >= 2 && path[path.len()-2] == p2)
            )
            .unwrap();
        path.push(p);
    }

    // Find cheats
    let mut cnt = 0;
    for i in 0..path.len() {
        for j in (i+limit)..path.len() {
            let dist = (path[j] - path[i]).manhattan();
            let save = j - i - dist;
            if dist <= max_cheat && save >= limit {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let ex = parse("inputs/day20_example.txt");
    let inp = parse("inputs/day20.txt");
    println!("ex1: {} == 1", solve(&ex, 2, 64));
    println!("sol1: {}", solve(&inp, 2, 100));
    println!("sol2: {}", solve(&inp, 20, 100));
}