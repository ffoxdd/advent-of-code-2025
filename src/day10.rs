use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct Machine {
    indicator_lights_target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages_target: Vec<u16>,
}

impl Machine {
    fn new(indicator_lights_target: Vec<bool>, buttons: Vec<Vec<usize>>, joltages_target: Vec<u16>) -> Self {
        Self {
            indicator_lights_target,
            buttons,
            joltages_target,
        }
    }

    pub fn parse_all(values: &Vec<String>) -> Result<Vec<Machine>, String> {
        values.iter().map(|value| Machine::try_from(value)).collect()
    }

    pub fn min_indicator_light_button_presses(&self) -> usize {
        let mut known_states: HashSet<Vec<bool>> = vec![self.initial_indicator_light_state()].into_iter().collect();
        let mut visited_states: HashSet<Vec<bool>> = HashSet::new();
        let mut button_press_count = 0;

        loop {
            let states_to_process: Vec<Vec<bool>> = known_states.difference(&visited_states).cloned().collect();
            button_press_count += 1;

            for starting_state in states_to_process {
                for button in self.buttons.iter() {
                    let new_state = Self::apply_indicator_light_button_press(&starting_state, button);

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

    pub fn min_joltage_button_presses(&self) -> u16 {
        ButtonPressSequence::cartesian_product(&self.valid_button_press_sequences_by_position()).iter()
            .map(|sequence| sequence.button_presses())
            .min()
            .unwrap()
    }

    fn buttons_by_position(&self) -> Vec<Vec<usize>> {
        (0..self.joltages_target.len()).map(|position| {
            self.buttons.iter().enumerate().filter_map(|(button_index, button)|
                if button.contains(&position) {
                    Some(button_index)
                } else {
                    None
                }
            ).collect()
        }).collect()
    }

    fn valid_button_press_sequences_by_position(&self) -> Vec<Vec<ButtonPressSequence>> {
        let buttons_by_position = self.buttons_by_position();

        buttons_by_position.iter().zip(self.joltages_target.iter()).map(|(buttons, target)| {
            let partitions = Self::weak_partitions(*target, buttons.len());

            partitions.iter().map(|partition| {
                let button_counts = partition.iter()
                    .enumerate()
                    .map(|(group_index, count)| (buttons[group_index], *count));

                ButtonPressSequence::new(button_counts)
            }).collect()
        }).collect()
    }

    fn weak_partitions(target: u16, parts: usize) -> Vec<Vec<u16>> {
        let partitions: Vec<Vec<u16>> = vec![Vec::new(); 1];
        Self::build_weak_partitions(target, parts, &partitions)
    }

    fn build_weak_partitions(target: u16, parts: usize, partitions: &Vec<Vec<u16>>) -> Vec<Vec<u16>> {
        if parts == 1 {
            return partitions.into_iter().map(|partition| {
                [partition.as_slice(), &[target]].concat()
            }).collect();
        }

        (0..=target).flat_map(|next_partition_size| {
            let new_partitions: Vec<Vec<u16>> = partitions.iter().map(|partition| {
                [partition.as_slice(), &[next_partition_size]].concat()
            }).collect();

            Self::build_weak_partitions(
                target - next_partition_size,
                parts - 1,
                &new_partitions
            )
        }).collect()
    }

    fn initial_indicator_light_state(&self) -> Vec<bool> {
        vec![false; self.indicator_lights_target.len()]
    }

    fn apply_indicator_light_button_press(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
        let mut new_state = state.clone();

        for indicator_light_index in button {
            new_state[*indicator_light_index] = !new_state[*indicator_light_index];
        }

        new_state
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct ButtonPressSequence {
    counts_by_button: HashMap<usize, u16>,
}

impl ButtonPressSequence {
    pub fn new<I: IntoIterator<Item = (usize, u16)>>(button_counts: I) -> Self {
        Self {
            counts_by_button: button_counts.into_iter()
                .filter(|(_, count)| *count > 0)
                .collect(),
        }
    }

    pub fn button_presses(&self) -> u16 {
        self.counts_by_button.values().sum()
    }

    pub fn compatible_with(&self, other: &ButtonPressSequence) -> bool {
        self.counts_by_button.iter().all(|(button, count)| {
            match other.counts_by_button.get(button) {
                Some(other_count) => *count == *other_count,
                None => true,
            }
        })
    }

    pub fn add(&self, other: &ButtonPressSequence) -> Self {
        let mut new_counts_by_button = self.counts_by_button.clone();
        new_counts_by_button.extend(other.counts_by_button.iter().map(|(k, v)| (*k, *v)));
        Self { counts_by_button: new_counts_by_button }
    }

    pub fn cartesian_product(groups: &Vec<Vec<ButtonPressSequence>>) -> Vec<ButtonPressSequence> {
        groups.iter().cloned().reduce(|mut product, group| {
            product = product.iter().flat_map(|sequence| {
                let sequence = sequence.clone();

                group.iter().filter_map(move |other_sequence| {
                    if !sequence.compatible_with(other_sequence) {
                        return None;
                    }

                    Some(sequence.add(other_sequence))
                })
            }).collect::<Vec<ButtonPressSequence>>();

            product
        }).unwrap_or_else(|| Vec::new())
    }
}

impl TryFrom<&String> for Machine {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let sections: Vec<&str> =  value.split_whitespace().collect();

        let indicator_lights_target_string = sections[0];
        let buttons_strings = sections[1..sections.len() - 1].to_vec();
        let joltages_target_string = sections[sections.len() - 1];

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

        let joltages_target: Vec<u16> = joltages_target_string
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .map(|s| s.parse::<u16>().map_err(|e| format!("invalid number: {}", e)))
            .collect::<Result<Vec<u16>, String>>()?;

        Ok(Self::new(indicator_lights_target, buttons, joltages_target))
    }
}
