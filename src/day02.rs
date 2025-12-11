use std::ops::RangeInclusive;
use itertools::Itertools;

pub fn answer(input: &[String]) -> Result<u64, Box<dyn std::error::Error>> {
    let bad_ids = input
        .iter()
        .flat_map(|s| s.split(","))
        .map(|s: &str| parse_range(s.trim()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .filter(|&id| is_repeated(id))
        ;

    Ok(bad_ids.sum())
}

pub fn parse_range(string: &str) -> Result<RangeInclusive<u64>, Box<dyn std::error::Error>> {
    let (start_string, end_string) = string.trim().split_once("-")
        .ok_or_else(|| format!("Invalid range format: '{}'", string))?;

    let start = start_string.parse()?;
    let end= end_string.parse()?;

    Ok(start..=end)
}

pub fn is_repeated(number: u64) -> bool {
    let string = number.to_string();
    (1..=string.len() / 2).any(|size| repeats_of_size(&string, size))
}

pub fn repeats_of_size(string: &str, size: usize) -> bool {
    if size == 0 {
        return false;
    }

    string
        .chars()
        .chunks(size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .unique()
        .count() <= 1
}
