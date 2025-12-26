#[derive(Debug)]
pub struct Machine {
    indicator_lights_target: Vec<bool>,
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u16>,
}

impl Machine {
    fn new(indicator_lights_target: Vec<bool>, buttons: Vec<Vec<usize>>, joltages: Vec<u16>) -> Self {
        let indicator_lights = vec![false; indicator_lights_target.len()];

        Self {
            indicator_lights_target,
            indicator_lights,
            buttons,
            joltages,
        }
    }

    pub fn parse_all(values: &Vec<String>) -> Result<Vec<Machine>, String> {
        values.iter().map(|value| Machine::try_from(value)).collect()
    }
}

impl TryFrom<&String> for Machine {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let sections: Vec<&str> =  value.split_whitespace().collect();

        let indicator_lights_target_string = sections[0];
        let buttons_strings = sections[1..sections.len() - 1].to_vec();
        let joltages_string = sections[sections.len() - 1];

        let indicator_lights_target: Vec<bool> = indicator_lights_target_string
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(format!("invalid character: {}", c))
            })
            .collect::<Result<Vec<bool>, String>>()?;

        let buttons: Vec<Vec<usize>> = buttons_strings.iter()
            .map(|buttons_string| {
                buttons_string
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.parse::<usize>().map_err(|e| format!("invalid number: {}", e)))
                    .collect::<Result<Vec<usize>, String>>()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()?;

        let joltages: Vec<u16> = joltages_string
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .map(|s| s.parse::<u16>().map_err(|e| format!("invalid number: {}", e)))
            .collect::<Result<Vec<u16>, String>>()?;

        Ok(Self::new(indicator_lights_target, buttons, joltages))
    }
}