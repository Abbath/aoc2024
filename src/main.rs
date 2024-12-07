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
    let h = field.len();
    let w = field[0].len();
    let field: Vec<char> = field.into_iter().flatten().collect();
    let pos = field.iter().position(|&c| c == '^').unwrap();
    let crd = (pos / w as usize, pos % w as usize);
    let turn = |d: char| -> char {
        match d {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Wrong direction"),
        }
    };
    let step = |(i, j): (i64, i64), d: char| -> (i64, i64) {
        match d {
            '^' => (i - 1, j),
            'v' => (i + 1, j),
            '>' => (i, j + 1),
            '<' => (i, j - 1),
            _ => panic!("Wrong direction"),
        }
    };
    let solve = |crd: (usize, usize), field: &Vec<char>| -> HashSet<(usize, usize, char)> {
        let mut visited: HashSet<_> = HashSet::new();
        let mut dir = '^';
        visited.insert((crd.0, crd.1, dir));
        let mut i = crd.0 as i64;
        let mut j = crd.1 as i64;
        loop {
            let (ni, nj) = step((i, j), dir);
            let nidx = ni as usize * w + nj as usize;
            if ni < 0 || ni >= h as i64 || nj < 0 || nj >= w as i64 {
                return visited;
            }
            if field[nidx] == '#' {
                dir = turn(dir);
            } else {
                visited.insert((ni as usize, nj as usize, dir));
                (i, j) = (ni, nj);
            }
        }
    };
    let solve2 = |crd: (usize, usize), field: &mut Vec<char>| -> (bool, Vec<(usize, usize)>) {
        let mut corners: Vec<_> = Vec::new();
        let mut dir = '^';
        let mut i = crd.0 as i64;
        let mut j = crd.1 as i64;
        loop {
            let (ni, nj) = step((i, j), dir);
            if ni < 0 || ni >= h as i64 || nj < 0 || nj >= w as i64 {
                return (false, corners);
            }
            let nidx = ni as usize * w + nj as usize;
            if field[nidx] == '.' {
                (i, j) = (ni, nj);
            } else if field[nidx] == dir {
                return (true, corners);
            } else if field[nidx] == '#' {
                let oidx = i as usize * w + j as usize;
                if field[oidx] == '.' {
                    field[oidx] = dir;
                    corners.push((i as usize, j as usize));
                }
                dir = turn(dir);
            } else {
                (i, j) = (ni, nj);
            }
        }
    };
    let v = solve(crd, &field);
    let hs: HashSet<_> =
        HashSet::from_iter(v.iter().filter_map(
            |(i, j, _)| {
                if (*i, *j) != crd {
                    Some((i, j))
                } else {
                    None
                }
            },
        ));
    let mut field2 = field.clone();
    let sum: u64 = hs
        .iter()
        .map(|&(&i, &j)| {
            field2[i * w + j] = '#';
            let (looped, v) = solve2(crd, &mut field2);
            v.iter().for_each(|(i, j)| {
                field2[i * w + j] = '.';
            });
            field2[i * w + j] = '.';
            return looped as u64;
        })
        .sum();
    println!("day06 {} {}", hs.len() + 1, sum);
}

fn day_07() {
    let nums: Vec<(u64, Vec<u64>)> = fs::read_to_string("input/input_07.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<_> = l
                .split_whitespace()
                .map(|s| {
                    if s.ends_with(":") {
                        s[..s.len() - 1].parse::<u64>().unwrap()
                    } else {
                        s.parse::<u64>().unwrap()
                    }
                })
                .collect();
            (v[0], v[1..].to_owned())
        })
        .collect();
    let sum: u64 = nums
        .iter()
        .map(|(n, v)| {
            'outer: for m in 0..2u64.pow(v.len() as u32 - 1) {
                let mut res = v[0];
                for i in 1..v.len() {
                    if (m >> (i - 1)) & 1 == 1 {
                        res *= v[i];
                    } else {
                        res += v[i];
                    }
                    if res > *n {
                        continue 'outer;
                    }
                }
                if res == *n {
                    return *n;
                }
            }
            0
        })
        .sum();
    fn solve(n: u64, v: &Vec<u64>, acc: u64, i: usize) -> u64 {
        if i == v.len() {
            if acc == n {
                return n;
            } else {
                return 0;
            }
        }
        if acc > n {
            return 0;
        }
        let a = solve(n, v, acc + v[i], i + 1);
        if a == n {
            return n;
        }
        let b = solve(n, v, acc * v[i], i + 1);
        if b == n {
            return n;
        }
        let c = solve(
            n,
            v,
            acc * (10u64.pow(v[i].ilog10() as u32 + 1)) + v[i],
            i + 1,
        );
        if c == n {
            n
        } else {
            0
        }
    }
    let sum2: u64 = nums.iter().map(|(n, v)| solve(*n, v, v[0], 1)).sum();
    println!("day07 {sum} {sum2}");
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
}
