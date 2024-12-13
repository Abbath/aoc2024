use std::cmp::Ordering;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

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
                    idx = idx.saturating_sub(1);
                    state = State::M;
                }
                if c == 'd' {
                    idx = idx.saturating_sub(1);
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
            looped as u64
        })
        .sum();
    println!("day06 {} {}", hs.len() + 1, sum);
}

fn day_07() {
    fn solve2(c: bool, n: u64, v: &[u64]) -> u64 {
        let mut stack = Vec::with_capacity(25);
        stack.push((v[0], 1));
        loop {
            if let Some((a, i)) = stack.pop() {
                if i == v.len() {
                    if a == n {
                        return n;
                    } else {
                        continue;
                    }
                }
                if a > n {
                    continue;
                }
                stack.push((a + v[i], i + 1));
                stack.push((a * v[i], i + 1));
                if c {
                    stack.push((a * (10u64.pow(v[i].ilog10() + 1)) + v[i], i + 1));
                }
            } else {
                return 0;
            }
        }
    }
    let (sum, sum2): (u64, u64) = fs::read_to_string("input/input_07.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<_> = l
                .split_whitespace()
                .map(|s| {
                    (if let Some(st) = s.strip_suffix(":") {
                        st
                    } else {
                        s
                    })
                    .parse::<u64>()
                    .unwrap()
                })
                .collect();
            (v[0], v[1..].to_owned())
        })
        .map(|(n, v)| (solve2(false, n, &v), solve2(true, n, &v)))
        .fold((0, 0), |(s, s2), (r, r2)| (s + r, s2 + r2));
    println!("day07 {sum} {sum2}");
}

