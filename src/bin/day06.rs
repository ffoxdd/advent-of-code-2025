use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day06::{Worksheet, Part1, Part2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(6)?;

    let worksheet_part1: Worksheet<Part1> = Worksheet::try_from(&input)?;
    let worksheet_part2: Worksheet<Part2> = Worksheet::try_from(&input)?;

    println!("Answer (Part 1): {}", worksheet_part1.answer());
    println!("Answer (Part 2): {}", worksheet_part2.answer());

    Ok(())
}
