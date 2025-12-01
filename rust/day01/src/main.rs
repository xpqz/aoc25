use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

fn main() {
    let data = read_lines("../../d/1").unwrap();

    let mut data: Vec<i32> = data
        .into_iter()
        .map(|l| {
            let val: i32 = l[1..].parse().unwrap();
            if l.starts_with("L") {
                -val
            } else {
                val
            }
        })
        .collect();

    data.insert(0, 50);

    let r = data
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .filter(|&x| x % 100 == 0)
        .count();

    println!("Part 1: {:#?}", r);

    let (_acc, r) = data.iter().fold((0, 0), |(acc, r), &n| {
        let zeros = (1..=n.abs())
            .map(|x| (acc + x * n.signum()) % 100)
            .filter(|&x| x == 0)
            .count();
        (acc + n, r + zeros)
    });

    println!("Part 2: {:#?}", r);
}
