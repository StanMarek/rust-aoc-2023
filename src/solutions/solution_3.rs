use std::io::BufRead;

use crate::utils::reader;
//557705
//84266818
pub fn solve() {
    println!("Day 3");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day3/1.txt")
        .unwrap()
        .lines();

    let rows_count = reader::read_file_line_by_line("src/inputs/day3/1.txt")
        .unwrap()
        .lines()
        .count();

    let output: Option<i32> = None;

    let mut data_vector = Vec::new();

    for l in data {
        data_vector.push(l.unwrap());
    }

    for i in 0..rows_count {
        // println!("{:?}", data_vector[i]);
        let row = data_vector.get(i).unwrap();
        let mut row_above = None;
        if i > 0 {
            row_above = data_vector.get(i - 1);
        }
        let mut row_below = None;
        if i < rows_count {
            row_below = data_vector.get(i + 1);
        }
        check_adjacency(row, row_below, row_above)
    }

    println!("\tSub solution 1: {:?}", output);
}

fn check_adjacency(row: &String, row_below: Option<&String>, row_above: Option<&String>) {
    println!("{:?}", row);
    let mut numeric_indices = Vec::new();
    for i in 0..row.chars().count() {
        let char = row.as_bytes().get(i).unwrap();
        if char.is_ascii_alphanumeric() {
            numeric_indices.push(i);
        }
    }
    // println!("{:?}", numeric_indices);
}

fn sub_solution_2() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day3/2.txt").unwrap();

    let output: Option<i32> = None;

    println!("\tSub solution 2: {:?}", output);
}
