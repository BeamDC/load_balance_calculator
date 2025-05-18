use crate::calc::Balancer;
use crate::cmd::{read_input, Args};

mod cmd;
mod calc;
mod operation;
mod helpers;

fn main() {
    let input = read_input();
    let args = Args::new(input);
    // println!("{:?}",args);
    if args.inputs.len() == 0 {
        panic!("No input belts given");
    }
    if args.outputs.len() == 0 {
        panic!("No output belts given");
    }
    let balancer = Balancer::new(args);
    let _ = balancer.find_ideal_balance();
}
