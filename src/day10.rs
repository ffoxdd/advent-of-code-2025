use std::collections::HashSet;

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

    pub fn indicator_light_min_button_press_count(&self) -> usize {
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

    pub fn joltage_min_button_press_count(&self) -> usize {

        // target: (3, 2, 1)
        //
        // (1, 0, 0), (0, 1, 0), (0, 0, 1) -> obvious (disjoint)
        // (1, 1, 0), (1, 0, 0), (0, 0, 1) -> max (2)
        // wait, can we enumerate all of the options?
        // for each position, you can have any integer partition for any button that includes that
        // that means we enumerate all of the integer partitions

        // (3, 2, 1) -> worst case: 6 choices.
        // | 3 can be split between 0 and 1, 3(0), 2(0) or 1(0)
        //     | 2 can be split between 0 and 2, 2(0) or 1(0)

        // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        // (0, 0, 0, 1), (0, 1, 0, 1), (0, 0, 1, 0), (1, 1, 0, 0)
        // {3,5,4,7}
        //  |_ can only be 3(3)
        //    |_ between 1,3: 5(1), 4(1), 3(1), 2(1), 1(1), 0(1)
        //      |_ only 4(2)
        //        |_ only 7(0)

        // what about cases where there are more than one option for a position?
        // (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2} -- this has that
        // position 0: target 7
        //   buttons 0, 2, 3; 7(0), 6(0)1(2), 6(0)0(2), 5(0)2(2), 5(0)1(2), 5(0)0(2), ...
        //     4(0)3(2), 4(0)2(2), 4(0)1(2), 4(0)0(2)
        //   7 choices for button 0..
        //   7(0) has 1 total way
        //   6(0) has 2 total ways
        //   5(0) has 3 total ways
        //   4(0) has 4 total ways ...
        //   0(0):
        // so 7 partitioned between 3 has 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 = 36 ways

        // for 2 buttons, there are n + 1 partitions
        // for 3 buttons there are

        // (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

        // for 0 buttons, there is 1 way p(0, 1) = 0
        // for 1 button there is sum(0..n) {i * 1} way
        // for 2 buttons there are sum(0..n) {p(1, i) }

        // I'm guessing that for n buttons, p(n, k) = sum(0..k) {p(n-1, i)}, p(0, _) = 1
        // this is the weak composition problem


        let mut known_states: HashSet<Vec<u16>> = vec![self.initial_joltage_state()].into_iter().collect();
        let mut visited_states: HashSet<Vec<u16>> = HashSet::new();
        let mut button_press_count = 0;

        println!("Machine: {:?}", self);

        loop {
            let states_to_process: Vec<Vec<u16>> = known_states.difference(&visited_states).cloned().collect();
            button_press_count += 1;

            for starting_state in states_to_process {
                println!("Starting state: {:?}", starting_state);

                for button in self.buttons.iter() {
                    let new_state = Self::apply_joltage_button_press(&starting_state, button);

                    println!("New state: {:?}", new_state);

                    if !self.valid_joltage_state(&new_state) {
                        continue;
                    }

                    if visited_states.contains(&new_state) {
                        continue;
                    }

                    if new_state == self.joltages_target {
                        return button_press_count;
                    }

                    known_states.insert(new_state);
                }

                visited_states.insert(starting_state);
            }
        }
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

    fn initial_joltage_state(&self) -> Vec<u16> {
        vec![0; self.joltages_target.len()]
    }

    fn apply_joltage_button_press(state: &Vec<u16>, button: &Vec<usize>) -> Vec<u16> {
        let mut new_state = state.clone();

        for joltage_index in button {
            new_state[*joltage_index] += 1;
        }

        new_state
    }

    fn valid_joltage_state(&self,state: &Vec<u16>) -> bool {
        state.iter()
            .zip(self.joltages_target.iter())
            .all(|(state_value, target_value)| state_value <= target_value)
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