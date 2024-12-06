use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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
    let xmas = b"XMAS";
    let check = |i: usize, j: usize, dir: Dir| {
        (1..4)
            .map(|n| match dir {
                Dir::N => xmass[i - n][j],
                Dir::S => xmass[i + n][j],
                Dir::W => xmass[i][j - n],
                Dir::E => xmass[i][j + n],
                Dir::NW => xmass[i - n][j - n],
                Dir::NE => xmass[i - n][j + n],
                Dir::SW => xmass[i + n][j - n],
                Dir::SE => xmass[i + n][j + n],
            } as u8 == xmas[n])
            .all(|p| p) as u64
    };
    let ms = b"MS";
    let sm = b"SM";
    let check2 = |i: usize, j: usize| -> u64 {
        let check31 =
            |ms: &[u8; 2]| xmass[i - 1][j - 1] as u8 == ms[0] && xmass[i + 1][j + 1] as u8 == ms[1];
        let check32 =
            |ms: &[u8; 2]| xmass[i - 1][j + 1] as u8 == ms[0] && xmass[i + 1][j - 1] as u8 == ms[1];
        if check31(ms) || check31(sm) {
            (check32(ms) || check32(sm)) as u64
        } else {
            0
        }
    };
    enum Cmp {
        G,
        L(usize),
        N,
    }
    let cmp = |n: usize, c: Cmp| -> bool {
        match c {
            Cmp::G => n >= 3,
            Cmp::L(m) => n < m - 3,
            Cmp::N => true,
        }
    };
    let check4 = |i: usize, j: usize, dir: Dir, c: (Cmp, Cmp)| -> u64 {
        if cmp(i, c.0) && cmp(j, c.1) {
            check(i, j, dir)
        } else {
            0
        }
    };
    let rows = xmass.len();
    let cols = xmass[0].len();
    let sum: u64 = (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| {
                    if xmass[i][j] == 'X' {
                        check4(i, j, Dir::N, (Cmp::G, Cmp::N))
                            + check4(i, j, Dir::S, (Cmp::L(rows), Cmp::N))
                            + check4(i, j, Dir::W, (Cmp::N, Cmp::G))
                            + check4(i, j, Dir::E, (Cmp::N, Cmp::L(cols)))
                            + check4(i, j, Dir::NW, (Cmp::G, Cmp::G))
                            + check4(i, j, Dir::NE, (Cmp::G, Cmp::L(cols)))
                            + check4(i, j, Dir::SW, (Cmp::L(rows), Cmp::G))
                            + check4(i, j, Dir::SE, (Cmp::L(rows), Cmp::L(cols)))
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum();
    let sum2: u64 = (1..rows - 1)
        .map(|i| {
            (1..cols - 1)
                .map(|j| if xmass[i][j] == 'A' { check2(i, j) } else { 0 })
                .sum::<u64>()
        })
        .sum();
    println!("day04 {sum} {sum2}");
}

fn day_05() {
    let (rules, updates): (Vec<_>, Vec<_>) = fs::read_to_string("input/input_05.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .partition(|s| s.contains("|"));
    let rules: HashSet<_> = rules
        .iter()
        .map(|s| {
            let t = s
                .split("|")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (t[0], t[1])
        })
        .collect();
    let updates: Vec<_> = updates
        .iter()
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let (sum, sum2): (u64, u64) = updates
        .iter()
        .map(|u| {
            if u.is_sorted_by(|&a, &b| rules.contains(&(a, b))) {
                (u[u.len() / 2], 0)
            } else {
                let mut mu = u.clone();
                mu.sort_by(|&a, &b| {
                    if rules.contains(&(a, b)) {
                        Ordering::Less
                    } else if rules.contains(&(b, a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                (0, mu[u.len() / 2])
            }
        })
        .fold((0, 0), |(s, s2), (n, n2)| (s + n, s2 + n2));
    println!("day05 {sum} {sum2}");
}

fn day_06() {
    let field: Vec<Vec<char>> = fs::read_to_string("input/input_06.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let crd = field
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if let Some(n) = v.iter().position(|&c| c == '^') {
                Some((i, n))
            } else {
                None
            }
        })
        .nth(0)
        .unwrap();
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Dir {
        N,
        S,
        E,
        W,
    }
    let turn = |d: Dir| -> Dir {
        match d {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    };
    let step = |(i, j): (i64, i64), d: Dir| -> (i64, i64) {
        match d {
            Dir::N => (i - 1, j),
            Dir::S => (i + 1, j),
            Dir::E => (i, j + 1),
            Dir::W => (i, j - 1),
        }
    };
    enum Res {
        Looped,
        NotLooped(HashSet<(usize, usize, Dir)>),
    }
    #[derive(PartialEq)]
    enum Typ {
        Loopy,
        NotLoopy,
    }
    let solve = |crd: (usize, usize), field: &Vec<Vec<char>>, typ: Typ| -> Res {
        let mut visited: HashSet<_> = HashSet::new();
        let mut dir = Dir::N;
        visited.insert((crd.0, crd.1, dir));
        let h = field.len() as i64;
        let w = field[0].len() as i64;
        let mut i = crd.0 as i64;
        let mut j = crd.1 as i64;
        loop {
            let (ni, nj) = step((i, j), dir);
            if ni < 0 || ni >= h || nj < 0 || nj >= w {
                return Res::NotLooped(visited);
            }
            if visited.contains(&(ni as usize, nj as usize, dir)) {
                return Res::Looped;
            }
            if field[ni as usize][nj as usize] == '#' {
                if typ == Typ::Loopy {
                    visited.insert((ni as usize, nj as usize, dir));
                }
                dir = turn(dir);
            } else {
                if typ == Typ::NotLoopy {
                    visited.insert((ni as usize, nj as usize, dir));
                }
                (i, j) = (ni, nj);
            }
        }
    };

    if let Res::NotLooped(v) = solve(crd, &field, Typ::NotLoopy) {
        let hs: HashSet<_> = HashSet::from_iter(v.iter().map(|(i, j, _)| (i, j)));
        let mut field2 = field.clone();
        let sum: u64 = hs
            .iter()
            .map(|&(&i, &j)| {
                if (i, j) == crd {
                    0
                } else {
                    field2[i][j] = '#';
                    if let Res::Looped = solve(crd, &field2, Typ::Loopy) {
                        field2[i][j] = '.';
                        1
                    } else {
                        field2[i][j] = '.';
                        0
                    }
                }
            })
            .sum();
        println!("day06 {} {sum}", hs.len());
    }
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
}
