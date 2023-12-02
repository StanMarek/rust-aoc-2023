use std::io::BufRead;

use crate::utils::{math, parser, reader};

pub fn solve() {
    println!("Day 2");

    sub_solution_one();
    sub_solution_two();
}

fn sub_solution_one() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day2/1.txt").unwrap();

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

        for game_record in game_records {
            if game_record.contains(", ") {
                let cubes = game_record.split(", ").collect::<Vec<&str>>();
                for round_entry in cubes {
                    let number_color = round_entry.split(" ").collect::<Vec<&str>>();
                    let number = number_color[0].parse::<i32>().unwrap();
                    let color = number_color[1];

                    let max_color = parser::parse_color_to_max(color);
                    if number > max_color {
                        game_valid = false;
                    }
                }
            } else {
                let number_color = game_record.split(" ").collect::<Vec<&str>>();
                let number = number_color[0].parse::<i32>().unwrap();
                let color = number_color[1];

                let max_color = parser::parse_color_to_max(color);
                if number > max_color {
                    game_valid = false;
                }
            }
        }

        if game_valid {
            impossible_games_ids.push(game_id);
        }
    }

    let sum: i32 = impossible_games_ids.iter().sum();

    println!("\tSub solution 1: {:?}", sum);
}

fn sub_solution_two() -> () {
    let data = reader::read_file_line_by_line("src/inputs/day2/2.txt").unwrap();

    let mut multiplied_sum = 0;

    for l in data.lines() {
        let line = l.unwrap();
        let game = line.split(": ").collect::<Vec<&str>>();
        let game_records_str = game[1];
        let game_records = game_records_str.split("; ").collect::<Vec<&str>>();

        let mut min = [0, 0, 0]; // R G B

        for game_record in game_records {
            if game_record.contains(", ") {
                let cubes = game_record.split(", ").collect::<Vec<&str>>();
                for round_entry in cubes {
                    let number_color = round_entry.split(" ").collect::<Vec<&str>>();
                    let number = number_color[0].parse::<i32>().unwrap();
                    let color = number_color[1];

                    let index = parser::parse_color_to_index(color);

                    if number > min[index] {
                        min[index] = number;
                    }
                }
            } else {
                let number_color = game_record.split(" ").collect::<Vec<&str>>();
                let number = number_color[0].parse::<i32>().unwrap();
                let color = number_color[1];

                let index = parser::parse_color_to_index(color);

                if number > min[index] {
                    min[index] = number;
                }
            }
        }

        multiplied_sum += math::multiply_array_elements(&min);
    }

    println!("\tSub solution 2: {:?}", multiplied_sum);
}