fn day_08() {
    let lines: Vec<_> = fs::read_to_string("input/input_08.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let h = lines.len() as i64;
    let w = lines[0].len() as i64;
    let mut hm: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    (0..h).for_each(|i| {
        (0..w).for_each(|j| {
            let c = lines[i as usize][j as usize];
            if c != '.' {
                hm.entry(c)
                    .and_modify(|v| v.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        });
    });
    let mut hs: HashSet<(i64, i64)> = HashSet::new();
    let mut hs2: HashSet<(i64, i64)> = HashSet::new();
    hm.values().for_each(|v| {
        (0..v.len() as i64).for_each(|i| {
            (i + 1..v.len() as i64).for_each(|j| {
                let a = v[i as usize];
                let b = v[j as usize];
                let (dx, dy) = (b.1 - a.1, b.0 - a.0);
                let mut k = 1;
                loop {
                    let an = (a.0 - dy * k, a.1 - dx * k);
                    let bn = (b.0 + dy * k, b.1 + dx * k);
                    let p1 = if an.0 >= 0 && an.0 < h && an.1 >= 0 && an.1 < w {
                        if k == 1 {
                            hs.insert(an);
                        }
                        hs2.insert(an);
                        true
                    } else {
                        false
                    };
                    let p2 = if bn.0 >= 0 && bn.0 < h && bn.1 >= 0 && bn.1 < w {
                        if k == 1 {
                            hs.insert(bn);
                        }
                        hs2.insert(bn);
                        true
                    } else {
                        false
                    };
                    if !(p1 || p2) {
                        break;
                    }
                    k += 1;
                }
            })
        })
    });
    let sum2: usize = hm
        .values()
        .map(|v| v.iter().map(|x| !hs2.contains(x) as usize).sum::<usize>())
        .sum();
    println!("day08 {} {}", hs.len(), hs2.len() + sum2);
}

fn day_09() {
    let nums: Vec<_> = fs::read_to_string("input/input_09.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let files: Vec<_> = nums.iter().step_by(2).collect();
    let free_spaces: Vec<_> = nums.iter().skip(1).step_by(2).collect();
    let len: usize = nums.iter().sum::<u32>() as usize;
    let mut v: Vec<i64> = vec![-1; len];
    let mut idx = 0usize;
    let mut fs: Vec<(usize, usize)> = Vec::new();
    let mut fss: Vec<(usize, usize)> = Vec::new();
    files.iter().enumerate().for_each(|(i, &n)| {
        fs.push((idx, *n as usize));
        (0..*n).for_each(|_| {
            v[idx] = i as i64;
            idx += 1;
        });
        if i != files.len() - 1 {
            fss.push((idx, *free_spaces[i] as usize));
            idx += *free_spaces[i] as usize;
        }
    });
    let mut v2 = v.clone();
    idx = 0;
    let mut ridx = v.len() - 1;
    loop {
        while v[idx] != -1 {
            idx += 1;
        }
        while v[ridx] == -1 {
            ridx -= 1;
        }
        if idx >= ridx {
            break;
        }
        v[idx] = v[ridx];
        v[ridx] = -1;
    }
    let sum: i64 = v
        .iter()
        .enumerate()
        .map(|(i, &x)| if x < 0 { 0 } else { x * i as i64 })
        .sum();
    let mut cache: [usize; 9] = [0; 9];
    fs.iter().rev().for_each(|&(pos, len)| {
        for (i, item) in fss.iter_mut().enumerate().skip(cache[len - 1]) {
            let &mut (spos, slen) = item;
            if spos >= pos {
                break;
            }
            if slen >= len {
                (0..len).for_each(|n| {
                    v2[spos + n] = v2[pos + n];
                    v2[pos + n] = -1;
                });
                *item = (spos + len, slen - len);
                cache[len - 1] = i;
                break;
            }
        }
    });
    let sum2: i64 = v2
        .iter()
        .enumerate()
        .map(|(i, &x)| if x < 0 { 0 } else { x * i as i64 })
        .sum();
    println!("day09 {sum} {sum2}");
}

fn day_10() {
    let nums: Vec<Vec<_>> = fs::read_to_string("input/input_10.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();
    fn solve(nums: &[Vec<u64>], i: usize, j: usize) -> (u64, u64) {
        let mut stack = Vec::with_capacity(10);
        let h = nums.len();
        let w = nums[0].len();
        stack.push((i, j));
        let mut sum = 0;
        let mut sum2 = 0;
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        loop {
            if let Some((i, j)) = stack.pop() {
                if nums[i][j] == 9 {
                    if !hs.contains(&(i, j)) {
                        sum += 1;
                        hs.insert((i, j));
                    }
                    sum2 += 1;
                } else {
                    let val = nums[i][j];
                    if i > 0 && nums[i - 1][j] - val == 1 {
                        stack.push((i - 1, j));
                    }
                    if j > 0 && nums[i][j - 1] - val == 1 {
                        stack.push((i, j - 1));
                    }
                    if i < h - 1 && nums[i + 1][j] - val == 1 {
                        stack.push((i + 1, j));
                    }
                    if j < w - 1 && nums[i][j + 1] - val == 1 {
                        stack.push((i, j + 1));
                    }
                }
            } else {
                return (sum, sum2);
            }
        }
    }
    let (sum, sum2): (u64, u64) = (0..nums.len())
        .map(|i| {
            (0..nums[i].len())
                .map(|j| {
                    if nums[i][j] == 0 {
                        solve(&nums, i, j)
                    } else {
                        (0, 0)
                    }
                })
                .fold((0, 0), |(s, s2), (n, n2)| (s + n, s2 + n2))
        })
        .fold((0, 0), |(s, s2), (n, n2)| (s + n, s2 + n2));
    println!("day10 {sum} {sum2}");
}

fn day_11() {
    let mut nums: HashMap<u64, u64> = fs::read_to_string("input/input_11.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect();
    let mut sum = 0u64;
    (0..75).for_each(|i| {
        let mut new_nums: HashMap<u64, u64> = HashMap::new();
        nums.iter().for_each(|(&n, &k)| {
            if n == 0 {
                new_nums.entry(1).and_modify(|x| *x += k).or_insert(k);
            } else if n.ilog10() % 2 == 1 {
                let m = (n.ilog10() + 1) / 2;
                new_nums
                    .entry(n / 10u64.pow(m))
                    .and_modify(|x| *x += k)
                    .or_insert(k);
                new_nums
                    .entry(n % 10u64.pow(m))
                    .and_modify(|x| *x += k)
                    .or_insert(k);
            } else {
                new_nums
                    .entry(n * 2024)
                    .and_modify(|x| *x += k)
                    .or_insert(k);
            }
        });
        nums = new_nums;
        if i == 25 {
            sum = nums.iter().map(|(&_, &k)| k).sum::<u64>();
        }
    });
    let sum2 = nums.iter().map(|(&_, &k)| k).sum::<u64>();
    println!("day11 {sum} {sum2}");
}

fn day_12() {
    let field: Vec<Vec<_>> = fs::read_to_string("input/input_12.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let h = field.len();
    let w = field[0].len();
    let is_hor = |(a, _): (usize, usize), (c, _): (usize, usize)| a == c;
    let angle = |x1: i64, y1: i64, x2: i64, y2: i64| {
        ((x1 * y2 - y1 * x2) as f64).atan2((x1 * x2 + y1 * y2) as f64)
    };
    let select_closest = |(a, b): (usize, usize), (c, d): (usize, usize), v: &[(usize, usize)]| {
        let v1 = (c as i64 - a as i64, d as i64 - b as i64);
        *v.iter()
            .min_by(|&x, &y| {
                let v2 = (x.0 as i64 - c as i64, x.1 as i64 - d as i64);
                let v3 = (y.0 as i64 - c as i64, y.1 as i64 - d as i64);
                angle(v1.0, v1.1, v2.0, v2.1).total_cmp(&angle(v1.0, v1.1, v3.0, v3.1))
            })
            .unwrap()
    };
    let find =
        |f: &[Vec<char>], fnd: &mut Vec<Vec<bool>>, i: usize, j: usize, c: char| -> (u64, u64) {
            let mut hs: HashSet<(usize, usize)> = HashSet::new();
            hs.insert((i, j));
            let mut stack = Vec::with_capacity(100);
            stack.push((i, j));
            let mut sum_a = 1u64;
            let mut edges = BTreeMap::<(usize, usize), Vec<(usize, usize)>>::new();
            loop {
                if let Some((i, j)) = stack.pop() {
                    if i > 0 && f[i - 1][j] == c && !fnd[i - 1][j] {
                        fnd[i - 1][j] = true;
                        sum_a += 1;
                        stack.push((i - 1, j));
                    } else if i == 0 || f[i - 1][j] != c {
                        edges.entry((i, j)).or_default().push((i, j + 1));
                    }
                    if j > 0 && f[i][j - 1] == c && !fnd[i][j - 1] {
                        fnd[i][j - 1] = true;
                        sum_a += 1;
                        stack.push((i, j - 1));
                    } else if j == 0 || f[i][j - 1] != c {
                        edges.entry((i + 1, j)).or_default().push((i, j));
                    }
                    if i < h - 1 && f[i + 1][j] == c && !fnd[i + 1][j] {
                        fnd[i + 1][j] = true;
                        sum_a += 1;
                        stack.push((i + 1, j));
                    } else if i == h - 1 || f[i + 1][j] != c {
                        edges.entry((i + 1, j + 1)).or_default().push((i + 1, j));
                    }
                    if j < w - 1 && f[i][j + 1] == c && !fnd[i][j + 1] {
                        fnd[i][j + 1] = true;
                        sum_a += 1;
                        stack.push((i, j + 1));
                    } else if j == w - 1 || f[i][j + 1] != c {
                        edges.entry((i, j + 1)).or_default().push((i + 1, j + 1));
                    }
                } else {
                    let mut tr = 0u64;
                    let mut hs = HashSet::<((usize, usize), (usize, usize))>::new();
                    let len = edges.values().map(|v| v.len()).sum::<usize>();
                    while len != hs.len() {
                        let ses = edges
                            .iter()
                            .find(|&(&k, v)| !hs.contains(&(k, v[0])))
                            .unwrap();
                        let &os = ses.0;
                        let oe = ses.1[0];
                        let mut ih = is_hor(os, oe);
                        let oih = ih;
                        let mut s = os;
                        let mut e = oe;
                        hs.insert((os, oe));
                        loop {
                            let es = edges[&e].clone();
                            let s0 = e;
                            e = if es.len() == 1 {
                                es[0]
                            } else {
                                select_closest(s, e, &es)
                            };
                            s = s0;
                            hs.insert((s, e));
                            if os == s && oe == e {
                                if ih != oih {
                                    tr += 1;
                                }
                                break;
                            }
                            let d = is_hor(s, e);
                            if d != ih {
                                tr += 1;
                                ih = d;
                            }
                        }
                    }
                    return (sum_a * len as u64, sum_a * tr);
                }
            }
        };
    let mut found: Vec<Vec<bool>> = field
        .iter()
        .map(|v| v.iter().map(|_| false).collect())
        .collect();
    let (sum, sum2): (u64, u64) = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| {
                    if !found[i][j] {
                        found[i][j] = true;
                        find(&field, &mut found, i, j, field[i][j])
                    } else {
                        (0, 0)
                    }
                })
                .fold((0, 0), |(s, s2), (n, n2)| (s + n, s2 + n2))
        })
        .fold((0, 0), |(s, s2), (n, n2)| (s + n, s2 + n2));

    println!("day12 {sum} {sum2}");
}

fn day_13() {
    let lines: Vec<_> = fs::read_to_string("input/test.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let mut tasks: Vec<((u64, u64), (u64, u64), (u64, u64))> = Vec::new();
    let mut a = (0u64, 0u64);
    let mut b = (0u64, 0u64);
    lines.iter().for_each(|l| {
        if l.len() > 0 {
            let parts: Vec<_> = l.split_whitespace().collect();
            if parts[1].starts_with("A") {
                a = (
                    parts[2]
                        .split("+")
                        .nth(1)
                        .unwrap()
                        .strip_suffix(",")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    parts[3].split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                );
            }
            if parts[1].starts_with("B") {
                b = (
                    parts[2]
                        .split("+")
                        .nth(1)
                        .unwrap()
                        .strip_suffix(",")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    parts[3].split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                );
            }
            if parts[1].starts_with("X") {
                let c = (
                    parts[1]
                        .split("=")
                        .nth(1)
                        .unwrap()
                        .strip_suffix(",")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    parts[2].split("=").nth(1).unwrap().parse::<u64>().unwrap(),
                );
                tasks.push((a, b, c));
            }
        }
    });
    tasks.iter().for_each(|(a, b, c)| {
        let target = c;
        let mut stack: Vec<(u64, u64)> = Vec::with_capacity(100);
        stack.push((0, 0));
        loop {
            if let Some((i, j)) = stack.pop() {
                let price = i * 3 + j;
                let x = a.0 * i + b.0 * j;
                let y = a.1 * i + b.1 * j;
                //println!("{x} {y} {price}");
                if (x, y) == *target {
                    println!("FOUND {price}");
                }
                if x < target.0 && y < target.1 {
                    if i < 100 {
                        stack.push((i + 1, j));
                    }
                    if j < 100 {
                        stack.push((i, j + 1));
                    }
                }
            } else {
                break;
            }
        }
    });
    println!("{tasks:?}");
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
    day_09();
    day_10();
    day_11();
    day_12();
    day_13();
}
