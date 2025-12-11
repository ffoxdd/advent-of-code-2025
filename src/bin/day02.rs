use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day02::answer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(2)?;
    println!("Answer: {}", answer(&input)?);
    Ok(())
}
