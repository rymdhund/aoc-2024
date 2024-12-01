use std::fs;

fn parse(filename: &str) -> (Vec<i32>, Vec<i32>) {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        let parts: Vec<&str> = line.split("   ").collect();
        (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
    }).unzip()
}

fn solve1(lines: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut xs, mut ys) = lines.clone();
    xs.sort();
    ys.sort();
    xs.iter().zip(ys.iter()).map(|(x, y)| (x-y).abs()).sum()
}

fn solve2(lines: &(Vec<i32>, Vec<i32>)) -> usize {
    let (a, b) = lines.clone();
    a.iter().map(|&x| {
        let cnt = b.iter().filter(|&&y| y == x).count();
        usize::try_from(x).unwrap() * cnt
    }).sum()
}


fn main() {
    let ex = parse("inputs/day1_example.txt");
    let inp = parse("inputs/day1.txt");
    assert!(solve1(&ex) == 11);
    assert!(solve2(&ex) == 31);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}