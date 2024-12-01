use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|line| {
            let parts: Vec<_> = line
                .split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .unzip();
    left.sort();
    right.sort();
    let sum: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    let mut hm: HashMap<i64, i64> = HashMap::new();
    right.iter().for_each(|n| {
        *hm.entry(*n).or_default() += 1;
    });
    let sum2: i64 = left.iter().map(|n| n * hm.get(n).unwrap_or(&0)).sum();
    println!("day01 {sum} {sum2}");
}

fn main() {
    let now = Instant::now();
    day_01();
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:?}");
}
