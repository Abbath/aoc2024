use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|line| {
            let parts: Vec<_> = line
                .split("   ")
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
    let sum2: i64 = left
        .iter()
        .map(|n| {
            n * right
                .iter()
                .map(|m| if m == n { 1 } else { 0 })
                .sum::<i64>()
        })
        .sum();
    println!("day01 {sum} {sum2}");
}

fn main() {
    day_01();
}
