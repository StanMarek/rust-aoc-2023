use crate::utils::*;

pub fn solve() {
    println!("Day 1");
    sub_solution1();
    sub_solution2();
}

fn sub_solution1() {
    let data = reader::read_file_line_by_line("src/inputs/day1/1.txt");
    let mut sum = 0;
    for line in data {
        let mut digits: Vec<char> = Vec::new();

        for character in line.chars() {
            if character.is_numeric() {
                digits.push(character);
            }
        }

        let mut number = String::new();
        number.push(digits[0]);
        number.push(digits[digits.len() - 1]);

        sum += number.parse::<i32>().unwrap();
        // println!("NUmbers: {:?}, NUmber: {:?}", digits, number);
    }
    println!("\tSub solution 1: {:?}", sum);
}

fn sub_solution2() {
    let data = reader::read_file_line_by_line("src/inputs/day1/2.txt");
    let all_digits = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut sum = 0;
    for line in data {
        // let line = l.unwrap();
        let mut pos_first = None;
        let mut digit_first = None;
        let mut pos_last = None;
        let mut digit_last = None;

        for digit in all_digits {
            if let Some(position) = line.find(digit) {
                if position <= pos_first.unwrap_or(position) {
                    pos_first = Some(position);
                    digit_first = Some(digit);
                }
            }

            if let Some(position) = line.rfind(digit) {
                if position >= pos_last.unwrap_or(position) {
                    pos_last = Some(position);
                    digit_last = Some(digit);
                }
            }
        }

        let first = digit_first.map(parser::parse_digit_to_number).unwrap();
        let last = digit_last.map(parser::parse_digit_to_number).unwrap();

        sum += first * 10 + last;
    }
    println!("\tSub solution 2: {:?}", sum);
}
