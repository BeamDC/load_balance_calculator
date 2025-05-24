use std::collections::{HashMap, HashSet};
use std::iter;
use crate::balancer::BalancerState;
use crate::operation::{Operation, ReverseOperation};

pub fn combinations(state: &Vec<u64>, n: u32) -> Vec<Vec<u64>> {
    if n == 0 || n > state.len() as u32 {
        return vec![];
    }

    if n == state.len() as u32 {
        return vec![state.to_vec()];
    }

    if n == 1 {
        return state.iter().map(|x| vec![*x]).collect::<Vec<Vec<u64>>>();
    }

    let mut res = vec![];

    for i in 0..=state.len() as u32 - n {
        let current = state[i as usize];
        let remaining = &state[i as usize + 1..].to_vec();

        for mut sub in combinations(remaining, n - 1) {
            let mut new = vec![current];
            new.append(&mut sub);
            res.push(new);
        }
    }

    res
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    let i = a.trailing_zeros();
    let j = b.trailing_zeros();
    let k:u32;
    if i<j{k=i}
    else {k=j}
    a >>= i;
    b >>= j;
    loop {
        if a > b {
            a^=b;
            b^=a;
            a^=b;
        }
        b -= a;
        if b == 0 {
            return a << k;
        }
        b >>= b.trailing_zeros();
    }
}

pub fn gcd_vec(a: Vec<u64>) -> u64 {
    let mut res = a[0];
    for i in 1..a.len() {
        let val = a[i];
        res = gcd(val, res);
    }
    res
}

pub fn factorize(a: u64) -> u64 {
    todo!("get the prime factorization of a");
}

pub fn multiset(state: Vec<u64>) -> BalancerState {
    let mut state = state;
    // ensure state values are sorted
    state.sort_unstable();
    BalancerState::new(state)
}

pub fn validate_state(state: &BalancerState, gcd: u64) -> bool{
    if state.len() == 0 {
        return false;
    }

    for &val in state.iter() {
        if gcd > val {
            return false
        }
        if val % gcd != 0 {
            return false;
        }
    }

    true
}

pub fn splits(state: &Vec<u64>) -> Vec<(Operation, BalancerState)> {
    fn get_split(n: u64) -> Vec<(Operation, Vec<u64>)> {
        let mut res = vec![];

        let half = n / 2u64;
        let third = n / 3u64;

        let half_split = iter::repeat(half).take(2).collect::<Vec<u64>>();
        let third_split = iter::repeat(third).take(3).collect::<Vec<u64>>();

        res.push((Operation::Split {
            input: n,
            output: (Some(half), Some(half), None)
        }, half_split));

        res.push((Operation::Split {
            input: n,
            output: (Some(third), Some(third), Some(third))
        }, third_split));

        res
    }

    let mut res = vec![];
    let values = state
        .iter()
        .cloned()
        .collect::<HashSet<u64>>()
        .iter()
        .cloned()
        .collect::<Vec<u64>>();

    for val in values {
        for (op, split) in get_split(val) {
            let mut new_state = state.clone().to_vec();
            new_state.remove(new_state.iter().position(|&x| x == val).unwrap());
            new_state.extend(split);
            res.push((op, multiset(new_state)));
        }
    }

    res
}

pub fn merges(state: &Vec<u64>) -> Vec<(Operation, BalancerState)> {
    let mut result = vec![];
    let mut seen = HashSet::new();

    for k in 2..=3 {
        for combination in combinations(&state, k) {
            let combination = multiset(combination);
            if seen.contains(&combination) {
                continue;
            }
            seen.insert(combination.clone());
            let merged: u64 = combination.iter().sum();
            let mut remaining = state.clone();

            for &val in combination.iter() {
                let i = remaining.iter().position(|&x| x == val).unwrap();
                remaining.remove(i);
            }

            remaining.push(merged);

            let new_state = multiset(remaining);

            let operation = match combination.len() {
                2 => {
                    Operation::Merge {
                        input: (Some(combination.values[0]), Some(combination.values[1]), None),
                        output: merged
                    }
                }
                3 => {
                    Operation::Merge {
                        input: (Some(combination.values[0]), Some(combination.values[1]), Some(combination.values[2])),
                        output: merged
                    }
                },
                // this case should never happen
                _ => {Operation::Err},
            };

            result.push((operation, new_state));
        }
    }
    result
}

pub fn rev_splits(state: &Vec<u64>, gcd: u64) -> Vec<(ReverseOperation, BalancerState)> {
    fn split2(n: u64, gcd: u64) -> Vec<(ReverseOperation, Vec<u64>)> {
        let mut res = vec![];

        for x in 1..(n / gcd / 2 + 1) {
            res.push((ReverseOperation::Split {
                input: n,
                output: (Some(x * gcd), Some((n / gcd) * gcd), None)
            }, vec![x * gcd, (n / gcd) * gcd]));
        }

        res
    }

    fn split3(n: u64, gcd: u64) -> Vec<(ReverseOperation, Vec<u64>)> {
        let mut res = vec![];

        for x in 1..(n / gcd / 3 + 1) {
            for (_, vals) in split2(n - x * gcd, gcd) {
                res.push((ReverseOperation::Split {
                    input: n,
                    output: (Some(vals[0]), Some(vals[1]), Some(x * gcd)),
                }, vec![vals[0], vals[1], x * gcd]));
            }
        }

        res
    }

    let mut res = vec![];
    let values = state
        .iter()
        .cloned()
        .collect::<HashSet<u64>>()
        .iter()
        .cloned()
        .collect::<Vec<u64>>();

    for val in values {
        for (op, split) in split2(val, gcd) {
            let mut new_state = state.clone().to_vec();
            new_state.remove(new_state.iter().position(|&x| x == val).unwrap());
            new_state.extend(split);
            res.push((op, multiset(new_state)));
        }
        for (op, split) in split3(val, gcd) {
            let mut new_state = state.clone().to_vec();
            new_state.remove(new_state.iter().position(|&x| x == val).unwrap());
            new_state.extend(split);
            res.push((op, multiset(new_state)));
        }
    }

    res
}

pub fn rev_merges(state: &Vec<u64>) -> Vec<(ReverseOperation, BalancerState)> {
    let mut freqs: HashMap<u64, u64> = HashMap::new();
    let mut result = vec![];

    for val in state.iter() {
        freqs.entry(*val).and_modify(|x| *x += 1).or_insert(1);
    }

    for (k, v) in freqs {
        if v > 1 {
            let mut tmp = state.clone();
            tmp.remove(tmp.iter().position(|x| *x == k).unwrap());
            tmp.remove(tmp.iter().position(|x| *x == k).unwrap());
            tmp.push(k * 2);
            result.push(
                (ReverseOperation::Merge {
                    input: (Some(k), Some(k), None),
                    output: k * 2
                }, multiset(tmp))
            );
        }
        if v > 2 {
            let mut tmp = state.clone();
            tmp.remove(tmp.iter().position(|x| *x == k).unwrap());
            tmp.remove(tmp.iter().position(|x| *x == k).unwrap());
            tmp.remove(tmp.iter().position(|x| *x == k).unwrap());
            tmp.push(k * 3);
            result.push(
                (ReverseOperation::Merge {
                    input: (Some(k), Some(k), Some(k)),
                    output: k * 3
                }, multiset(tmp))
            );
        }
    }

    result
}