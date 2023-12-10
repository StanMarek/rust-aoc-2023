use std::collections::HashMap;

use crate::utils::reader;

pub fn solve() {
    println!("Day 3");

    sub_solution_1();
    sub_solution_2();
}

type Input = (Vec<u32>, HashMap<(usize, usize), Vec<u32>>);

fn handle_input() -> Input {
    let lines = reader::read_file_line_by_line("src/inputs/day3/1.txt");

    let input = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part_numbers = vec![];
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    'next_line: for (row, line) in input.iter().enumerate() {
        let mut col1 = 0;
        let mut col2;
        while col1 < line.len() {
            while !line[col1].is_numeric() {
                col1 += 1;
                if col1 >= line.len() {
                    continue 'next_line;
                }
            }
            col2 = col1;
            while col2 < line.len() && line[col2].is_numeric() {
                col2 += 1;
            }
            let n: String = line[col1..col2].iter().copied().collect();
            let n: u32 = n.parse().unwrap();

            let start_row = if row > 0 { row - 1 } else { 0 };
            let end_row = (row + 2).min(input.len());
            let start_col = if col1 > 0 { col1 - 1 } else { 0 };
            let end_col = (col2 + 1).min(line.len());

            'outer: for (i, _) in input.iter().enumerate().take(end_row).skip(start_row) {
                for j in start_col..end_col {
                    if !input[i][j].is_numeric() && input[i][j] != '.' {
                        if input[i][j] == '*' {
                            if let Some(parts) = gears.get_mut(&(i, j)) {
                                parts.push(n);
                            } else {
                                gears.insert((i, j), vec![n]);
                            }
                        }
                        part_numbers.push(n);
                        break 'outer;
                    }
                }
            }

            col1 = col2;
        }
    }

    (part_numbers, gears)
}

fn sub_solution_1() {
    let (part_numbers, _) = handle_input();
    let output: u32 = part_numbers.iter().sum();

    println!("\tSub solution 1: {:?}", output);
}

fn sub_solution_2() {
    let (_, gears) = handle_input();

    let output: u32 = gears
        .into_values()
        .filter(|g| g.len() == 2)
        .map(|g| g.into_iter().product::<u32>())
        .sum();

    println!("\tSub solution 2: {:?}", output);
}
