use std::collections::HashMap;

pub fn solve() {
    println!("Day 7");

    sub_solution_1();
    sub_solution_2();
}

trait Parse {
    fn parse(&self) -> Vec<(&str, usize)>;
}

impl Parse for String {
    fn parse(&self) -> Vec<(&str, usize)> {
        self.lines()
            .map(|line| {
                let split = line.split_whitespace().take(2).collect::<Vec<&str>>();
                let hand_cards = *split.first().unwrap();
                let points = split.get(1).unwrap().parse::<usize>().unwrap();
                (hand_cards, points)
            })
            .collect::<Vec<(&str, usize)>>()
    }
}

fn classify(cards: &str) -> usize {
    let counts = cards
        .chars()
        .map(|card| cards.matches(card).count())
        .collect::<Vec<usize>>();
    if counts.contains(&5) {
        return 6;
    };
    if counts.contains(&4) {
        return 5;
    };
    if counts.contains(&3) {
        if counts.contains(&2) {
            return 4;
        };
        return 3;
    };
    if counts.iter().filter(|&n| *n == 2).count() == 4 {
        return 2;
    };
    if counts.contains(&2) {
        return 1;
    };
    0
}

fn replacement(cards: &str) -> Vec<String> {
    if cards.is_empty() {
        return vec!["".to_string()];
    }

    let first_chars = if cards.starts_with('J') {
        "23456789TQKA".chars().collect::<Vec<_>>()
    } else {
        vec![cards.chars().next().unwrap()]
    };

    let mut result = Vec::new();

    for x in first_chars {
        for y in replacement(&cards[1..]) {
            result.push(format!("{}{}", x, y));
        }
    }

    result
}

fn strength(
    cards: &str,
    map: HashMap<char, char>,
    func: &dyn Fn(&str) -> usize,
) -> (usize, Vec<char>) {
    let cards_char_vec = cards
        .chars()
        .map(|card| *map.get(&card).unwrap_or(&card))
        .collect::<Vec<char>>();

    (func(cards), cards_char_vec)
}

fn sub_solution_1() {
    let binding = include_str!("../inputs/day7/input.txt").to_string();
    let mut input = binding.parse();

    let map = HashMap::from([('T', 'A'), ('J', 'B'), ('Q', 'C'), ('K', 'D'), ('A', 'E')]);

    input.sort_by_key(|tuple| strength(tuple.0, map.clone(), &classify));

    let mut output = 0;

    for ((_, (_, points)), multiplier) in input.into_iter().enumerate().zip(1..) {
        output += multiplier * points;
    }

    println!("\tSub solution 1: {output}");
}

fn sub_solution_2() {
    let input = include_str!("../inputs/day7/input.txt").to_string();
    let mut tuples = input.parse();

    let map: HashMap<char, char> =
        HashMap::from([('T', 'A'), ('J', '.'), ('Q', 'C'), ('K', 'D'), ('A', 'E')]);

    let score = |hand_cards: &str| {
        replacement(hand_cards)
            .iter()
            .map(|card| classify(card))
            .max()
            .unwrap()
    };

    tuples.sort_by_key(|tuple| strength(tuple.0, map.clone(), &score));

    let mut output = 0;
    for ((_, (_, points)), multiplier) in tuples.into_iter().enumerate().zip(1..) {
        output += multiplier * points;
    }

    println!("\tSub solution 2: {output}");
}
