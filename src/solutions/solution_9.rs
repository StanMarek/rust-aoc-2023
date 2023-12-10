pub fn solve() {
    println!("Day 9");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day9/input.txt").lines();

    let mut output = 0;

    for line in input {
        let series = line
            .split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut accumulator = Vec::new();

        solve_series(series.clone(), &mut accumulator);
        extrapolate(&mut accumulator, 1);

        output += accumulator.last().unwrap().last().unwrap();
    }

    println!("\tSub solution 1: {output}");
}

fn solve_series(main_series: Vec<i64>, accumulator: &mut Vec<Vec<i64>>) {
    if !main_series.iter().all(|x| *x == 0) {
        let mut diffs = Vec::new();
        for i in 0..main_series.clone().len() - 1 {
            let current = main_series.get(i).unwrap();
            let next = main_series.get(i + 1).unwrap();
            diffs.push(next - current);
        }
        solve_series(diffs, accumulator);
    }

    accumulator.push(main_series);
}

fn extrapolate(accumulator: &mut Vec<Vec<i64>>, extrapolation: i8) {
    for index in 0..accumulator.len() - 1 {
        if extrapolation == 1 {
            let bottom = *accumulator.get(index).unwrap().last().unwrap();
            let above = *accumulator.get(index + 1).unwrap().last().unwrap();

            accumulator.get_mut(index + 1).unwrap().push(above + bottom);
        }
        if extrapolation == -1 {
            let bottom = *accumulator.get(index).unwrap().last().unwrap();
            let above = *accumulator.get(index + 1).unwrap().first().unwrap();

            accumulator.get_mut(index + 1).unwrap().push(above - bottom);
        }
    }
}

fn sub_solution_2() {
    let input = include_str!("../inputs/day9/input.txt").lines();

    let mut output = 0;

    for line in input {
        let series = line
            .split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut accumulator = Vec::new();

        solve_series(series.clone(), &mut accumulator);
        extrapolate(&mut accumulator, -1);
        output += accumulator.last().unwrap().last().unwrap();
    }

    println!("\tSub solution 2: {output}");
}
