use std::collections::HashMap;

use crate::utils::math::transpose;

pub fn solve() {
    println!("Day 11");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day11/input.txt");
    let mut grid = expand_grid(input);
    let mut count_galaxies = 0;
    let mut coordinates = Vec::new();
    for (i, row) in grid.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == -1 {
                grid[i][j] = count_galaxies + 1;
                count_galaxies += 1;
                coordinates.push((i as u32, j as u32));
            }
        }
    }
    let pairs = (count_galaxies * (count_galaxies - 1)) / 2;
    let mut shortest_paths: HashMap<String, u32> = HashMap::new();

    for (primary_galaxy_index, (x, y)) in coordinates.iter().enumerate() {
        for secondary_galaxy_index in 0..coordinates.len() {
            if secondary_galaxy_index == primary_galaxy_index {
                continue;
            }
            let (x_secondary, y_secondary) = coordinates.get(secondary_galaxy_index).unwrap();

            let x_diff: u32 = x_secondary.abs_diff(*x);
            let y_diff: u32 = y_secondary.abs_diff(*y);

            let steps = x_diff + y_diff;

            let key = generate_key(primary_galaxy_index, secondary_galaxy_index);

            if !shortest_paths.contains_key(&key) {
                shortest_paths.insert(key, steps);
            } else {
                let value = shortest_paths.get(&key).unwrap();
                if steps < *value {
                    shortest_paths.insert(key, steps);
                }
            }
        }
    }

    if !pairs == shortest_paths.len() as i32 {
        panic!("Algorithm output is wrong. Amount of pairs not satisfied");
    }

    let output: u32 = shortest_paths.iter().map(|path| path.1).sum();

    println!("\tSub solution 1: {output}");
}

fn sub_solution_2() {
    let input = include_str!("../inputs/day11/input.txt");

    let AbstractGrid {
        mut grid,
        columns_no_galaxies,
        rows_no_galaxies,
    } = abstract_grid(input);

    let mut count_galaxies = 0;
    let mut coordinates = Vec::new();
    for (i, row) in grid.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == -1 {
                grid[i][j] = count_galaxies + 1;
                count_galaxies += 1;
                coordinates.push((i as u64, j as u64));
            }
        }
    }
    let pairs = (count_galaxies * (count_galaxies - 1)) / 2;
    let mut shortest_paths: HashMap<String, u64> = HashMap::new();

    for (primary_galaxy_index, (x, y)) in coordinates.iter().enumerate() {
        for secondary_galaxy_index in 0..coordinates.len() {
            if secondary_galaxy_index == primary_galaxy_index {
                continue;
            }
            let (x_secondary, y_secondary) = coordinates.get(secondary_galaxy_index).unwrap();

            let mut x_diff = 0;
            let mut y_diff = 0;

            match x_secondary > x {
                true => {
                    for column in *x..*x_secondary {
                        if rows_no_galaxies.contains(&(column as usize)) {
                            x_diff += 1_000_000;
                            continue;
                        }
                        x_diff += 1;
                    }
                }
                false => {
                    for column in *x_secondary..*x {
                        if rows_no_galaxies.contains(&(column as usize)) {
                            x_diff += 1_000_000;
                            continue;
                        }
                        x_diff += 1;
                    }
                }
            }
            match y_secondary > y {
                true => {
                    for column in *y..*y_secondary {
                        if columns_no_galaxies.contains(&(column as usize)) {
                            y_diff += 1_000_000;
                            continue;
                        }
                        y_diff += 1;
                    }
                }
                false => {
                    for column in *y_secondary..*y {
                        if columns_no_galaxies.contains(&(column as usize)) {
                            y_diff += 1_000_000;
                            continue;
                        }
                        y_diff += 1;
                    }
                }
            }

            let steps = x_diff + y_diff;

            let key = generate_key(primary_galaxy_index, secondary_galaxy_index);

            if !shortest_paths.contains_key(&key) {
                shortest_paths.insert(key, steps);
            } else {
                let value = shortest_paths.get(&key).unwrap();
                if steps < *value {
                    shortest_paths.insert(key, steps);
                }
            }
        }
    }

    if !pairs == shortest_paths.len() as i64 {
        panic!("Algorithm output is wrong. Amount of pairs not satisfied");
    }

    let output: u64 = shortest_paths.iter().map(|path| path.1).sum();

    println!("\tSub solution 1: {output}");
}

fn generate_key(primary: usize, secondary: usize) -> String {
    if primary < secondary {
        let mut owned = primary.to_string();
        owned.push('.');
        owned.push_str(&secondary.to_string());
        return owned;
    }
    let mut owned = secondary.to_string();
    owned.push('.');
    owned.push_str(&primary.to_string());
    owned
}

fn expand_grid(input: &str) -> Vec<Vec<i32>> {
    let mut grid = Vec::new();
    let mut rows_no_galaxies = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            match char {
                '.' => row.push(0),
                '#' => row.push(-1),
                _ => (),
            };
        }

        grid.push(row);
    }

    for (i, row) in grid.iter().enumerate() {
        if !row.contains(&-1) {
            rows_no_galaxies.push(i)
        }
    }

    let mut columns_no_galaxies = Vec::new();

    for (i, row_transposed) in transpose(grid.clone()).iter().enumerate() {
        if !row_transposed.contains(&-1) {
            columns_no_galaxies.push(i)
        }
    }
    let mut expanded_rows = Vec::new();
    for i in 0..grid.len() {
        expanded_rows.push(grid.get(i).unwrap().clone());

        if rows_no_galaxies.contains(&i) {
            expanded_rows.push(grid.get(i).unwrap().clone());
        }
    }

    let transpose_expanded_rows = transpose(expanded_rows);
    let mut expanded_columns = Vec::new();
    for i in 0..transpose_expanded_rows.len() {
        expanded_columns.push(transpose_expanded_rows.get(i).unwrap().clone());

        if columns_no_galaxies.contains(&i) {
            expanded_columns.push(transpose_expanded_rows.get(i).unwrap().clone());
        }
    }

    transpose(expanded_columns)
}

struct AbstractGrid {
    grid: Vec<Vec<i64>>,
    rows_no_galaxies: Vec<usize>,
    columns_no_galaxies: Vec<usize>,
}

fn abstract_grid(input: &str) -> AbstractGrid {
    let mut grid = Vec::new();
    let mut rows_no_galaxies = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            match char {
                '.' => row.push(0),
                '#' => row.push(-1),
                _ => (),
            };
        }

        grid.push(row);
    }

    for (i, row) in grid.iter().enumerate() {
        if !row.contains(&-1) {
            rows_no_galaxies.push(i)
        }
    }

    let mut columns_no_galaxies = Vec::new();
    for (i, row_transposed) in transpose(grid.clone()).iter().enumerate() {
        if !row_transposed.contains(&-1) {
            columns_no_galaxies.push(i)
        }
    }

    AbstractGrid {
        grid,
        rows_no_galaxies,
        columns_no_galaxies,
    }
}
