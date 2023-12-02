pub fn parse_digit_to_number(spelled_digit: &str) -> u32 {
    match spelled_digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    }
}

pub fn parse_color_to_max(spelled_digit: &str) -> u32 {
    match spelled_digit {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        n => n.parse::<u32>().unwrap(),
    }
}
