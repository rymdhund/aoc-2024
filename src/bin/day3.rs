use std::fs;

use itertools::Itertools;
use regex::Regex;

fn read(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().trim().to_string()
}

fn solve1(data: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(data).map(|c| {
        let (_, [a, b]) = c.extract();
        a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    }).sum()
}

fn solve2(data: &str) -> i32 {
    let new_data = data.split("do()").map(|s: &str| s.split("don't()").next().unwrap()).join("x");
    solve1(&new_data)
}

#[test]
fn test() {
    assert!(solve1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))") == 161);
    assert!(solve2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))") == 48);
}

fn main() {
    let inp = read("inputs/day3.txt");
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}