use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day03::BatteryBank;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(3)?;
    let battery_banks = BatteryBank::parse_all(&input)?;

    let sum: u128 = battery_banks
        .iter()
        .map(|bank| bank.maximum_joltage() as u128)
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}
