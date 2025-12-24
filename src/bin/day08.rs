use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day08::Playground;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(8)?;
    let mut playground = Playground::try_from(&input)?;

    let pairs = playground.closest_pairs();

    for pair in pairs.iter().take(1000) {
        playground.connect(*pair);
    }

    let part_1_answer: u32 = playground.circuits()
        .sorted_by_key(|circuit| -(circuit.len() as i32))
        .take(3)
        .map(|circuit| circuit.len() as u32)
        .product();

    println!("Part 1 Answer: {}", part_1_answer);

    for pair in pairs {
        playground.connect(pair);

        if playground.circuits().count() == 1 {
            let part_2_answer = playground.x(pair.0) * playground.x(pair.1);
            println!("Part 2 Answer: {}", part_2_answer);
            break;
        }
    }

    Ok(())
}
