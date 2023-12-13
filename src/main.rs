mod solutions;
mod types;
mod utils;
use std::env;

use solutions::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Provide day number: cargo run <day-number>");
    }

    let day_number = args.get(1).unwrap().parse::<u32>();
    match day_number {
        Ok(1) => solution_1::solve(),
        Ok(2) => solution_2::solve(),
        Ok(3) => solution_3::solve(),
        Ok(4) => solution_4::solve(),
        Ok(5) => solution_5::solve(),
        Ok(6) => solution_6::solve(),
        Ok(7) => solution_7::solve(),
        Ok(8) => solution_8::solve(),
        Ok(9) => solution_9::solve(),
        Ok(10) => solution_10::solve(),
        Ok(11) => solution_11::solve(),
        Ok(12) => solution_12::solve(),
        Ok(13) => solution_13::solve(),
        _ => panic!("Day not implemented yet!"),
    }
}
