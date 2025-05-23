use std::{fmt, iter};
use std::fmt::{Formatter};
use std::iter::{Repeat, Take};
use crate::cmd::Args;
use crate::operation::Operation;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BalancerState {
    pub values: Box<[u64]>
}

impl BalancerState {
    pub fn new(state: Vec<u64>) -> BalancerState {
        BalancerState {
            values: state.into_boxed_slice()
        }
    }

    pub fn to_vec(&self) -> Vec<u64> {
        self.values.to_vec()
    }

    pub fn iter(&self) -> std::slice::Iter<u64> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl fmt::Display for BalancerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, val) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", *val as f64 / 1e8)?;
        }
        write!(f, ")")
    }
}

pub struct Balancer {
    pub max_belt: u64,
    pub inputs: Vec<u64>,
    pub outputs: Vec<u64>,
    // pub depth_limit: u64,
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
            // depth_limit: 0,
        }
    }
}

pub struct BalancerResult {
    path: Vec<(Operation, BalancerState)>
}

impl BalancerResult {
    pub fn new(path: Vec<(Operation, BalancerState)>) -> BalancerResult {
        BalancerResult {
            path
        }
    }

    pub fn iter(&self) -> std::slice::Iter<(Operation, BalancerState)> {
        self.path.iter()
    }
}

impl Default for BalancerResult {
    fn default() -> BalancerResult {
        BalancerResult::new(vec![])
    }
}

impl fmt::Display for BalancerResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: Finish Display process")
    }
}

