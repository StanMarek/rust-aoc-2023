pub fn solve() {
    println!("Day 9");

    sub_solution_1();
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
        solve_accumulator(&mut accumulator);

        output += accumulator.last().unwrap().last().unwrap();
    }

    println!("\tSub solution 2: {output}");
}

fn solve_series(main_series: Vec<i64>, accumulator: &mut Vec<Vec<i64>>) -> () {
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

fn solve_accumulator(accumulator: &mut Vec<Vec<i64>>) {
    for index in 0..accumulator.len() - 1 {
        let bottom = *accumulator.get(index).unwrap().last().unwrap();
        let above = *accumulator.get(index + 1).unwrap().last().unwrap();

        accumulator.get_mut(index + 1).unwrap().push(above + bottom);
    }
}
