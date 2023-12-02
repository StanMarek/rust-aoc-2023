use std::{io::BufRead, iter::Sum};

use crate::utils::{parser, reader};

pub fn solve() {
    println!("Day 2");

    sub_solution_one();
    // sub_solution_two();
}

fn sub_solution_one() -> () {
    println!("Sub solution 1:");
    let data = reader::read_file_line_by_line("src/inputs/day2/1.txt").unwrap();
    // println!("{:?}", data);

    let mut impossible_games_ids = Vec::new();

    for l in data.lines() {
        let line = l.unwrap();
        let game = line.split(": ").collect::<Vec<&str>>();
        let game_id = game[0].split("Game ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let game_records_str = game[1];
        let game_records = game_records_str.split("; ").collect::<Vec<&str>>();

        let mut game_valid = true;
        // println!("{:?}", game_id);

        for game_record in game_records {
            // println!("{:?}", game_record);
            if game_record.contains(", ") {
                let cubes = game_record.split(", ").collect::<Vec<&str>>();
                // println!("{:?}", cubes);
                for round_entry in cubes {
                    let number_color = round_entry.split(" ").collect::<Vec<&str>>();
                    // println!("{:?}", number_color);
                    let number = number_color[0].parse::<i32>().unwrap();
                    let color = number_color[1];
                    // let max = parser::parse_digit_to_number(color);
                    if color == "red" && number > 12 {
                        game_valid = false;
                    }
                    if color == "blue" && number > 14 {
                        game_valid = false;
                    }
                    if color == "green" && number > 13 {
                        game_valid = false;
                    }
                    // println!("max: {:?}", max);
                }
            } else {
                let number_color = game_record.split(" ").collect::<Vec<&str>>();
                // println!("{:?}", number_color);
                let number = number_color[0].parse::<i32>().unwrap();
                let color = number_color[1];
                // let max = parser::parse_digit_to_number(color);
                if color == "red" && number > 12 {
                    game_valid = false;
                }
                if color == "blue" && number > 14 {
                    game_valid = false;
                }
                if color == "green" && number > 13 {
                    game_valid = false;
                }
            }
        }

        if !game_valid {
            impossible_games_ids.push(game_id);
        }
    }

    let sum: i32 = impossible_games_ids.iter().sum();
    println!("Sum: {:?}", sum);
}

fn sub_solution_two() -> () {
    println!("Sub solution 2:");
    let data = reader::read_file_line_by_line("src/inputs/day1/1.txt").unwrap();
}
