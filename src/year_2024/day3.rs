use crate::utils;
use regex::Regex;

fn part1() -> usize {
    let input = utils::input::read(2024, 3);

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    return re.find_iter(input.as_str()).fold(0, |acc, current| {
        return acc + apply_multiplication_from_string(&input[current.range()]);
    });
}

fn part2() -> usize {
    let input = utils::input::read(2024, 3);

    let mut can_process = true;
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    return re.find_iter(input.as_str()).fold(0, |acc, current| {
        let current_value = &input[current.range()];
        if current_value == "do()" {
            can_process = true;
            return acc;
        }
        if current_value == "don't()" {
            can_process = false;
            return acc;
        }

        if can_process {
            return acc + apply_multiplication_from_string(current_value);
        }

        return acc;
    });
}

fn apply_multiplication_from_string(value: &str) -> usize {
    let value = value
        .to_string()
        .replace("mul(", "")
        .replace(")", "")
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return value[0] * value[1];
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
