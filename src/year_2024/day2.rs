use crate::utils;

fn part1() -> usize {
    let input = parse_input();

    return input.iter().fold(0, |acc, line| {
        let value = if is_increasing(line) || is_decreasing(line) {
            1
        } else {
            0
        };

        return acc + value;
    });
}

fn part2() -> usize {
    let input = parse_input();

    return input.iter().fold(0, |acc, line| {
        let variations = create_variations(line);
        for variation in &variations {
            if is_increasing(variation) || is_decreasing(variation) {
                return acc + 1;
            }
        }

        return acc;
    });
}

fn create_variations(line: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut variations = Vec::new();
    variations.push(line.clone());

    for (index, _) in line.iter().enumerate() {
        let variation = line
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, e)| *e)
            .collect::<Vec<usize>>();

        variations.push(variation);
    }

    return variations;
}

fn is_increasing(line: &Vec<usize>) -> bool {
    if line[1] < line[0] {
        return false;
    }

    let mut current = line[0];
    for element in line.iter().skip(1) {
        if *element == current {
            return false;
        }

        if *element < current || *element > current + 3 {
            return false;
        }

        current = *element;
    }

    return true;
}

fn is_decreasing(line: &Vec<usize>) -> bool {
    if line[0] < line[1] {
        return false;
    }

    let mut current = line[0];
    for element in line.iter().skip(1) {
        if *element == current {
            return false;
        }

        if *element > current || *element + 3 < current {
            return false;
        }

        current = *element;
    }

    return true;
}

fn parse_input() -> Vec<Vec<usize>> {
    let input = utils::input::read(2024, 2);
    return input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|e| e.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
