use crate::balancer::{Balancer, BalancerResult, BalancerState};
use crate::helpers::{gcd, gcd_vec, merges, multiset, rev_merges, rev_splits, splits, validate_state};
use crate::operation::{Operation, ReverseOperation};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

// balancer related functions
impl Balancer {
    fn heuristic(state: &Vec<u64>, target: &Vec<u64>, gcd: u64) -> i64 {
        let mut state_counts = HashMap::new();
        let mut target_counts = HashMap::new();

        for &val in state {
            *state_counts.entry(val).or_insert(0) += 1;
        }
        for &val in target {
            *target_counts.entry(val).or_insert(0) += 1;
        }

        let mut estimated_cost = 0i64;

        // For each target value, estimate cost to create it
        for (&target_val, &needed) in &target_counts {
            let available = state_counts.get(&target_val).copied().unwrap_or(0);
            if available < needed {
                let deficit = needed - available;
                let creation_cost = if target_val % gcd == 0 {
                    3
                } else if state.iter().any(|&s| s > target_val && s % target_val == 0) {
                    3 // Can be easily split from existing value
                } else if state.iter().any(|&s| target_val % s == 0) {
                    2 // Factor of target_val
                } else {
                    1
                };
                estimated_cost += deficit * creation_cost;
            }
        }

        // too many values in state
        if state.len() > target.len() {
            estimated_cost += (state.len() - target.len()) as i64 / 2;
        }

        estimated_cost
    }

    fn get_states_fwd(&self, state: &Vec<u64>, gcd: u64) -> Vec<(Operation, BalancerState)> {
        let mut next_states = vec![];

        let split_states = splits(&state.clone())
            .iter()
            .filter(|(_, state)| validate_state(state, gcd))
            .cloned()
            .collect::<Vec<(Operation, BalancerState)>>();

        let merge_states = merges(&state)
            .iter()
            .filter(|(_, state)| validate_state(state, gcd))
            .cloned()
            .collect::<Vec<(Operation, BalancerState)>>();

        // splits
        for (action, split) in split_states {
            if *split.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                next_states.push((action, split));
            }
        }

