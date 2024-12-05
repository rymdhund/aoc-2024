use std::fs;
use std::collections::HashSet;
use std::cmp::Ordering;

use itertools::Itertools;

fn parse(filename: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let text = fs::read_to_string(filename).unwrap();
    let (a, b) = text.trim().split("\n\n").collect_tuple().unwrap();
    let pairs: HashSet<(i32, i32)> = a.lines().map(|line| {
        line.split("|").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap()
    }).collect();
    let updates = b.lines().map(|line| {
        line.split(",").map(|s| s.parse::<i32>().unwrap()).collect_vec()
    }).collect_vec();
    (pairs, updates)
}

fn solve1(inp: &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (pairs, updates) = inp;
    
    updates.iter().filter_map(|seq| 
        is_ok(seq, pairs)
          .then_some(seq[seq.len()/2])
    ).sum()
}

fn solve2(inp: &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (pairs, updates) = inp;
    
    updates.iter().filter_map(|seq| {
        if is_ok(seq, &pairs) {
            None
        } else {
            let mut seq = seq.clone();
            seq.sort_by(|a, b| {
                if pairs.contains(&(*a, *b)) {
                    Ordering::Less
                } else if a == b {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            });
            Some(seq[seq.len()/2])
        }
    }).sum()
}

fn is_ok(seq: &Vec<i32>, pairs: &HashSet<(i32, i32)>) -> bool {
    for i in 0..seq.len() {
        for prev in 0..i {
            let breaking = (seq[i], seq[prev]);
            if pairs.contains(&breaking) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let ex = parse("inputs/day5_example.txt");
    let inp = parse("inputs/day5.txt");
    assert!(solve1(&ex) == 143);
    assert!(solve2(&ex) == 123);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}