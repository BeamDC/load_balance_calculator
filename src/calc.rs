use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;
use std::time::Instant;
use crate::cmd::Args;
use crate::helpers::{combinations, gcd_vec};
use crate::operation::Operation;

pub fn multiset(state: Vec<u64>) -> Box<[u64]> {
    let mut state = state;
    // ensure state values are sorted
    state.sort_unstable();
    let res = state.into_boxed_slice();
    res
}

pub fn splits(n: u64, gcd: u64) -> Vec<(Operation, Vec<u64>)> {
    let mut res = vec![];

    let half = n / 2u64;
    let third = n / 3u64;

    let half_split = iter::repeat(half).take(2).collect::<Vec<u64>>();
    let third_split = iter::repeat(third).take(3).collect::<Vec<u64>>();

    if half < gcd {
        return res;
    }

    res.push((Operation::Split {
        input: n,
        output: (Some(half), Some(half), None)
    }, half_split));

    if third < gcd {
        return res;
    }

    res.push((Operation::Split {
        input: n,
        output: (Some(third), Some(third), Some(third))
    }, third_split));

    res
}

pub fn merges(state: &Vec<u64>) -> Vec<(Operation, Box<[u64]>)> {
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
                        input: (Some(combination[0]), Some(combination[1]), None),
                        output: merged
                    }
                }
                3 => {
                    Operation::Merge {
                        input: (Some(combination[0]), Some(combination[1]), Some(combination[2])),
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

fn heuristic(state: &Vec<u64>, target: &Vec<u64>, gcd: u64) -> u64 {
    // check how many matches are the same,
    // close splits / merges will yield values close to the original.
    // we want to prioritize states that have more matching values.
    // we also check against the gcd between input and output,
    // since those values can be used universally in
    // creating the target values
    state.iter()
        .zip(target)
        .map(|(&s, &t)| s == t || s == gcd)
        .count() as u64
}

pub enum BalancerResult {
    Error(String),
    Solution {
        end_state: Box<[u64]>,
    }
}

pub struct Balancer {
    max_belt: u64,
    inputs: Vec<u64>,
    outputs: Vec<u64>,
    depth_limit: u64,
}

impl Balancer {
    pub fn new(args: Args) -> Balancer {
        let inputs = args.inputs
            .iter().map(|x| (*x as f64 * 1e8) as u64).collect::<Vec<u64>>();
        let outputs = args.outputs
            .iter().map(|x| (*x as f64 * 1e8) as u64).collect::<Vec<u64>>();

        Balancer {
            inputs,
            outputs,
            max_belt: args.max_belt * 10u64.pow(8),
            depth_limit: 1000,
        }
    }

    pub fn get_next_states(&self, state: Vec<u64>, gcd: u64) -> Vec<(Operation, Box<[u64]>)> {
        let mut next_states = vec![];

        // splits
        for (i, &val) in state.iter().enumerate() {
            for (action, split) in splits(val, gcd) {
                let mut new_state = state[..i].to_vec();
                new_state.extend(state[i + 1..].to_vec());
                new_state.extend(split);
                let new_state = multiset(new_state);
                if *new_state.iter().max().unwrap_or(&0u64) <= self.max_belt {
                    next_states.push((action, new_state));
                }
            }
        }

        // merges
        for (action, merged) in merges(&state) {
            if *merged.iter().max().unwrap_or(&0u64) <= self.max_belt {
                next_states.push((action, multiset(merged.to_vec())));
            }
        }

        next_states
    }

    pub fn find_ideal_balance(&self) -> i32 {
        let start = Instant::now();
        if self.inputs.iter().sum::<u64>() != self.outputs.iter().sum::<u64>() {
            println!("Unbalanced I/O");
            println!("{} != {}",
                     self.inputs.iter().sum::<u64>(),
                     self.outputs.iter().sum::<u64>()
            );
            return -1;
        }

        let initial_state = multiset(self.inputs.clone());
        let target_state = multiset(self.outputs.clone());
        let gcd = gcd_vec(self.outputs.clone());

        let mut frontier = BinaryHeap::new();
        frontier.push((Reverse(0u64), initial_state.clone()));

        let mut visited = HashMap::new();
        visited.insert(initial_state.clone(), 0u64);

        let mut from: HashMap<Box<[u64]>, (Option<Operation>, Option<Vec<u64>>)> = HashMap::new();
        from.insert(initial_state.clone(), (None, None));

        while let Some((Reverse(cost), current)) = frontier.pop() {
            // println!("Frontier: {:?}\nWith cost: {:?}", current, cost);
            // Ideal Balance found
            if current == target_state {
                let mut path = vec![];
                let mut state = current.clone();
                // Backtrack to build the path
                while state != initial_state {
                    if let Some((Some(action), Some(prev))) = from.get(&state) {
                        path.push((action.clone(), state.clone()));
                        state = prev.clone().into_boxed_slice();
                    }else {
                        break
                    }
                }
                println!("Balancer found in {} steps :", path.len());
                for (i, (action, state)) in path.iter().rev().enumerate() {
                    let state = state.iter().map(|x| *x as f64 / 1e8).collect::<Vec<f64>>();
                    println!("{}. {} => {:?}", i + 1, action, state);
                }
                println!("Balancer found in {}ms", start.elapsed().as_millis());
                return path.len() as i32;
            }

            for (action, next) in self.get_next_states(current.clone().to_vec(), gcd) {
                let new_cost = cost + action.cost() +
                    heuristic(&next.to_vec(), &target_state.to_vec(), gcd);

                if visited.get(&next).map_or(true, |&prev_cost| new_cost < prev_cost) {
                    visited.insert(next.clone(), new_cost);
                    from.insert(next.clone(), (Some(action), Some(current.clone().to_vec())));
                    frontier.push((Reverse(new_cost), next));
                }
            }
        }

        println!("No Solution Found :(");
        -1
    }
}