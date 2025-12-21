use std::ops::RangeInclusive;
use std::fmt;

#[derive(Debug)]
pub struct IngredientDatabase {
    fresh_ingredient_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl IngredientDatabase {
    pub fn new(fresh_ingredient_ranges: Vec<RangeInclusive<u64>>, available_ingredients: Vec<u64>) -> Self {
        Self {
            fresh_ingredient_ranges: Self::remove_overlaps(fresh_ingredient_ranges),
            available_ingredients: available_ingredients,
        }
    }

    pub fn fresh_ingredient_count(&self) -> usize {
        self.fresh_ingredients().count()
    }

    pub fn known_fresh_ingredient_count(&self) -> u64 {
        self.fresh_ingredient_ranges
            .iter()
            .map(|range| Self::range_measure(range))
            .sum()
    }

    fn fresh_ingredients(&self) -> impl Iterator<Item = u64> {
        self.available_ingredients
            .iter()
            .filter(|&ingredient| self.is_fresh(*ingredient))
            .copied()
    }

    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ingredient_ranges
            .iter()
            .any(|range| range.contains(&ingredient))
    }

    fn parse_range(line: &str) -> Result<RangeInclusive<u64>, String> {
        let (min, max) = line.split_once('-').ok_or("Invalid line")?;

        let min = Self::parse_u64(min)?;
        let max = Self::parse_u64(max)?;

        Ok(min..=max)
    }

    fn parse_u64(s: &str) -> Result<u64, String> {
        s.parse::<u64>().map_err(|e| e.to_string())
    }

    fn range_measure(range: &RangeInclusive<u64>) -> u64 {
        range.end() - range.start() + 1
    }

    fn remove_overlap(
        range_to_adjust: RangeInclusive<u64>,
        existing_range: &RangeInclusive<u64>,
    ) -> Result<RangeInclusive<u64>, ()> {
        let mut start = *range_to_adjust.start();
        let mut end = *range_to_adjust.end();

        if existing_range.contains(&start) {
            start = *existing_range.end() + 1;
        }

        if existing_range.contains(&end) {
            end = *existing_range.start() - 1;
        }

        if start > end {
            return Err(());
        }

        Ok(start..=end)
    }

    fn remove_overlaps(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        let mut sorted_ranges = ranges;

        // sort by descending measure so that completely contained ranges can just be dropped
        sorted_ranges.sort_by_key(|r| -(Self::range_measure(r) as i64));

        let mut result = Vec::new();

        for range_to_add in sorted_ranges {
            let adjusted_range = result.iter()
                .try_fold(range_to_add, |range, existing_range| {
                    Self::remove_overlap(range, existing_range)
                });

            if let Ok(range) = adjusted_range {
                result.push(range);
            }
        }

        result
    }
}

impl TryFrom<Vec<String>> for IngredientDatabase {
    type Error = String;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();

        let fresh_ingredient_ranges: Vec<RangeInclusive<u64>> = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| Self::parse_range(&line))
            .collect::<Result<_, _>>()?;

        let available_ingredients: Vec<u64> = iter
            .map(|line| Self::parse_u64(&line))
            .collect::<Result<_, _>>()?;

        Ok(Self::new(fresh_ingredient_ranges, available_ingredients))
    }
}

impl fmt::Display for IngredientDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Fresh ingredients:")?;

        for range in self.fresh_ingredient_ranges.iter() {
            writeln!(f, "  {}..={}", range.start(), range.end())?;
        }

        writeln!(f, "Available ingredients:")?;

        for ingredient in self.available_ingredients.iter() {
            writeln!(f, "  {}", ingredient)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::IngredientDatabase;

    #[test]
    fn reports_fresh_ingredient_count() {
        let db = IngredientDatabase {
            fresh_ingredient_ranges: vec![10..=20, 30..=40],
            available_ingredients: vec![5, 10, 15, 20, 25, 35],
        };

        assert_eq!(db.fresh_ingredient_count(), 4);
    }

    #[test]
    fn reports_known_fresh_ingredient_count() {
        let disjoint = IngredientDatabase::new(
            vec![10..=20, 30..=40],
            vec![],
        );

        let overlapping = IngredientDatabase::new(
            vec![10..=20, 15..=25],
            vec![],
        );

        let overlapping_at_end = IngredientDatabase::new(
            vec![10..=20, 20..=25],
            vec![],
        );

        let overlapping_at_start = IngredientDatabase::new(
            vec![10..=20, 5..=10],
            vec![],
        );

        let completely_contained = IngredientDatabase::new(
            vec![10..=30, 15..=25],
            vec![],
        );

        assert_eq!(disjoint.known_fresh_ingredient_count(), 22); // 11 + 11
        assert_eq!(overlapping.known_fresh_ingredient_count(), 16); // 10..25
        assert_eq!(overlapping_at_end.known_fresh_ingredient_count(), 16); // 10..25
        assert_eq!(overlapping_at_start.known_fresh_ingredient_count(), 16); // 5..20
        assert_eq!(completely_contained.known_fresh_ingredient_count(), 21); // 10..30
    }
}