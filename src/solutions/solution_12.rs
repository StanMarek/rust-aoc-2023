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

fn count(cfg: &str, nums: &[u64], cache: &mut HashMap<String, u64>) -> u64 {
    if cfg.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if cfg.contains('#') { 0 } else { 1 };
    }

    let key = format!(
        "{}-{}",
        cfg,
        nums.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join(".")
    );

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    if cfg.starts_with('.') || cfg.starts_with('?') {
        result += count(&cfg[1..], nums, cache);
    }

    if (cfg.starts_with('#') || cfg.starts_with('?'))
        && nums[0] <= cfg.len() as u64
        && !cfg[..nums[0] as usize].contains('.')
        && (nums[0] == cfg.len() as u64 || cfg.chars().nth(nums[0] as usize).unwrap() != '#')
    {
        result += count(
            if nums[0] + 1 < cfg.len() as u64 {
                &cfg[(nums[0] + 1) as usize..]
            } else {
                ""
            },
            &nums[1..],
            cache,
        );
    }

    cache.insert(key, result);
    result
}
