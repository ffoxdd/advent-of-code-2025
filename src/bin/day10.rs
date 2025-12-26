use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day10::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _input = input_for_day(10)?;

    let machines = Machine::parse_all(&_input)?;

    for machine in machines {
        println!("machine: {:?}", machine);
    }

    Ok(())
}
