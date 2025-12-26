use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day10::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _input = input_for_day(10)?;

    let machines = Machine::parse_all(&_input)?;

    let minimal_button_presses_to_solve: usize = machines.iter()
        .map(|machine| machine.min_button_press_count())
        .sum();

    println!("Minimal button presses to solve: {}", minimal_button_presses_to_solve);

    Ok(())
}
