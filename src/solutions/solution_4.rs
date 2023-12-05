use std::{collections::HashSet, vec};

pub fn solve() {
    println!("Day 4");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day4/input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let (_, cards) = line.split_once(":").unwrap();
        let (left, right) = cards.split_once("|").unwrap();

        let parse_number = |s| match s {
            " " | "|" => None,
            _ => Some(s.parse::<u8>().unwrap()),
        };

        let winning = left
            .split_whitespace()
            .filter_map(parse_number)
            .collect::<HashSet<u8>>();
        let hand = right
            .split_whitespace()
            .filter_map(parse_number)
            .collect::<HashSet<u8>>();

        let _winning = winning.intersection(&hand).count() as u8;
        let points = match _winning {
            0 => 0,
            n => u32::pow(2, n as u32 - 1),
        };

        sum += points;
    }

    println!("\tSub solution 1: {:?}", sum);
}

fn sub_solution_2() {
    let input = include_str!("../inputs/day4/input.txt");

    let mut cards_tuple = Vec::new();
    for line in input.lines() {
        let (_, cards) = line.split_once(":").unwrap();
        let (left, right) = cards.split_once("|").unwrap();

        let parse_number = |s| match s {
            " " | "|" => None,
            _ => Some(s.parse::<u8>().unwrap()),
        };

        let winning = left
            .split_whitespace()
            .filter_map(parse_number)
            .collect::<HashSet<u8>>();
        let hand = right
            .split_whitespace()
            .filter_map(parse_number)
            .collect::<HashSet<u8>>();

        cards_tuple.push((winning, hand));
    }

    let mut card_count = vec![1u32; cards_tuple.len()];

    for (i, card) in cards_tuple.iter().enumerate() {
        let (winner, hand) = card;
        let winner_count = winner.intersection(&hand).count() as u8;
        let n = card_count[i];

        card_count[(i + 1).min(cards_tuple.len())
            ..(i + winner_count as usize + 1).min(cards_tuple.len())]
            .iter_mut()
            .for_each(|c| *c += n);
    }

    let sum: u32 = card_count.iter().sum();

    println!("\tSub solution 2: {:?}", sum);
}
