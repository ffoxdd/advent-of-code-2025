pub struct Worksheet<P> {
    problems: Vec<Problem>,
    _part: std::marker::PhantomData<P>,
}

impl<P> Worksheet<P> {
    pub fn answer(&self) -> u64 {
        self.problems.iter().map(|problem| problem.answer()).sum()
    }

    fn split_at_columns(lines: &Vec<String>) -> Vec<Vec<String>> {
        let mut result = vec![Vec::<String>::default(); lines.len()];
        let mut number_start = 0;
        let mut index = 0;

        while index <= lines[0].len() {
            let all_whitespace = lines.iter()
                .map(|line| line.chars().nth(index).unwrap_or(' '))
                .all(|c| c.is_whitespace());

            if all_whitespace {
                for line_index in 0..lines.len() {
                    let word = lines[line_index][number_start..index].to_string();
                    result[line_index].push(word);
                }

                number_start = index + 1;
            }

            index += 1;
        }

        result
    }
}

impl<P: GridTransformation> TryFrom<&Vec<String>> for Worksheet<P> {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let problem_grid = Self::split_at_columns(value);
        let transformed_grid = P::transform_grid(&problem_grid);

        let problems: Vec<Problem> = transformed_grid.iter()
            .map(|row| Problem::try_from(row))
            .collect::<Result<Vec<Problem>, String>>()?;

        Ok(Worksheet {
            problems,
            _part: std::marker::PhantomData,
        })
    }
}

pub trait GridTransformation {
    fn transform_grid(grid: &[Vec<String>]) -> Vec<Vec<String>>;
}

pub struct Part1;

impl Part1 {
    fn trim_row(row: &[String]) -> Vec<String> {
        row.iter().map(|s| s.trim().to_string()).collect()
    }
}

impl GridTransformation for Part1 {
    fn transform_grid(grid: &[Vec<String>]) -> Vec<Vec<String>> {
        pivot(grid).iter()
            .map(|row| Part1::trim_row(row))
            .collect()
    }
}

pub struct Part2;

impl Part2 {
    fn pivot_characters(number_strings: &[String]) -> Vec<String> {
        let number_grid: Vec<Vec<char>> = number_strings.iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();

        let padded_number_grid = pad(number_grid, ' ');
        let pivoted_number_grid = pivot(&padded_number_grid);

        pivoted_number_grid.iter()
            .map(|row| row.iter().collect::<String>().trim().to_string())
            .collect()
    }

    fn transform_column(column: &[String]) -> Vec<String> {
        let operation = column.last().unwrap().trim().to_string();
        let number_strings = &column[..column.len() - 1];

        let mut result = Self::pivot_characters(number_strings);
        result.push(operation);

        result
    }
}

impl GridTransformation for Part2 {
    fn transform_grid(grid: &[Vec<String>]) -> Vec<Vec<String>> {
        let columns = pivot(grid);

        columns.iter()
            .map(|column| Part2::transform_column(column))
            .collect()
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(format!("Invalid operation: {}", value)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Problem {
    numbers: Vec<u16>,
    operation: Operation,
}

impl Problem {
    fn parse_u16s(value: &[String]) -> Result<Vec<u16>, String> {
        value.iter()
            .map(|s| s.parse::<u16>().map_err(|e| format!("Invalid number: {}", e)))
            .collect()
    }

    fn answer(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().map(|&n| n as u64).sum(),
            Operation::Multiply => self.numbers.iter().map(|&n| n as u64).product(),
        }
    }
}

impl TryFrom<&Vec<String>> for Problem {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let last_element = value.last().ok_or("No last element")?;
        let operation = Operation::try_from(last_element.as_str())?;
        let numbers = Self::parse_u16s(&value[..value.len() - 1])?;

        Ok(Problem { numbers, operation })
    }
}

fn pivot<T: Clone>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    for i in 0..input[0].len() {
        let mut column = Vec::new();

        for row in input.iter() {
            column.push(row[i].clone());
        }

        result.push(column);
    }

    result
}

fn pad<T: Clone>(grid: Vec<Vec<T>>, padding_value: T) -> Vec<Vec<T>> {
    let max_length = grid.iter().map(|row| row.len()).max().unwrap();

    grid.into_iter().map(|row| {
        let padding = vec![padding_value.clone(); max_length - row.len()];
        row.into_iter().chain(padding.into_iter()).collect()
    }).collect()
}
