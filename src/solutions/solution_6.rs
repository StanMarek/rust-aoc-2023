pub fn solve() {
    println!("Day 6");

    sub_solution_1();
    sub_solution_2();
}

fn parse_input() -> ((Vec<u32>, Vec<u32>), (u64, u64)) {
    let input = include_str!("../inputs/day6/input.txt");

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    let mut times = Vec::new();
    let mut distances = Vec::new();
    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                let split = line
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .split_whitespace();

                time = split.clone().collect::<String>().parse::<u64>().unwrap();
                times = split
                    .clone()
                    .map(|dist| dist.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            }
            1 => {
                let split = line
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .split_whitespace();

                distance = split.clone().collect::<String>().parse::<u64>().unwrap();
                distances = split
                    .clone()
                    .map(|dist| dist.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            }
            _ => (),
        };
    }

    ((times, distances), (time, distance))
}

fn sub_solution_1() {
    let ((times, distances), _) = parse_input();
    let output = times
        .into_iter()
        .zip(distances)
        .map(|(time, dist)| {
            (0..time)
                .filter_map(|speed| {
                    let distance = (time - speed) * speed;
                    (distance > dist).then_some(distance)
                })
                .count()
        })
        .product::<usize>();

    println!("\tSub solution 1: {output}");
}

fn sub_solution_2() {
    let (_, (time, distance)) = parse_input();

    let output = (0..time)
        .filter_map(|speed| {
            let dist = (time - speed) * speed;
            (dist > distance).then_some(dist)
        })
        .count();

    println!("\tSub solution 2: {output}");
}
