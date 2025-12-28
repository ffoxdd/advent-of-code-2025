use advent_of_code_2025::input_for_day;
use advent_of_code_2025::day11::DirectedGraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input_for_day(11)?;
    let graph = DirectedGraph::try_from(&input)?;

    let part_1_count = graph.paths_between("you", "out")?;
    println!("Part 1 paths: {}", part_1_count);

    let part_2_count = graph.paths_between_including("svr", "out", &vec!["dac", "fft"])?;
    println!("Part 2 paths: {}", part_2_count);

    Ok(())
}
