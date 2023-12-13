use itertools::Itertools;

pub fn solve() {
    println!("Day 13");

    let input = include_str!("../inputs/day13/input.txt");

    sub_solution_1(input);
    sub_solution_2(input);
}

type FoldFunctionType =
    dyn Fn(&str, &dyn Fn(&str) -> Option<Fold>, &dyn Fn(&str) -> Option<Fold>) -> Option<Fold>;

#[derive(Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn handle_folding(
    input: &str,
    fold_fn: &FoldFunctionType,
    vertical_fn: &dyn Fn(&str) -> Option<Fold>,
    horizontal_fn: &dyn Fn(&str) -> Option<Fold>,
) -> usize {
    let (horizontal, vertical) = input
        .split("\n\n")
        .flat_map(|s| fold_fn(s, vertical_fn, horizontal_fn))
        .fold((0usize, 0usize), |mut acc, item| match item {
            Fold::Horizontal(num) => {
                acc.0 += 100 * num;
                acc
            }
            Fold::Vertical(num) => {
                acc.1 += num;
                acc
            }
        });

    horizontal + vertical
}

fn sub_solution_1(input: &str) {
    let output = handle_folding(input, &fold, &vertical, &horizontal);

    println!("\tSub solution 1: {output}");
}

fn sub_solution_2(input: &str) {
    let output = handle_folding(input, &fold, &vertical_2, &horizontal_2);

    println!("\tSub solution 2: {output}");
}

fn fold(
    input: &str,
    vertical_fn: &dyn Fn(&str) -> Option<Fold>,
    horizontal_fn: &dyn Fn(&str) -> Option<Fold>,
) -> Option<Fold> {
    vertical_fn(input).or(horizontal_fn(input))
}

fn vertical(input: &str) -> Option<Fold> {
    let mut columns_iter_collection = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    let columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| line_a == line_b)
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = columns[0..=index_a].iter().rev();
            let lines_b = columns[index_b..].iter();
            lines_a
                .zip(lines_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });

    result.map(Fold::Vertical)
}

fn horizontal(input: &str) -> Option<Fold> {
    let lines: Vec<&str> = input.lines().collect();

    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| line_a == line_b)
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = lines[0..=index_a].iter().rev();
            let lines_b = lines[index_b..].iter();
            lines_a
                .zip(lines_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });

    result.map(Fold::Horizontal)
}

fn vertical_2(input: &str) -> Option<Fold> {
    let mut columns_iter_collection = input.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
                || line_a
                    .iter()
                    .zip(line_b.iter())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 1
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = columns[0..=index_a].iter().rev();
            let lines_b = columns[index_b..].iter();

            (lines_a
                .flatten()
                .zip(lines_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(index_a + 1)
        });
    result.map(Fold::Vertical)
}

fn horizontal_2(input: &str) -> Option<Fold> {
    let lines: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
                || line_a
                    .chars()
                    .zip(line_b.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 1
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = lines[0..=index_a].iter().map(|line| line.chars()).rev();
            let lines_b = lines[index_b..].iter().map(|line| line.chars());

            (lines_a
                .flatten()
                .zip(lines_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(index_a + 1)
        });
    result.map(Fold::Horizontal)
}
