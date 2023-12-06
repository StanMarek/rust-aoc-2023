use std::{collections::HashMap, i32::MAX, str::FromStr};

pub fn solve() {
    println!("Day 5:");

    sub_solution_1();
    // sub_solution_2();
}

#[derive(Debug, Clone, Copy)]
struct Node {
    source_start: i64,
    source_end: i64,
    destination_start: i64,
    destination_end: i64,
    range: i64,
}

#[derive(Clone)]
struct SeedMapping {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Node>,
    soil_to_fertilizer: Vec<Node>,
    fertilizer_to_water: Vec<Node>,
    water_to_light: Vec<Node>,
    light_to_temp: Vec<Node>,
    temp_to_hum: Vec<Node>,
    hum_to_location: Vec<Node>,
}

impl FromStr for SeedMapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut seed_to_soil: Vec<Node> = Vec::new();
        let mut soil_to_fertilizer: Vec<Node> = Vec::new();
        let mut fertilizer_to_water: Vec<Node> = Vec::new();
        let mut water_to_light: Vec<Node> = Vec::new();
        let mut light_to_temp: Vec<Node> = Vec::new();
        let mut temp_to_hum: Vec<Node> = Vec::new();
        let mut hum_to_location: Vec<Node> = Vec::new();

        let mut condition = 0;
        // dbg!(condition);
        let parse_number = |s| match s {
            " " => None,
            _ => Some(s.parse::<i64>().unwrap()),
        };

        for line in s.lines() {
            if line.starts_with("seeds:") {
                let (_, read_seeds) = line.split_once(":").unwrap();
                seeds = read_seeds
                    .split_whitespace()
                    .filter_map(parse_number)
                    .into_iter()
                    .collect::<Vec<i64>>();
            }

            match line {
                "seed-to-soil map:" => condition = 1,
                "soil-to-fertilizer map:" => condition = 2,
                "fertilizer-to-water map:" => condition = 3,
                "water-to-light map:" => condition = 4,
                "light-to-temperature map:" => condition = 5,
                "temperature-to-humidity map:" => condition = 6,
                "humidity-to-location map:" => condition = 7,
                _ => (),
            }

            if condition >= 1 && condition <= 7 && !line.is_empty() && !line.contains("map:") {
                let line_numbers = line
                    .split_whitespace()
                    .filter_map(parse_number)
                    .collect::<Vec<i64>>();
                // dbg!(line_numbers.clone());
                let destination_start = *line_numbers.get(0).unwrap();
                let source_start = *line_numbers.get(1).unwrap();
                let range = *line_numbers.get(2).unwrap();
                // dbg!(range);

                match condition {
                    1 => seed_to_soil.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    2 => soil_to_fertilizer.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    3 => fertilizer_to_water.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    4 => water_to_light.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    5 => light_to_temp.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    6 => temp_to_hum.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    7 => hum_to_location.push(Node {
                        source_start: source_start,
                        source_end: source_start + range - 1,
                        destination_start: destination_start,
                        destination_end: destination_start + range - 1,
                        range: range,
                    }),
                    _ => (),
                };
            }

            // dbg!(condition);
        }
        // let _seeds = seeds.clone();

        // dbg!(seeds.clone());

        Ok(SeedMapping {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_hum,
            hum_to_location,
        })
    }
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day5/input.txt");
    let mut output = i64::MAX;

    let seed_mapping = input.parse::<SeedMapping>().unwrap();
    // let mut locations = Vec::new();

    // dbg!(seed_mapping.fertilizer_to_water.clone());
    for seed in seed_mapping.seeds.iter() {
        let soil_key = get_value(seed_mapping.seed_to_soil.clone(), *seed);
        let fertilizer_key = get_value(seed_mapping.soil_to_fertilizer.clone(), soil_key);
        let water_key = get_value(seed_mapping.fertilizer_to_water.clone(), fertilizer_key);
        let light_key = get_value(seed_mapping.water_to_light.clone(), water_key);
        let temp_key = get_value(seed_mapping.light_to_temp.clone(), light_key);
        let hum_key = get_value(seed_mapping.temp_to_hum.clone(), temp_key);
        let location = get_value(seed_mapping.hum_to_location.clone(), hum_key);

        if location < output {
            output = location;
        }
    }

    // dbg!(seed_mapping.seed_to_soil.clone());

    println!("\tSub solution 1: {:?}", output);
}

fn get_value(nodes: Vec<Node>, key: i64) -> i64 {
    let mut next_key = key;
    // dbg!(key);
    for node in nodes.iter().rev() {
        if key >= node.source_start && key <= node.source_end {
            let diff = key - node.source_start;

            if diff <= node.range {
                next_key = node.destination_start + diff;
            }

            break;
        } else {
            continue;
        }
    }

    return next_key;
}

struct SeedMappingAdvanced {
    seeds: Vec<Node>,
    seed_to_soil: Vec<Node>,
    soil_to_fertilizer: Vec<Node>,
    fertilizer_to_water: Vec<Node>,
    water_to_light: Vec<Node>,
    light_to_temp: Vec<Node>,
    temp_to_hum: Vec<Node>,
    hum_to_location: Vec<Node>,
}

impl From<SeedMapping> for SeedMappingAdvanced {
    fn from(value: SeedMapping) -> Self {
        let mut seeds = Vec::new();

        for i in (0..value.seeds.len()).step_by(2) {
            let source_start = value.seeds.get(i).unwrap();
            let range = value.seeds.get(i + 1).unwrap();

            seeds.push(Node {
                source_start: *source_start,
                source_end: source_start + range - 1,
                destination_start: 0,
                destination_end: 0,
                range: *range,
            })
        }

        SeedMappingAdvanced {
            seeds,
            seed_to_soil: value.seed_to_soil,
            soil_to_fertilizer: value.soil_to_fertilizer,
            fertilizer_to_water: value.fertilizer_to_water,
            water_to_light: value.water_to_light,
            light_to_temp: value.light_to_temp,
            temp_to_hum: value.temp_to_hum,
            hum_to_location: value.hum_to_location,
        }
    }
}

fn sub_solution_2() {
    let input = include_str!("../inputs/day5/input.txt");
    let mut output = i64::MAX;

    let seed_mapping = input.parse::<SeedMapping>().unwrap();
    let seed_mapping_advanced = SeedMappingAdvanced::from(seed_mapping.clone());
    let _seeds = seed_mapping.seeds;
    // let mut seeds = Vec::new();

    // dbg!(seed_mapping_advanced.seeds.clone());
    // for

    for _seed in seed_mapping_advanced.seeds.iter() {
        let source_start = _seed.source_start;
        let source_end = _seed.source_end;
        // let range = _seed.range;

        for seed in source_start..source_end {
            let soil_key = get_value(seed_mapping.seed_to_soil.clone(), seed);
            let fertilizer_key = get_value(seed_mapping.soil_to_fertilizer.clone(), soil_key);
            let water_key = get_value(seed_mapping.fertilizer_to_water.clone(), fertilizer_key);
            let light_key = get_value(seed_mapping.water_to_light.clone(), water_key);
            let temp_key = get_value(seed_mapping.light_to_temp.clone(), light_key);
            let hum_key = get_value(seed_mapping.temp_to_hum.clone(), temp_key);
            let location = get_value(seed_mapping.hum_to_location.clone(), hum_key);

            if location < output {
                output = location;
            }
        }
    }
    println!("\tSub solution 2: {:?}", output);
}
