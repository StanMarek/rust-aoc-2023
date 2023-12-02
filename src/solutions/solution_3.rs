use crate::utils::reader;

pub fn solve() {
    println!("Day 3");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day3/1.txt").unwrap();

    let output: Option<i32> = None;

    println!("\tSub solution 1: {:?}", output);
}

fn sub_solution_2() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day3/2.txt").unwrap();

    let output: Option<i32> = None;

    println!("\tSub solution 2: {:?}", output);
}
