use std::fmt::{Debug, Formatter};
use std::io;

pub struct Args {
    pub inputs: Vec<f32>,
    pub outputs: Vec<f32>,
    pub max_belt: u64,
    pub quit: bool,
}

impl Args {
    pub fn new(input: String) -> Self {
        let mut opts: Vec<&str> = input.split_whitespace().rev().collect();
        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut max_belt = 1200; // assume mk6 by default
        let mut quit = false;

        while opts.len() > 0 {
            match *opts.last().unwrap_or(&"") {
                "-in" => {
                    opts.pop();
                    // consume all non flags
                    while opts.last().unwrap_or(&"").find("-").is_none() {
                        let belt = opts.pop().unwrap_or("").to_lowercase();
                        let compound = belt.split("x").collect::<Vec<&str>>();
                        if compound.len() == 2 {
                            let rate = compound[0].parse::<f32>().unwrap_or(0.0);
                            let rep = compound[1].parse::<usize>().unwrap_or(0);
                            for _ in 0..rep { inputs.push(rate); }
                        }
                        inputs.push(belt.parse::<f32>().unwrap_or(0.0));
                    }
                }
                "-out" => {
                    opts.pop();
                    while opts.last().unwrap_or(&"").find("-").is_none() {
                        let belt = opts.pop().unwrap_or("").to_lowercase();
                        let compound = belt.split("x").collect::<Vec<&str>>();
                        if compound.len() == 2 {
                            let rate = compound[0].parse::<f32>().unwrap_or(0.0);
                            let rep = compound[1].parse::<usize>().unwrap_or(0);
                            for _ in 0..rep { outputs.push(rate); }
                        }
                        outputs.push(belt.parse::<f32>().unwrap_or(0.0));
                    }
                },
                "-mb" => {
                    opts.pop();
                    max_belt = opts
                        .pop()
                        .unwrap_or("")
                        .parse::<u64>()
                        .unwrap_or(1200);
                },
                "-q" => {
                    opts.pop();
                    quit = true;
                },
                _ => println!("Invalid option: {}", opts.pop().unwrap_or(&"None")),
            }
        }

        inputs.retain(|&x| x > 0.0);
        outputs.retain(|&x| x > 0.0);

        Self {
            inputs,
            outputs,
            max_belt,
            quit,
        }
    }
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Args [\n  inputs: {:?}\n  outputs: {:?}\n  max_belt: {}\n  quit: {}\n]",
            self.inputs,
            self.outputs,
            self.max_belt,
            self.quit
        )
    }
}

pub fn read_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin().read_line(&mut buffer);
    match stdin {
        Ok(_) => {}
        Err(error) => {
            println!("error: {}", error);
        }
    }

    buffer
}
