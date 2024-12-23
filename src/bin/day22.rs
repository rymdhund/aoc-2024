use std::collections::{HashSet, HashMap};
use std::fs;

use itertools::Itertools;

fn parse(path: &str) -> Vec<i64> {
    fs::read_to_string(path).unwrap().lines().map(|line| line.parse().unwrap()).collect()
}


fn next(n: i64) -> i64 {
    let n = ((n * 64) ^ n) % 16777216;
    let n = ((n / 32) ^ n) % 16777216;
    let n = ((n * 2048) ^ n) % 16777216;
    n
}

fn run(start: i64, iterations: usize) -> i64 {
    let mut n = start;
    for _ in 0..iterations {
        n = next(n);
    }
    n
}

fn solve1(numbers: &Vec<i64>) -> i64 {
    numbers.iter().map(|&n| run(n, 2000)).sum()
}

fn solve2(numbers: &Vec<i64>) -> i64 {
    let mut price_per_seq: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for &n in numbers {
        // Only consider the first time we see a sequence for the given monkey
        let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        let mut cur = n;
        let mut a = cur % 10;
        cur = next(cur);
        let mut b = cur % 10;
        cur = next(cur);
        let mut c = cur % 10;
        cur = next(cur);
        let mut d = cur % 10;

        for _ in 0..1996 {
            cur = next(cur);
            let e = cur % 10;
            let seq = (b-a, c-b, d-c, e-d);
            if !seen.contains(&seq) {
                let sum = price_per_seq.get(&seq).unwrap_or(&0) + e;
                price_per_seq.insert(seq, sum);
                seen.insert(seq);
            }
            (a, b, c, d) = (b, c, d, e);
        }
    }

    let best = *price_per_seq.values().sorted_by_key(|&&v| -v).next().unwrap();
    best
}

fn main() {
    let ex = parse("inputs/day22_example.txt");
    let ex2 = parse("inputs/day22_example2.txt");
    let inp = parse("inputs/day22.txt");
    println!("ex1: {}", solve1(&ex));
    println!("ex2: {}", solve2(&ex2));
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}