use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day01::Safe;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut safe = Safe::new();
    let input = input_for_day(1)?;

    safe.apply_instructions(input)?;

    println!("Zero Position Count: {}", safe.zero_position_count());
    println!("Zero Pass Count: {}", safe.zero_pass_count());

    Ok(())
}
