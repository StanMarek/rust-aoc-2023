use std::collections::HashMap;

pub fn solve() {
    println!("Day 12");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() {
    let total = handle(1);
    println!("\tSub solution 1: {total}");
}

fn sub_solution_2() {
    let total = handle(5);
    println!("\tSub solution 2: {total}");
}

fn handle(reproduce: u32) -> u64 {
    let lines: Vec<(&str, &str)> = include_str!("../inputs/day12/input.txt")
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let first_word_chars = words.next().unwrap_or("");
            let second_word = words.next().unwrap_or("");
            (first_word_chars, second_word)
        })
        .collect();

    let mut total: u64 = 0;

    let mut cache: HashMap<String, u64> = HashMap::new();

    for (_spring, _arrangement) in lines.iter() {
        let mut spring = (*_spring).to_owned();
        let mut arrangement = (*_arrangement).to_owned();

        for _i in 0..reproduce - 1 {
            spring.push('?');
            spring.push_str(_spring);
            arrangement.push(',');
            arrangement.push_str(_arrangement);
        }

        let arrangements = arrangement
            .split(',')
            .map(|number| number.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        total += count(spring.as_str(), &arrangements, &mut cache);
    }

    total
}

fn count(springs: &str, arrangements: &[u64], cache: &mut HashMap<String, u64>) -> u64 {
    if springs.is_empty() {
        return if arrangements.is_empty() { 1 } else { 0 };
    }

    if arrangements.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    let key = format!(
        "{}-{}",
        springs,
        arrangements
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join(".")
    );

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    if springs.starts_with('.') || springs.starts_with('?') {
        result += count(&springs[1..], arrangements, cache);
    }

    if (springs.starts_with('#') || springs.starts_with('?'))
        && arrangements[0] <= springs.len() as u64
        && !springs[..arrangements[0] as usize].contains('.')
        && (arrangements[0] == springs.len() as u64
            || springs.chars().nth(arrangements[0] as usize).unwrap() != '#')
    {
        result += count(
            if arrangements[0] + 1 < springs.len() as u64 {
                &springs[(arrangements[0] + 1) as usize..]
            } else {
                ""
            },
            &arrangements[1..],
            cache,
        );
    }

    cache.insert(key, result);

    result
}
