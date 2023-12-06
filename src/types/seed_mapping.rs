use std::str::FromStr;

use super::node::Node;

#[derive(Clone)]
pub struct SeedMapping {
    pub seeds: Vec<i64>,
    pub seed_to_soil: Vec<Node>,
    pub soil_to_fertilizer: Vec<Node>,
    pub fertilizer_to_water: Vec<Node>,
    pub water_to_light: Vec<Node>,
    pub light_to_temp: Vec<Node>,
    pub temp_to_hum: Vec<Node>,
    pub hum_to_location: Vec<Node>,
}

pub struct SeedMappingAdvanced {
    pub seeds: Vec<Node>,
    pub seed_to_soil: Vec<Node>,
    pub soil_to_fertilizer: Vec<Node>,
    pub fertilizer_to_water: Vec<Node>,
    pub water_to_light: Vec<Node>,
    pub light_to_temp: Vec<Node>,
    pub temp_to_hum: Vec<Node>,
    pub hum_to_location: Vec<Node>,
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
                let destination_start = *line_numbers.get(0).unwrap();
                let source_start = *line_numbers.get(1).unwrap();
                let range = *line_numbers.get(2).unwrap();

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
        }

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
