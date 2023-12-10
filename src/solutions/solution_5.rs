use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::types::{
    node::Node,
    seed_mapping::{SeedMapping, SeedMappingAdvanced},
};

pub fn solve() {
    println!("Day 5:");

    sub_solution_1();
    // sub_solution_2();
}

fn sub_solution_1() {
    let input = include_str!("../inputs/day5/input.txt");
    let mut output = i64::MAX;

    let seed_mapping = input.parse::<SeedMapping>().unwrap();

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

    println!("\tSub solution 1: {:?}", output);
}

///
/// This should be optimized
/// e.g. use parallel
#[warn(dead_code)]
fn _sub_solution_2() {
    let input = include_str!("../inputs/day5/input.txt");
    // let mut output = i64::MAX;
    let output = std::sync::Mutex::new(std::i64::MAX);

    let seed_mapping = input.parse::<SeedMapping>().unwrap();
    let seed_mapping_advanced = SeedMappingAdvanced::from(seed_mapping.clone());
    let _seeds = seed_mapping.seeds;

    seed_mapping_advanced.seeds.par_iter().for_each(|_seed| {
        let source_start = _seed.source_start;
        let source_end = _seed.source_end;

        for seed in source_start..source_end {
            let soil_key = get_value(seed_mapping_advanced.seed_to_soil.clone(), seed);
            let fertilizer_key =
                get_value(seed_mapping_advanced.soil_to_fertilizer.clone(), soil_key);
            let water_key = get_value(
                seed_mapping_advanced.fertilizer_to_water.clone(),
                fertilizer_key,
            );
            let light_key = get_value(seed_mapping_advanced.water_to_light.clone(), water_key);
            let temp_key = get_value(seed_mapping_advanced.light_to_temp.clone(), light_key);
            let hum_key = get_value(seed_mapping_advanced.temp_to_hum.clone(), temp_key);
            let location = get_value(seed_mapping_advanced.hum_to_location.clone(), hum_key);

            let mut output = output.lock().unwrap();
            if location < *output {
                *output = location;
            }
        }
    });

    println!("\tSub solution 2: {:?}", output);
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

    next_key
}
