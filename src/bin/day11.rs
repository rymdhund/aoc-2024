use std::collections::HashMap;

fn solve(inp: Vec<u64>, n: usize) -> usize {
    let mut cache = HashMap::new();
    let mut tot = 0;
    for s in inp {
        tot += count(s, n, &mut cache);
    }
    tot
}

fn count(s: u64, n: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }
    let key = (s, n);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let d = digits(s);
    let res = if s == 0 {
        count(1, n-1, cache)
    } else if d % 2 == 0 {
        let h = pow(d/2);
        count(s / h, n-1, cache) + count(s % h, n-1, cache)
    } else {
        count(s * 2024, n-1, cache)
    };
    cache.insert(key, res);
    res
}

fn digits(mut n: u64) -> usize {
    let mut cnt = 0;
    while n > 0 {
        cnt += 1;
        n /= 10;
    }
    cnt
}

fn pow(n: usize) -> u64{
    let mut res = 1;
    for _ in 0..n {
        res *= 10;
    }
    res
}

fn main() {
    println!("ex1: {}", solve(vec![125, 17], 25));
    println!("sol1: {}", solve(vec![4329, 385, 0, 1444386, 600463, 19, 1, 56615], 25));
    println!("sol2: {}", solve(vec![4329, 385, 0, 1444386, 600463, 19, 1, 56615], 75));
}