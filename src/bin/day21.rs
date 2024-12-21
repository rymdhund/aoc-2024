use std::collections::HashMap;
use aoc2024::coord::{ Coord, Dir };


fn cost_of_seq(seq: &str, costs: &HashMap<(char, char), usize>) -> usize {
    let mut from = 'A';
    let mut sum = 0;
    for to in seq.chars() {
        sum += costs[&(from, to)];
        from = to;
    }
    sum
}

fn create_dir_costs(depth: usize) -> HashMap<(char, char), usize> {
    let chars = vec!['U', 'D', 'L', 'R', 'A'];

    let mut costs: HashMap<(char, char), usize> = HashMap::new();
    // At bottom the cost is 1 for each button
    for &from in chars.iter() {
        for &to in chars.iter() {
            costs.insert((from, to), 1);
        }
    }

    for _ in 1..(depth+1) {
        let mut new_costs: HashMap<(char, char), usize> = HashMap::new();

        for &from in chars.iter() {
            for &to in chars.iter() {
                let seqs = dir_seqs(from, to);

                let cost = seqs.iter().map(|seq| {
                    cost_of_seq(&seq, &costs)
                }).min().unwrap();

                new_costs.insert((from, to), cost);
            }
        }
        costs = new_costs;
    }
    costs
}

// Given a current position and a key you want to press on numpad, find all shortest sequences of instructions that does that
fn num_seqs(from: char, to: char) -> Vec<String> {
    let from = num_to_coord(from);
    let to = num_to_coord(to);

    let mut seqs = seqs_with_avoid(from, to, Coord::new(0, 3));
    seqs.iter_mut().for_each(|seq| seq.push('A'));
    seqs
}

// Given a current position and a key you want to press on direction pad, find all shortest sequences of instructions that does that
fn dir_seqs(from: char, to: char) -> Vec<String> {
    let from = dir_to_coord(from);
    let to = dir_to_coord(to);

    let mut seqs = seqs_with_avoid(from, to, Coord::new(0, 0));
    seqs.iter_mut().for_each(|seq| seq.push('A'));
    seqs
}

// Find all shortest manhattan sequences of instructions to move between two coords that does not pass through a specific coord
fn seqs_with_avoid(from: Coord, to: Coord, avoid: Coord) -> Vec<String> {
    if to == avoid {
        return vec![];
    }
    if from == to {
        return vec!["".to_string()];
    }

    let mut res = vec![];
    if from.x < to.x {
        seqs_with_avoid(from, to.go(Dir::Left), avoid).into_iter().for_each(|mut p| {
            p.push('R');
            res.push(p);
        });
    }
    if from.x > to.x {
        seqs_with_avoid(from, to.go(Dir::Right), avoid).into_iter().for_each(|mut p| {
            p.push('L');
            res.push(p);
        });
    }
    if from.y > to.y {
        seqs_with_avoid(from, to.go(Dir::Down), avoid).into_iter().for_each(|mut p| {
            p.push('U');
            res.push(p);
        });
    }
    if from.y < to.y {
        seqs_with_avoid(from, to.go(Dir::Up), avoid).into_iter().for_each(|mut p| {
            p.push('D');
            res.push(p);
        });
    }
    res
}

fn num_to_coord(c: char) -> Coord {
    match c {
        '7' => Coord::new(0, 0),
        '8' => Coord::new(1, 0),
        '9' => Coord::new(2, 0),
        '4' => Coord::new(0, 1),
        '5' => Coord::new(1, 1),
        '6' => Coord::new(2, 1),
        '1' => Coord::new(0, 2),
        '2' => Coord::new(1, 2),
        '3' => Coord::new(2, 2),
        '0' => Coord::new(1, 3),
        'A' => Coord::new(2, 3),
        _ => panic!("Unexpected char")
    }
}

fn dir_to_coord(c: char) -> Coord {
    match c {
        'U' => Coord::new(1, 0),
        'A' => Coord::new(2, 0),
        'L' => Coord::new(0, 1),
        'D' => Coord::new(1, 1),
        'R' => Coord::new(2, 1),
        _ => panic!("Unexpected dir")
    }
}

fn numpad_cost(seq: &str, dir_costs: &HashMap<(char, char), usize>) -> usize {
    let mut tot = 0;

    let mut from = 'A';
    for to in seq.chars() {
        let seqs = num_seqs(from, to);

        tot += seqs.iter()
            .map(|seq| cost_of_seq(&seq, dir_costs))
            .min().unwrap();

        from = to;
    }
    tot
}

fn solve(numpad_seqs: &Vec<&str>, depth: usize) -> usize {
    let dir_costs = create_dir_costs(depth);

    numpad_seqs.iter().map(|numpad_seq| {
        let cost = numpad_cost(numpad_seq, &dir_costs);
        let value = numpad_seq[..(numpad_seq.len()-1)].parse::<usize>().unwrap();
        cost * value
    }).sum()
}

fn main() {
    let ex = vec![
        "029A",
        "980A",
        "179A",
        "456A",
        "379A",
    ];
    let inp = vec![
        "935A",
        "319A",
        "480A",
        "789A",
        "176A",
    ];
    println!("ex: {}", solve(&ex, 2));
    println!("sol1: {}", solve(&inp, 2));
    println!("sol2: {}", solve(&inp, 25));
}