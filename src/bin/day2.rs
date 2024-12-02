use std::fs;

fn parse(filename: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect()
    }).collect()
}

fn solve1(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|&levels| {
        let dir = levels[1] - levels[0];
        for i in 1..levels.len() {
            if !is_good(levels[i-1], levels[i], dir) {
                return false;
            }
        }
        return true;
    }).count()
}

fn solve2(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|&levels| {
        is_safe(levels, 1) || is_safe(levels, -1)
    }).count()
}

fn is_safe(levels: &Vec<i32>, dir: i32) -> bool {
    match find_fail(&levels[..], dir, 1000000) {
        None => true,
        Some(x) => {
            find_fail(&levels[..], dir, x).is_none() ||
            find_fail(&levels[..], dir, x-1).is_none()
        }
    }
}

fn find_fail(levels: &[i32], dir: i32, skip: usize) -> Option<usize> {
    let mut prev = 0;
    let mut start = 1;
    if skip == 0 {
        prev = 1;
        start = 2;
    }

    for i in start..levels.len() {
        if i != skip {
            if !is_good(levels[prev], levels[i], dir) {
                return Some(i);
            }
            prev = i;
        }
    }
    None
}

fn is_good(a: i32, b: i32, dir: i32) -> bool {
    let diff = b - a;
    diff.abs() <= 3 && diff.abs() >= 1 && (diff < 0) == (dir < 0)
}

fn main() {
    let ex = parse("inputs/day2_example.txt");
    let inp = parse("inputs/day2.txt");
    assert!(solve1(&ex) == 2);
    assert!(solve2(&ex) == 4);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}