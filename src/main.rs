use crate::balancer::Balancer;
use crate::cmd::{read_input, Args};

mod cmd;
mod calc;
mod operation;
mod helpers;
mod tests;
mod balancer;

fn main() {
    loop {
        let input = read_input();
        let args = Args::new(input);
        // println!("{:?}",args);
        if args.quit {
            println!("Quitting");
            break;
        }
        if args.inputs.len() == 0 {
            panic!("No input belts given");
        }
        if args.outputs.len() == 0 {
            panic!("No output belts given");
        }
        let balancer = Balancer::new(args);
        let result = balancer.find_ideal_balance();
        for (i, (op, state))in result.iter().enumerate() {
            println!("{}. {} => {}", i + 1, op, state)
        }
        println!("{}", result);
    }
}
