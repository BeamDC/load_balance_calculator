use crate::balancer::{Balancer, BalancerResult, BalancerState};
use crate::helpers::{gcd, gcd_vec, merges, multiset, splits};
use crate::operation::Operation;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

// balancer related functions
impl Balancer {
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
        // todo: make this better :)
    }

    fn get_next_states(&self, state: Vec<u64>, gcd: u64) -> Vec<(Operation, BalancerState)> {
        let mut next_states = vec![];

        // splits
        for (i, &val) in state.iter().enumerate() {
            for (action, split) in splits(val, gcd) {
                // replace value at i with its split
                let mut new_state = state[..i].to_vec();
                new_state.extend(state[i + 1..].to_vec());
                new_state.extend(split);
                let new_state = multiset(new_state);
                if *new_state.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                    next_states.push((action, new_state));
                }
            }
        }

        // merges
        for (action, merged) in merges(&state) {
            if *merged.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                next_states.push((action, multiset(merged.to_vec())));
            }
        }

        next_states
    }

    pub fn find_ideal_balance(&self) -> BalancerResult {
        let start = Instant::now();
        if self.inputs.iter().sum::<u64>() != self.outputs.iter().sum::<u64>() {
            println!("Unbalanced I/O");
            println!("{} in != {} out",
                     self.inputs.iter().sum::<u64>(),
                     self.outputs.iter().sum::<u64>()
            );
            return BalancerResult::default();
        }

        let initial_state = multiset(self.inputs.clone());
        let target_state = multiset(self.outputs.clone());
        let gcd = gcd(
            gcd_vec(self.outputs.clone()), gcd_vec(self.inputs.clone())
        );

        let mut frontier = BinaryHeap::new();
        frontier.push((Reverse(0u64), initial_state.clone()));

        let mut visited = HashMap::new();
        visited.insert(initial_state.clone(), 0u64);

        let mut from: HashMap<BalancerState, (Option<Operation>, Option<Vec<u64>>)> = HashMap::new();
        from.insert(initial_state.clone(), (None, None));

        let mut states_checked = 0u64;
        let mut total_states = 0u64;
        while let Some((Reverse(cost), current)) = frontier.pop() {
            states_checked += 1;

            // Ideal Balance found
            if current == target_state {
                let mut path = vec![];
                let mut state = current.clone();
                // Backtrack to build the path
                while state != initial_state {
                    if let Some((Some(action), Some(prev))) = from.get(&state) {
                        path.push((action.clone(), state.clone()));
                        state = BalancerState::new(prev.clone());
                        continue
                    }
                    break
                }
                path.reverse();
                println!("Balancer found in {}ms", start.elapsed().as_millis());
                println!(
                    "{} of {} states checked ({:.3}%)",
                    states_checked, total_states,
                    states_checked as f64 / total_states as f64 * 100.0
                );
                return BalancerResult::new(path);
            }

            let next_states = self.get_next_states(current.clone().to_vec(), gcd);
            total_states += next_states.len() as u64;

            for (action, next) in next_states.iter() {
                let new_cost = cost + action.cost() +
                    Balancer::heuristic(&next.to_vec(), &target_state.to_vec(), gcd);

                if visited.get(&next).map_or(true, |&prev_cost| new_cost < prev_cost) {
                    visited.insert(next.clone(), new_cost);
                    from.insert(next.clone(), (Some(action.clone()), Some(current.clone().to_vec())));
                    frontier.push((Reverse(new_cost), next.clone()));
                }
            }
        }

        println!("No Solution Found :(");
        BalancerResult::default()
    }
}