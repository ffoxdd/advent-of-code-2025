use std::collections::HashSet;

#[derive(Debug)]
pub struct Machine {
    indicator_lights_target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u16>,
}

impl Machine {
    fn new(indicator_lights_target: Vec<bool>, buttons: Vec<Vec<usize>>, joltages: Vec<u16>) -> Self {
        Self {
            indicator_lights_target,
            buttons,
            joltages,
        }
    }

    pub fn parse_all(values: &Vec<String>) -> Result<Vec<Machine>, String> {
        values.iter().map(|value| Machine::try_from(value)).collect()
    }

    pub fn min_button_press_count(&self) -> usize {
        let mut known_states: HashSet<Vec<bool>> = vec![self.initial_state()].into_iter().collect();
        let mut visited_states: HashSet<Vec<bool>> = HashSet::new();
        let mut button_press_count = 0;

        loop {
            let states_to_process: Vec<Vec<bool>> = known_states.difference(&visited_states).cloned().collect();
            button_press_count += 1;

            for starting_state in states_to_process {
                for button in self.buttons.iter() {
                    let new_state = Self::apply_button_press(&starting_state, button);

                    if visited_states.contains(&new_state) {
                        continue;
                    }

                    if new_state == self.indicator_lights_target {
                        return button_press_count;
                    }

                    known_states.insert(new_state);
                }

                visited_states.insert(starting_state);
            }
        }
    }

    fn initial_state(&self) -> Vec<bool> {
        vec![false; self.indicator_lights_target.len()]
    }

    fn apply_button_press(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
        let mut new_state = state.clone();

        for indicator_light_index in button {
            new_state[*indicator_light_index] = !new_state[*indicator_light_index];
        }

        new_state
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