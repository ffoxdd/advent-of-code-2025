use std::collections::HashSet;
use good_lp::*;
use itertools::izip;
use nalgebra::DMatrix;

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

    pub fn min_indicator_light_button_presses(&self) -> Result<usize, String> {
        let initial_state = self.initial_indicator_light_state();
        let transitions = self.indicator_light_transitions();

        let mut state_machine = StateMachine::new(
            transitions,
            self.indicator_lights_target.clone(),
            initial_state,
        );

        state_machine.min_transition_count()
    }

    fn indicator_light_transitions(&self) -> Vec<Box<dyn Fn(&Vec<bool>) -> Vec<bool>>> {
        self.buttons.iter()
            .cloned()
            .map(|button| {
                Box::new(move |state: &Vec<bool>|
                    Machine::apply_indicator_light_button_press(state, &button)
                ) as Box<dyn Fn(&Vec<bool>) -> Vec<bool>>
            })
            .collect()
    }

    pub fn min_joltage_button_presses(&self) -> Result<u16, String> {
        let basis_vectors = self.joltage_basis_vectors();
        let solver = ILPSolver::new(basis_vectors, self.joltages_target.clone());
        let solution = solver.solution().map_err(|e| e.to_string())?;

        Ok(solution.iter().sum())
    }

    fn joltage_basis_vectors(&self) -> Vec<Vec<u16>> {
        self.buttons.iter()
            .map(|button| (0..self.joltages_target.len())
                .map(|i| if button.contains(&i) { 1u16 } else { 0u16 })
                .collect())
            .collect()
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

struct StateMachine<S> {
    transitions: Vec<Box<dyn Fn(&S) -> S>>,
    target_state: S,
    known_states: HashSet<S>,
    visited_states: HashSet<S>,
    transition_count: usize,
}

impl<S: Clone + std::hash::Hash + Eq> StateMachine<S> {
    const MAX_TRANSITIONS: usize = 10000;

    pub fn new(transitions: Vec<Box<dyn Fn(&S) -> S>>, target_state: S, initial_state: S) -> Self {
        let known_states: HashSet<S> = vec![initial_state].into_iter().collect();
        let visited_states: HashSet<S> = HashSet::new();

        Self { transitions, target_state, known_states, visited_states, transition_count: 0 }
    }

    pub fn min_transition_count(&mut self) -> Result<usize, String> {
        loop {
            if self.transition_count > Self::MAX_TRANSITIONS {
                return Err(format!("Maximum transition count ({}) exceeded", Self::MAX_TRANSITIONS));
            }

            if let Some(count) = self.step() {
                return Ok(count);
            }
        }
    }

    fn step(&mut self) -> Option<usize> {
        let states_to_process = self.unprocessed_states();
        self.transition_count += 1;

        for starting_state in states_to_process {
            let new_states = self.all_states_from(&starting_state);

            for new_state in new_states {
                if let Some(count) = self.visit_state(new_state) {
                    return Some(count);
                }
            }

            self.visited_states.insert(starting_state);
        }

        None
    }

    fn all_states_from(&self, starting_state: &S) -> Vec<S> {
        self.transitions.iter()
            .map(|transition| (transition)(starting_state))
            .collect()
    }

    fn visit_state(&mut self, state: S) -> Option<usize> {
        if self.visited_states.contains(&state) {
            return None;
        }

        if state == self.target_state {
            return Some(self.transition_count);
        }

        self.known_states.insert(state);

        None
    }

    fn unprocessed_states(&self) -> Vec<S> {
        self.known_states.difference(&self.visited_states).cloned().collect()
    }
}

struct ILPSolver {
    basis_vectors: Vec<Vec<u16>>,
    target: Vec<u16>,
}

impl ILPSolver {
    pub fn new(basis_vectors: Vec<Vec<u16>>, target: Vec<u16>) -> Self {
        Self::assert_dimension(&basis_vectors, target.len(), "basis vector must have dimension");
        Self { basis_vectors, target }
    }

    pub fn solution(&self) -> Result<Vec<u16>, good_lp::ResolutionError> {
        let (problem, variables) = self.set_up_problem();
        let solution = problem.solve()?;

        let solution_vector = variables.iter()
            .map(|var| solution.value(*var) as u16)
            .collect();

        Ok(solution_vector)
    }

    fn assert_dimension(vectors: &[Vec<u16>], expected_dimension: usize, error_message: &str) {
        for vector in vectors.iter() {
            assert_eq!(vector.len(), expected_dimension, "{} {}", error_message, expected_dimension);
        }
    }

    fn set_up_problem(&self) -> (solvers::coin_cbc::CoinCbcProblem, Vec<Variable>) {
        let mut problem_variables = variables!();
        let variables = self.variables(&mut problem_variables);
        let objective = Self::objective(&variables);
        let mut problem = Self::problem(problem_variables, objective);
        let constraints = self.constraints(&variables);

        problem = Self::add_constraints(problem, &constraints);

        (problem, variables)
    }

    fn variables(&self, problem_variables: &mut ProblemVariables) -> Vec<Variable> {
        (0..self.basis_vectors.len())
            .map(|_| problem_variables.add(variable().integer().min(0)))
            .collect()
    }

    fn objective(variables: &[Variable]) -> Expression {
        variables.iter().copied().sum()
    }

    fn problem(problem_variables: ProblemVariables, objective: Expression) -> solvers::coin_cbc::CoinCbcProblem {
        let mut problem = problem_variables.minimise(objective).using(default_solver);
        Self::silence_logs(&mut problem);

        problem
    }

    fn constraints(&self, variables: &[Variable]) -> Vec<Constraint> {
        izip!(self.target.iter(), self.basis_expressions(variables))
            .map(|(target, expression)| constraint!(expression == *target))
            .collect()
    }

    fn add_constraints(
        problem: solvers::coin_cbc::CoinCbcProblem,
        constraints: &[Constraint]
    ) -> solvers::coin_cbc::CoinCbcProblem {
        constraints.iter()
            .fold(problem, |problem, constraint|
                problem.with(constraint.clone())
            )
    }

    fn silence_logs(problem: &mut solvers::coin_cbc::CoinCbcProblem) {
        problem.as_inner_mut().set_parameter("log", "0");
    }

    fn basis_expressions(&self, variables: &[Variable]) -> Vec<Expression> {
        self.basis_matrix().row_iter()
            .map(|row| {
                izip!(row.iter(), variables.iter().copied())
                .map(|(component, var)| (*component as f64) * var)
                .sum()
            })
            .collect()
    }

    fn basis_matrix(&self) -> DMatrix<u16> {
        DMatrix::from_fn(
            self.basis_vectors[0].len(),
            self.basis_vectors.len(),
            |row, col| self.basis_vectors[col][row]
        )
    }
}

