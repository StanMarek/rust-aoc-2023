use std::collections::HashMap;

pub fn solve() {
    println!("Day 10");

    let (grid, start) = init_grid();

    let distance_map = sub_solution_1(&grid, start);
    sub_solution_2(&grid, &distance_map);
}

fn init_grid() -> (Vec<Vec<char>>, (i32, i32)) {
    let input = include_str!("../inputs/day10/input.txt");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }

    for (i, row) in grid.clone().into_iter().enumerate() {
        if row.contains(&'S') {
            start = (row.iter().position(|c| *c == 'S').unwrap() as i32, i as i32);
            break;
        }
    }

    grid[start.1 as usize][start.0 as usize] = 'F';

    (grid, start)
}

fn sub_solution_1(grid: &[Vec<char>], start: (i32, i32)) -> HashMap<(i32, i32), usize> {
    let pipe_map: HashMap<char, [(i32, i32); 2]> = HashMap::from([
        ('|', [(0, 1), (0, -1)]),
        ('-', [(1, 0), (-1, 0)]),
        ('L', [(0, -1), (1, 0)]),
        ('J', [(0, -1), (-1, 0)]),
        ('7', [(-1, 0), (0, 1)]),
        ('F', [(0, 1), (1, 0)]),
    ]);

    let mut dist = 0;
    let mut pipe: (i32, i32) = start;
    let mut distance_map: HashMap<(i32, i32), usize> = HashMap::new();

    while !distance_map.contains_key(&pipe) {
        distance_map.insert(pipe, dist);
        dist += 1;

        let (x, y) = pipe;
        let c = grid[y as usize][x as usize];
        let pipe_map_value = pipe_map.get(&c).unwrap();

        for (dx, dy) in pipe_map_value {
            let nx = x + dx;
            let ny = y + dy;

            if !distance_map.contains_key(&(nx, ny)) {
                pipe = (nx, ny);
                break;
            }
        }
    }

    dist /= 2;

    println!("\tSub solution 1: {dist}");

    distance_map
}

fn sub_solution_2(grid: &[Vec<char>], distance_map: &HashMap<(i32, i32), usize>) {
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut parity = 0;
        for (j, char) in row.iter().enumerate() {
            if !distance_map.contains_key(&(j as i32, i as i32)) {
                if parity % 2 == 1 {
                    count += 1;
                }
                continue;
            }

            if ['|', 'L', 'J'].contains(char) {
                parity += 1;
            }
        }
    }

    println!("\tSub solution 2: {count}");
}
