use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day09::{Floor, Filter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _input = input_for_day(9)?;
    let floor = Floor::try_from(&_input)?;

    println!("Largest rectangle area: {}", floor.largest_rectangle_area(Filter::All));
    println!("Largest valid rectangle area: {}", floor.largest_rectangle_area(Filter::ValidOnly));

    Ok(())
}
