use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day04::FactoryFloor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(4)?;
    let mut factory_floor = FactoryFloor::try_from(&input)?;

    let original_roll_count = factory_floor.roll_count();

    println!("Factory Floor: \n{}", factory_floor);
    println!("Roll Count: {}", factory_floor.roll_count());
    println!("Accessible Rolls: {}", factory_floor.accessible_roll_count());

    println!("\n--------------------------------\n");

    factory_floor.remove_accessible_rolls();
    let final_roll_count = factory_floor.roll_count();

    println!("Final Factory Floor: \n{}", factory_floor);
    println!("Final roll count: {}", final_roll_count);
    println!("Removed rolls: {}", original_roll_count - final_roll_count);

    Ok(())
}
