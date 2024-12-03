use std::collections::HashMap;
use std::fs;

fn day_01() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = fs::read_to_string("input/input_01.txt")
        .unwrap()
        .lines()
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
    let sum: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, &b)| a.abs_diff(b))
        .sum();
    let mut hm: HashMap<i64, i64> = HashMap::new();
    right.iter().for_each(|&n| {
        *hm.entry(n).or_default() += 1;
    });
    let sum2: i64 = left.iter().map(|n| n * hm.get(n).unwrap_or(&0)).sum();
    println!("day01 {sum} {sum2}");
}

fn day_02() {
    let nums: Vec<Vec<_>> = fs::read_to_string("input/input_02.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let check_safety = |ns: &Vec<i64>| {
        (ns.is_sorted_by(|a, b| a < b && b - a < 4) || ns.is_sorted_by(|a, b| a > b && a - b < 4))
            as i64
    };
    let check_safety2 = |ns: &Vec<i64>, check: fn(i64, i64) -> bool| {
        let v: Vec<_> = ns.windows(2).map(|p| check(p[0], p[1])).collect();
        let c1 = v.windows(2).filter(|p| !(p[0] || p[1])).count() == 1;
        let c2 = v.iter().filter(|&p| !p).count();
        if c1 && c2 == 2 {
            let idx = v.iter().position(|p| !p).unwrap();
            if check(ns[idx], ns[idx + 2]) {
                return true;
            }
        }
        if c2 == 1 {
            let idx = v.iter().position(|p| !p).unwrap();
            if idx == 0 && !check(ns[idx], ns[idx + 1])
                || idx == ns.len() - 2 && !check(ns[idx], ns[idx + 1])
                || idx < ns.len() - 2 && check(ns[idx], ns[idx + 2])
                || idx > 0 && check(ns[idx - 1], ns[idx + 1])
            {
                return true;
            }
        }
        false
    };
    let sum: i64 = nums.iter().map(check_safety).sum();
    let sum2: i64 = nums
        .iter()
        .map(|ns| {
            if check_safety(ns) == 1 {
                1
            } else {
                (check_safety2(ns, |a, b| a < b && b - a < 4)
                    || check_safety2(ns, |a, b| a > b && a - b < 4)) as i64
            }
        })
        .sum();
    println!("day02 {sum} {sum2}");
}

fn day_03() {
    let lines: Vec<String> = fs::read_to_string("input/input_03.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    fn compute(line: &str, enabling: bool, old_enabled: bool) -> (u64, bool) {
        enum State {
            S,
            D,
            M,
            D1,
            C,
            D2,
            RP,
        }
        let mut state = State::S;
        let mut idx = 0usize;
        let mut d1s: Vec<char> = Vec::new();
        let mut d2s: Vec<char> = Vec::new();
        let mut sum = 0u64;
        let mut enabled = old_enabled;
        loop {
            if idx >= line.len() {
                break;
            }
            let c = line.chars().nth(idx).unwrap();
            match state {
                State::S => {
                    if (!enabling || enabled) && c == 'm' {
                        idx -= 1;
                        state = State::M;
                    }
                    if enabling && c == 'd' {
                        idx -= 1;
                        state = State::D;
                    }
                }
                State::D => {
                    if idx + 4 <= line.len() && line[idx..idx + 4] == *"do()" {
                        idx += 3;
                        enabled = true;
                    }
                    if idx + 7 <= line.len() && line[idx..idx + 7] == *"don't()" {
                        idx += 6;
                        enabled = false;
                    }
                    state = State::S;
                }
                State::M => {
                    if idx + 4 <= line.len() && line[idx..idx + 4] == *"mul(" {
                        idx += 3;
                        state = State::D1;
                    } else {
                        state = State::S;
                    }
                }
                State::D1 => {
                    if c.is_ascii_digit() {
                        d1s.push(c);
                    } else {
                        idx -= 1;
                        state = State::C;
                    }
                }
                State::C => {
                    if c == ',' {
                        state = State::D2;
                    } else {
                        d1s.clear();
                        state = State::S;
                    }
                }
                State::D2 => {
                    if c.is_ascii_digit() {
                        d2s.push(c)
                    } else {
                        idx -= 1;
                        state = State::RP;
                    }
                }
                State::RP => {
                    if c == ')' {
                        let d1: u64 = d1s.iter().collect::<String>().parse().unwrap();
                        let d2: u64 = d2s.iter().collect::<String>().parse().unwrap();
                        sum += d1 * d2;
                    }
                    d1s.clear();
                    d2s.clear();
                    state = State::S;
                }
            }
            idx += 1;
        }
        (sum, enabled)
    }
    let sum: u64 = lines.iter().map(|l| compute(l, false, true).0).sum();
    let sum2: u64 = lines
        .iter()
        .fold((0u64, true), |(s, e), l| {
            let (ns, ne) = compute(l, true, e);
            (s + ns, ne)
        })
        .0;
    println!("day03 {sum} {sum2}");
}

fn main() {
    day_01();
    day_02();
    day_03();
}