        // merges
        for (action, merged) in merge_states {
            if *merged.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                next_states.push((action, multiset(merged.to_vec())));
            }
        }

        next_states
    }

    fn get_states_bkwd(&self, state: &Vec<u64>, gcd: u64) -> Vec<(ReverseOperation, BalancerState)> {
        let mut next_states = vec![];

        let split_states = rev_splits(&state, gcd)
            .iter()
            .filter(|(_, state)| validate_state(state, gcd))
            .cloned()
            .collect::<Vec<(ReverseOperation, BalancerState)>>();

        let merge_states = rev_merges(&state)
            .iter()
            .filter(|(_, state)| validate_state(state, gcd))
            .cloned()
            .collect::<Vec<(ReverseOperation, BalancerState)>>();

        // splits
        for (action, split) in split_states {
            if *split.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                next_states.push((action, multiset(split.to_vec())))
            }
        }

        // merges
        for (action, merged) in merge_states {
            if *merged.iter().max().unwrap_or(&(self.max_belt + 1)) <= self.max_belt {
                next_states.push((action, multiset(merged.to_vec())));
            }
        }

        next_states
    }

    pub fn build_path(
        meeting_point: &BalancerState,
        initial_state: &BalancerState,
        target_state: &BalancerState,
        from_fwd: HashMap<BalancerState, (Option<Operation>, Option<Vec<u64>>)>,
        from_bkwd: HashMap<BalancerState, (Option<ReverseOperation>, Option<Vec<u64>>)>,
        total_states: u64,
        checked_states: u64,
        time: f64,
    ) -> BalancerResult
    {
        let mut path = vec![];

        let mut current = meeting_point.clone();
        let mut fwd = vec![];

        while current != *initial_state {
            if let Some((Some(op), Some(parent))) = from_fwd.get(&current) {
                fwd.push((op.clone(), current.clone()));
                current = multiset(parent.clone());
                continue;
            }
            break;
        }
        fwd.reverse();
        path.extend(fwd);

        current = meeting_point.clone();
        while current != *target_state {
            if let Some((Some(op), Some(parent))) = from_bkwd.get(&current) {
                path.push((op.clone().forward(), multiset(parent.clone())));
                current = multiset(parent.clone());
                continue;
            }
            break;
        }

        BalancerResult::new(path, total_states, checked_states, time)
    }

    pub fn find_ideal_balance(&self) -> BalancerResult {
        let start = Instant::now();
        if self.inputs.iter().sum::<u64>() != self.outputs.iter().sum::<u64>() {
            println!("Unbalanced I/O. Kick rocks kid");
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

        /// forward traversal data
        let initial_h = Self::heuristic(&initial_state.to_vec(), &target_state.to_vec(), gcd);

        let mut frontier_fwd = BinaryHeap::new();
        frontier_fwd.push((Reverse(initial_h), 0i64, initial_state.clone()));

        let mut cost_fwd = HashMap::new();
        cost_fwd.insert(initial_state.clone(), 0i64);

        let mut from_fwd: HashMap<BalancerState, (Option<Operation>, Option<Vec<u64>>)> = HashMap::new();
        from_fwd.insert(initial_state.clone(), (None, None));

        /// reverse traversal data
        let target_h = Self::heuristic(&target_state.to_vec(), &initial_state.to_vec(), gcd);

        let mut frontier_bkwd = BinaryHeap::new();
        frontier_bkwd.push((Reverse(target_h), 0i64, target_state.clone()));

        let mut cost_bkwd = HashMap::new();
        cost_bkwd.insert(target_state.clone(), 0i64);

        let mut from_bkwd: HashMap<BalancerState, (Option<ReverseOperation>, Option<Vec<u64>>)> = HashMap::new();
        from_bkwd.insert(target_state.clone(), (None, None));

        /// main traversal loop
        let mut states_checked = 0u64;
        let mut total_states = 0u64;
        let mut fwd = true;
        let mut best_confidence = i64::MAX;
        let mut meeting_point: Option<BalancerState> = None;

        while (!frontier_fwd.is_empty() || !frontier_bkwd.is_empty()) &&
            start.elapsed().as_secs() < 30 {
            states_checked += 1;

            if fwd && !frontier_fwd.is_empty(){
                let (Reverse(_priority), cost, current) = frontier_fwd.pop().unwrap();

                if let Some(&rev_cost) = cost_bkwd.get(&current) {
                    let total_cost = cost + rev_cost;
                    if total_cost < best_confidence {
                        best_confidence = total_cost;
                        meeting_point = Some(current.clone());
                    }
                }

                // get next forward states
                let next_states = self.get_states_fwd(&current.clone().to_vec(), gcd);
                total_states += next_states.len() as u64;

                for (action, next) in next_states {
                    let new_cost = cost + action.cost();

                    if cost_fwd.get(&next).map_or(true, |&prev_cost| new_cost < prev_cost) {
                        cost_fwd.insert(next.clone(), new_cost);
                        from_fwd.insert(next.clone(), (Some(action.clone()), Some(current.clone().to_vec())));

                        let confidence = Self::heuristic(&next.to_vec(), &target_state.to_vec(), gcd) + cost;
                        frontier_fwd.push((Reverse(confidence), new_cost, next.clone()));
                    }
                }
            } else if !frontier_bkwd.is_empty(){
                let (Reverse(_priority), cost, current) = frontier_bkwd.pop().unwrap();

                if let Some(&fwd_cost) = cost_fwd.get(&current) {
                    let total_confidence = cost + fwd_cost;
                    if total_confidence < best_confidence {
                        best_confidence = total_confidence;
                        meeting_point = Some(current.clone());
                    }
                }

                // get next backward states
                let next_states = self.get_states_bkwd(&current.clone().to_vec(), gcd);
                total_states += next_states.len() as u64;
                for (action, next) in next_states {
                    let new_cost = cost + action.cost();

                    if cost_bkwd.get(&next).map_or(true, |&prev_cost| new_cost < prev_cost) {
                        cost_bkwd.insert(next.clone(), new_cost);
                        from_bkwd.insert(next.clone(), (Some(action.clone()), Some(current.clone().to_vec())));

                        let confidence = Self::heuristic(&next.to_vec(), &initial_state.to_vec(), gcd) + cost;
                        frontier_bkwd.push((Reverse(confidence), new_cost, next.clone()));
                    }
                }
            }

            //
            if meeting_point.is_some() {
                break;
            }

            fwd = !fwd;
        }

        if let Some(meeting) = meeting_point {
            let path = Self::build_path(
                &meeting,
                &initial_state,
                &target_state,
                from_fwd,
                from_bkwd,
                total_states,
                states_checked,
                start.elapsed().as_secs_f64()
            );
            return path;
        }

        println!("No Solution Found after {}s :(", start.elapsed().as_secs());
        BalancerResult::default()
    }
}