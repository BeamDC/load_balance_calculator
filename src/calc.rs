use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;
use crate::cmd::Args;
pub fn multiset(state: Vec<u64>) -> Box<[u64]> {
    let mut state = state;
    // ensure state values are sorted
    state.sort_unstable();
    let res = state.into_boxed_slice();
    res
}

// return all n length subsequences in state
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

pub fn splits(n: u64) -> Vec<(String, Vec<u64>)> {
    let half = n / 2u64;
    let third = n / 3u64;

    let half = iter::repeat(half).take(2).collect::<Vec<u64>>();
    let third = iter::repeat(third).take(3).collect::<Vec<u64>>();

    let half_f64 = half.iter().map(|x| *x as f64 / 1e8).collect::<Vec<f64>>();
    let third_f64 = third.iter().map(|x| *x as f64 / 1e8).collect::<Vec<f64>>();
    let n_f64 = n as f64 / 1e8;

    vec![
        (format!("split {} -> {:?}", n_f64, half_f64), half),
        (format!("split {} -> {:?}", n_f64, third_f64), third),
    ]
}

pub fn merges(state: Vec<u64>) -> Vec<(String, Box<[u64]>)> {
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
            let combination_f64 = combination.iter().map(|x| *x as f64 / 1e8).collect::<Vec<f64>>();
            let merged_f64 = merged as f64 / 1e8;

            result.push(
                (format!("merge {:?} -> {}", combination_f64, merged_f64), new_state)
            );
        }
    }
    result
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

    pub fn get_next_states(&self, state: Vec<u64>) -> Vec<(String, Box<[u64]>)> {
        let mut next_states = vec![];

        // splits
        for (i, &val) in state.iter().enumerate() {
            for (action, split) in splits(val) {
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
        for (action, merged) in merges(state) {
            if *merged.iter().max().unwrap_or(&0u64) <= self.max_belt {
                next_states.push((action, multiset(merged.to_vec())));
            }
        }

        next_states
    }

    pub fn find_ideal_balance(&self) -> i32 {
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

        let mut frontier = BinaryHeap::new();

        frontier.push((Reverse(0u64), initial_state.clone()));
        let mut visited = HashMap::new();
        visited.insert(initial_state.clone(), 0u64);

        let mut from: HashMap<Box<[u64]>, (Option<String>, Option<Vec<u64>>)> = HashMap::new();
        from.insert(initial_state.clone(), (None, None));

        while let Some((Reverse(cost), current)) = frontier.pop() {
            // println!("Frontier: {:?}\nWith cost: {:?}", current, cost);
            // Ideal Balance found
            if current == target_state {
                let mut path = vec![];
                let mut state = current.clone();
                while state != initial_state {
                    if let Some((Some(action), Some(prev))) = from.get(&state) {
                        path.push((action.clone(), state.clone()));
                        state = prev.clone().into_boxed_slice();
                    }else {
                        break
                    }
                }
                path.reverse();
                println!("Balancer found in {} steps :", path.len());
                for (i, (action, state)) in path.iter().enumerate() {
                    let state = state.iter().map(|x| *x as f64 / 1e8).collect::<Vec<f64>>();
                    println!("{}. {} => {:?}", i + 1, action, state);
                }

                return path.len() as i32;
            }

            for (action, next) in self.get_next_states(current.clone().to_vec()) {
                let new_cost = cost + 1;
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