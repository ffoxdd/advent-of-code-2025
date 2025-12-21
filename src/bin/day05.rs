use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day05::IngredientDatabase;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(5)?;
    let ingredient_database = IngredientDatabase::try_from(input)?;

    println!("Fresh ingredient count: {}", ingredient_database.fresh_ingredient_count());
    println!("Known fresh ingredient count: {}", ingredient_database.known_fresh_ingredient_count());

    Ok(())
}

