use crate::utils;

fn part1() -> usize {
    let (mut left, mut right) = parse_input_to_arrays();

    left.sort();
    right.sort();

    let mut distance = 0;
    for (index, left_element) in left.iter().enumerate() {
        let right_element = right[index];

        if *left_element > right_element {
            distance += *left_element - right_element;
        } else if right_element > *left_element {
            distance += right_element - *left_element;
        }
    }

    return distance;
}

fn part2() -> usize {
    let (left, right) = parse_input_to_arrays();

    return left.iter().fold(0, |acc, left_element| {
        let count = right.iter().filter(|x| *x == left_element).count();
        return acc + (left_element * count);
    });
}

fn parse_input_to_arrays() -> (Vec<usize>, Vec<usize>) {
    let input = utils::input::read(2024, 1);
    let lines = input.split("\n");

    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for line in lines {
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        array1.push(splitted[0].parse::<usize>().unwrap());
        array2.push(splitted[1].parse::<usize>().unwrap());
    }

    if array1.len() != array2.len() {
        panic!("Arrays are not of the same length !");
    }

    return (array1, array2);
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
