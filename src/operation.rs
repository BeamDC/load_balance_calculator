use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Operation {
    Err,
    Merge {
        input: (Option<u64>, Option<u64>, Option<u64>),
        output: u64,
    },
    Split {
        input: u64,
        output: (Option<u64>, Option<u64>, Option<u64>),
    }
}

impl Operation {
    // cost function for giving priority to different operations,
    // higher values are higher priority when used in the queue
    pub fn cost(&self) -> u64 {
        match self {
            // if we call cost on this something has gone horribly wrong
            Operation::Err => 0,
            Operation::Split {input: _, output: _} => 1,
            Operation::Merge {input: _, output: _} => 3,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Err => write!(f, "Error!"),
            Operation::Split {input, output} => {
                let input = *input as f64 / 1e8;

                if output.2.is_none() {
                    let h1 = output.0.unwrap() as f64 / 1e8;
                    let h2 = output.1.unwrap() as f64 / 1e8;
                    write!(f, "split {} -> {}, {}", input, h1, h2)
                }else {
                    let t1 = output.0.unwrap() as f64 / 1e8;
                    let t2 = output.1.unwrap() as f64 / 1e8;
                    let t3 = output.2.unwrap() as f64 / 1e8;
                    write!(f, "split {} -> {}, {}, {}", input, t1, t2, t3)
                }
            }
            Operation::Merge {input, output} => {
                let output = *output as f64 / 1e8;

                if input.2.is_none() {
                    let h1 = input.0.unwrap() as f64 / 1e8;
                    let h2 = input.1.unwrap() as f64 / 1e8;
                    write!(f, "merge {}, {} -> {}", h1, h2, output)
                }else {
                    let t1 = input.0.unwrap() as f64 / 1e8;
                    let t2 = input.1.unwrap() as f64 / 1e8;
                    let t3 = input.2.unwrap() as f64 / 1e8;
                    write!(f, "merge {}, {}, {} -> {}", t1, t2, t3, output)
                }
            }
        }
    }
}