use std::fs;
use std::path::PathBuf;
use std::io;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub mod day01;
pub mod day02;
pub mod day03;

pub fn input_for_day(day: u8) -> io::Result<Vec<String>> {
    let directory = PathBuf::from(MANIFEST_DIR).join("input");
    let filename = format!("day{:02}.txt", day);
    let path = directory.join(filename);
    let content = fs::read_to_string(&path)?;
    let lines = content.lines().map(String::from).collect();

    Ok(lines)
}
