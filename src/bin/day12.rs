use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day12::TreeFarm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(12)?;

    let tree_farm = TreeFarm::try_from(&input)?;
    let valid_regions = tree_farm.valid_regions();

    println!("Valid regions: {}", valid_regions);

    Ok(())
}
