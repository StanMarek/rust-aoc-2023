use std::collections::HashMap;

pub fn solve() {
    println!("Day 8");

    sub_solution_1();
    sub_solution_2();
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day8/input.txt");
    let mut instructions = String::new();

    let mut map = HashMap::new();
    let instructions_map = HashMap::from([('L', 0), ('R', 1)]);

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            instructions = String::from(line);
        }

        if i > 1 {
            let key = &line[0..3];
            let split = line[7..15].split_once(", ").unwrap();
            map.insert(key, (split.0, split.1));
        }
    }

    let mut step = "AAA";
    let mut steps = 0;
    let mut instruction_index = 0;
    let instructions = instructions.chars().collect::<Vec<char>>();

    while step != "ZZZ" {
        let next_step = map.get(step).unwrap();

        if instruction_index > instructions.len() - 1 {
            instruction_index = 0;
        }

        let direction = *instructions.get(instruction_index).unwrap();
        let direction = *instructions_map.get(&direction).unwrap();

        match direction {
            0 => step = next_step.0,
            1 => step = next_step.1,
            _ => (),
        }

        steps += 1;
        instruction_index += 1;
    }

    println!("\tSub solution 1: {steps}");
}

///
/// needs to be optimized
/// brute force approach
#[warn(dead_code)]
fn sub_solution_2() {
    let input = include_str!("../inputs/day8/input.txt");
    let mut instructions = String::new();

    let mut map = HashMap::new();
    let instructions_map = HashMap::from([('L', 0), ('R', 1)]);
    let mut starting_steps = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            instructions = String::from(line);
        }

        if i > 1 {
            let key = &line[0..3];
            let split = line[7..15].split_once(", ").unwrap();
            map.insert(key.to_string(), (split.0, split.1));

            if key.ends_with("A") {
                starting_steps.push(key.to_string());
            }
        }
    }
    // dbg!(starting_steps);

    let mut steps: i64 = 0;
    let mut iteration_steps = Vec::new();
    let mut instruction_index = 0;
    let instructions = instructions.chars().collect::<Vec<char>>();

    let mut steps_vector = Vec::from(starting_steps.clone());

    let mut end_mapping = HashMap::new();

    for starting_step in starting_steps.clone().into_iter() {
        let mut prefix = starting_step[0..2].to_owned();
        let suffix = "Z";

        prefix.push_str(suffix);

        end_mapping.insert(starting_step, prefix);
    }

    // dbg!(end_mapping);

    loop {
        // dbg!(iteration_steps.clone());
        // dbg!(starting_steps.clone());
        let mut contains = 0;

        for starting_step in starting_steps.clone().into_iter() {
            let ending_value = end_mapping.get(&starting_step).unwrap();

            if iteration_steps.clone().contains(ending_value) {
                contains += 1;
            }
        }

        if contains == starting_steps.len() {
            break;
        }

        // dbg!(contains);

        iteration_steps.clear();

        if instruction_index > instructions.len() - 1 {
            instruction_index = 0;
        }

        let direction = *instructions.get(instruction_index).unwrap();
        let direction = *instructions_map.get(&direction).unwrap();

        for starting_step in steps_vector.clone().into_iter() {
            let next_step = map.get(&starting_step).unwrap();
            match direction {
                0 => iteration_steps.push(next_step.0.to_string()),
                1 => iteration_steps.push(next_step.1.to_string()),
                _ => (),
            }
        }

        steps_vector.clear();
        steps_vector = Vec::from(iteration_steps.clone());
        // steps_vector = Vec::from(*iteration_steps);

        steps += 1;
        instruction_index += 1;
    }

    println!("\tSub solution 1: {steps}");
}
