use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day10::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _input = input_for_day(10)?;

    let machines = Machine::parse_all(&_input)?;

    let min_indicator_light_button_presses: usize = machines.iter()
        .map(|machine| machine.min_indicator_light_button_presses())
        .collect::<Result<Vec<usize>, String>>()?
        .iter()
        .sum();

    let min_joltage_button_presses: u16 = machines.iter()
        .map(|machine| machine.min_joltage_button_presses())
        .collect::<Result<Vec<u16>, _>>()?
        .iter()
        .sum();

    println!("Minimum indicator lights button presses: {}", min_indicator_light_button_presses);
    println!("Minimum joltage button presses: {}", min_joltage_button_presses);

    Ok(())
}
