use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day07::Manifold;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(7)?;
    let mut manifold = Manifold::try_from(&input)?;

    println!("Manifold:\n{}", manifold);

    manifold.extend_beam();

    println!("Updated Manifold:\n{}", manifold);
    println!("Split Count: {}", manifold.split_count());
    println!("Timeline Count: {}", manifold.timeline_count());

    Ok(())
}

