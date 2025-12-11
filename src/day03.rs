use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct BatteryBank {
    batteries: Vec<Battery>,
}

impl BatteryBank {
    const ACTIVE_BATTERY_COUNT: usize = 12;

    pub fn new(batteries: Vec<Battery>) -> Result<Self, String> {
        if batteries.len() < Self::ACTIVE_BATTERY_COUNT {
            return Err(
                format!("BatteryBank must have at least {} batteries, got {}",
                Self::ACTIVE_BATTERY_COUNT, batteries.len())
            );
        }

        Ok(Self { batteries })
    }

    pub fn parse_all(strings: &[String]) -> Result<Vec<Self>, String> {
        strings
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn maximum_joltage(&self) -> u64 {
        let indices = self.max_battery_indices();
        self.joltage_from_indices(&indices)
    }

    fn max_battery_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = vec![];
        let mut start = 0;

        for index in 0..Self::ACTIVE_BATTERY_COUNT {
            let end = self.batteries.len() - Self::ACTIVE_BATTERY_COUNT + index + 1;
            let max_index = self.max_battery_index(start, end);

            indices.push(max_index);
            start = max_index + 1;
        }

        indices
    }

    fn joltage_from_indices(&self, indices: &[usize]) -> u64 {
        indices
            .iter()
            .map(|&index| self.batteries[index].joltage)
            .map(|joltage| joltage.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    fn max_battery_index(&self, start: usize, end: usize) -> usize {
        Self::max_index(&self.batteries, start, end)
    }

    fn max_index<T: Ord>(items: &[T], start: usize, end: usize) -> usize {
        let mut max_index = start;
        let mut max_value = &items[start];

        for index in start..end {
            if items[index] > *max_value {
                max_value = &items[index];
                max_index = index;
            }
        }

        max_index
    }
}

impl fmt::Display for BatteryBank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for battery in &self.batteries {
            write!(f, "{}", battery.joltage)?;
        }

        Ok(())
    }
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<Battery> = s
            .chars()
            .map(|c| Battery::try_from(c))
            .collect::<Result<Vec<_>, _>>()?;

        BatteryBank::new(batteries)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Battery {
    joltage: u8,
}

impl Battery {
    pub fn new(joltage: u8) -> Result<Self, String> {
        Self::try_from(joltage)
    }
}

impl TryFrom<char> for Battery {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let joltage = c.to_digit(10)
            .ok_or_else(|| format!("Character '{}' is not a digit", c))?
            as u8;

        Battery::try_from(joltage)
    }
}

impl TryFrom<u8> for Battery {
    type Error = String;

    fn try_from(joltage: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&joltage) {
            return Err(format!("Joltage must be between 1 and 9, got {}", joltage));
        }

        Ok(Self { joltage })
    }
}
