mod utils;
mod year_2024;

fn main() {
    let year = 2024;
    let day = 2;

    match year {
        2024 => match day {
            1 => year_2024::day1::run(),
            2 => year_2024::day2::run(),
            _ => panic!("day not found"),
        },
        _ => panic!("year not found"),
    };
}
