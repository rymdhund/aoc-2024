use std::fs;

use itertools::Itertools;

fn parse(filename: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(filename).unwrap().lines().map(|line| {
        let (head, tail): (&str, &str) = line.split(": ").collect_tuple().unwrap();
        let nums = tail.split(" ").map(|n| n.parse::<i64>().unwrap()).collect_vec();
        (head.parse::<i64>().unwrap(), nums)
    }).collect()
}

fn solve1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter().filter_map(|(tot, nums)| {
        is_possible(*tot, &nums[1..], nums[0]).then_some(tot)
    }).sum()
}

fn is_possible(tot: i64, nums: &[i64], acc: i64) -> bool {
    if nums.is_empty() {
        return acc == tot
    }
    is_possible(tot, &nums[1..], acc + nums[0]) ||
        is_possible(tot, &nums[1..], acc * nums[0])
}

fn solve2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter().filter_map(|(tot, nums)| {
        is_possible2(*tot, &nums[1..], nums[0]).then_some(tot)
    }).sum()
}

fn is_possible2(tot: i64, nums: &[i64], acc: i64) -> bool {
    if nums.is_empty() {
        return acc == tot
    }
    is_possible2(tot, &nums[1..], acc + nums[0]) ||
        is_possible2(tot, &nums[1..], acc * nums[0]) ||
        is_possible2(tot, &nums[1..], op(acc, nums[0]))
}

fn op(x: i64, y: i64) -> i64 {
    let mut x1 = x;
    let mut y1 = y;
    while y1 > 0 {
        x1 *= 10;
        y1 /= 10;
    }
    x1 + y
}

fn main() {
    let ex = parse("inputs/day7_example.txt");
    let inp = parse("inputs/day7.txt");
    println!("ex1: {} == 3749", solve1(&ex));
    println!("ex2: {} == 11387", solve2(&ex));
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}