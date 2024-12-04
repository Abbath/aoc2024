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
    let line: String = fs::read_to_string("input/input_03.txt").unwrap();
    let mut sum = 0u64;
    let mut sum2 = 0u64;
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
    let mut d1 = 0u64;
    let mut d2 = 0u64;
    let mut enabled = true;
    let read_digits = |i: &usize| -> String {
        line.chars()
            .skip(*i)
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
    };
    while idx < line.len() {
        let c = line.chars().nth(idx).unwrap();
        match state {
            State::S => {
                if c == 'm' {
                    idx -= 1;
                    state = State::M;
                }
                if c == 'd' {
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
                let s = read_digits(&idx);
                if let Ok(d) = s.parse::<u64>() {
                    d1 = d;
                    idx += s.len() - 1;
                    state = State::C;
                } else {
                    state = State::S;
                }
            }
            State::C => {
                if c == ',' {
                    state = State::D2;
                } else {
                    state = State::S;
                }
            }
            State::D2 => {
                let s = read_digits(&idx);
                if let Ok(d) = s.parse::<u64>() {
                    d2 = d;
                    idx += s.len() - 1;
                    state = State::RP;
                } else {
                    state = State::S;
                }
            }
            State::RP => {
                if c == ')' {
                    let pr = d1 * d2;
                    sum += pr;
                    if enabled {
                        sum2 += pr;
                    }
                }
                state = State::S;
            }
        }
        idx += 1;
    }
    println!("day03 {sum} {sum2}");
}

fn day_04() {
    let xmass: Vec<Vec<char>> = fs::read_to_string("input/input_04.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    enum Dir {
        N,
        S,
        W,
        E,
        NW,
        NE,
        SW,
        SE,
    }
    let check = |i: usize, j: usize, dir: Dir| match dir {
        Dir::N => {
            xmass[i][j] == 'X'
                && xmass[i - 1][j] == 'M'
                && xmass[i - 2][j] == 'A'
                && xmass[i - 3][j] == 'S'
        }
        Dir::S => {
            xmass[i][j] == 'X'
                && xmass[i + 1][j] == 'M'
                && xmass[i + 2][j] == 'A'
                && xmass[i + 3][j] == 'S'
        }
        Dir::W => {
            xmass[i][j] == 'X'
                && xmass[i][j - 1] == 'M'
                && xmass[i][j - 2] == 'A'
                && xmass[i][j - 3] == 'S'
        }
        Dir::E => {
            xmass[i][j] == 'X'
                && xmass[i][j + 1] == 'M'
                && xmass[i][j + 2] == 'A'
                && xmass[i][j + 3] == 'S'
        }
        Dir::NW => {
            xmass[i][j] == 'X'
                && xmass[i - 1][j - 1] == 'M'
                && xmass[i - 2][j - 2] == 'A'
                && xmass[i - 3][j - 3] == 'S'
        }
        Dir::NE => {
            xmass[i][j] == 'X'
                && xmass[i - 1][j + 1] == 'M'
                && xmass[i - 2][j + 2] == 'A'
                && xmass[i - 3][j + 3] == 'S'
        }
        Dir::SW => {
            xmass[i][j] == 'X'
                && xmass[i + 1][j - 1] == 'M'
                && xmass[i + 2][j - 2] == 'A'
                && xmass[i + 3][j - 3] == 'S'
        }
        Dir::SE => {
            xmass[i][j] == 'X'
                && xmass[i + 1][j + 1] == 'M'
                && xmass[i + 2][j + 2] == 'A'
                && xmass[i + 3][j + 3] == 'S'
        }
    } as u64;
    let check2 = |i: usize, j: usize| -> u64 {
        (xmass[i][j] == 'A'
            && xmass[i - 1][j - 1] == 'M'
            && xmass[i + 1][j + 1] == 'S'
            && xmass[i - 1][j + 1] == 'M'
            && xmass[i + 1][j - 1] == 'S') as u64
            + (xmass[i][j] == 'A'
                && xmass[i - 1][j - 1] == 'S'
                && xmass[i + 1][j + 1] == 'M'
                && xmass[i - 1][j + 1] == 'M'
                && xmass[i + 1][j - 1] == 'S') as u64
            + (xmass[i][j] == 'A'
                && xmass[i - 1][j - 1] == 'M'
                && xmass[i + 1][j + 1] == 'S'
                && xmass[i - 1][j + 1] == 'S'
                && xmass[i + 1][j - 1] == 'M') as u64
            + (xmass[i][j] == 'A'
                && xmass[i - 1][j - 1] == 'S'
                && xmass[i + 1][j + 1] == 'M'
                && xmass[i - 1][j + 1] == 'S'
                && xmass[i + 1][j - 1] == 'M') as u64
    };
    let mut sum = 0u64;
    for i in 0..xmass.len() {
        for j in 0..xmass[0].len() {
            if i >= 3 {
                sum += check(i, j, Dir::N);
            }
            if i < xmass.len() - 3 {
                sum += check(i, j, Dir::S);
            }
            if j >= 3 {
                sum += check(i, j, Dir::W);
            }
            if j < xmass[0].len() - 3 {
                sum += check(i, j, Dir::E);
            }
            if i >= 3 && j >= 3 {
                sum += check(i, j, Dir::NW);
            }
            if i >= 3 && j < xmass[0].len() - 3 {
                sum += check(i, j, Dir::NE);
            }
            if i < xmass.len() - 3 && j >= 3 {
                sum += check(i, j, Dir::SW);
            }
            if i < xmass.len() - 3 && j < xmass[0].len() - 3 {
                sum += check(i, j, Dir::SE);
            }
        }
    }
    let mut sum2 = 0u64;
    for i in 1..xmass.len() - 1 {
        for j in 1..xmass[0].len() - 1 {
            sum2 += check2(i, j);
        }
    }
    println!("day04 {sum} {sum2}");
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
}
