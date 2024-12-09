use std::fs;

fn parse(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename).unwrap().trim()
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .enumerate()
      .flat_map(|(i, n)| {
        let id = if i % 2 == 0 { (i / 2) as i32 } else { -1 };
        (0..n).map(move |_| id)
    }).collect()
}

fn solve1(inp: &Vec<i32>) -> usize {
    let mut ids: Vec<i32> = inp.clone();

    let mut last = ids.len() - 1;
    for i in 0..ids.len() {
        while ids[last] == -1 {
            last -= 1;
        }
        if i >= last {
            break;
        }
        if ids[i] == -1 {
            ids[i] = ids[last];
            ids[last] = -1;
        }
    }
    checksum(ids)
}

fn solve2(inp: &Vec<i32>) -> usize {
    let mut ids = inp.clone();

    // A little hacky optimization to not have to start at the beginning in the inner loop
    let mut first_space = 0;

    let mut id = *ids.last().unwrap() + 1;
    for j in (0..ids.len()).rev() {
        // find decending id
        if ids[j] >= id || ids[j] == -1 {
            continue;
        }
        id = ids[j];
        let mut j2 = j;
        while j2 > 0 && ids[j2-1] == id {
            j2 -= 1;
        }
        let size = j - j2 + 1;

        let mut found_space = false;
        let mut space_size = 0;
        for i in first_space..ids.len() {
            if i >= j2 {
                break;
            }
            if ids[i] == -1 {
                space_size += 1;
                if !found_space {
                    first_space = i;
                    found_space = true;
                }
            } else {
                space_size = 0;
            }
            if space_size >= size {
                for i2 in 0..size {
                    ids[i - i2] = id;
                    ids[j - i2] = -1;
                }
                break;
            }
        }
    }
    checksum(ids)
}

fn checksum(ids: Vec<i32>) -> usize {
    ids.iter().enumerate().map(|(i, &id)| {
        if id == -1 {
            0
        } else {
            i * (id as usize)
        }
    }).sum()
}

fn main() {
    let ex = parse("inputs/day9_example.txt");
    let inp = parse("inputs/day9.txt");
    println!("ex1: {}", solve1(&ex));
    println!("ex2: {}", solve2(&ex));
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));

}