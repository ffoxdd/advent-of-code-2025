use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day10::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _input = input_for_day(10)?;

    let machines = Machine::parse_all(&_input)?;

    let indicator_light_min_button_press_count: usize = machines.iter()
        .map(|machine| machine.indicator_light_min_button_press_count())
        .sum();

    let joltage_min_button_press_count: usize = machines.iter()
        .map(|machine| machine.joltage_min_button_press_count())
        .sum();

    println!("Indicator lights minimum button press count: {}", indicator_light_min_button_press_count);
    println!("Joltages minimum button press count: {}", joltage_min_button_press_count);

    Ok(())
}
