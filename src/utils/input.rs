use std::fs;

pub fn read(year: usize, day: usize) -> String {
    let path = format!("{}/inputs/{}/{:0>2}.txt", env!("PWD"), year, day);
    return fs::read_to_string(path)
        .expect("Could not read file")
        .trim()
        .to_string();
}
