pub struct Manifold {
    grid: Vec<Vec<Cell>>
}

impl Manifold {
    pub fn split_count(&self) -> usize {
        self.grid.iter()
            .map(|row| Self::illuminated_split_count(row))
            .sum()
    }

    pub fn extend_beam(&mut self) {
        for row_index in 0..self.grid.len() - 1 {
            let next_row_index = row_index + 1;

            let (previous_rows, current_and_after) =
                self.grid.split_at_mut(next_row_index); // rust bs

            let previous_row = &previous_rows[row_index];
            let next_row = &mut current_and_after[0];

            for cell_index in 0..previous_row.len() {
                let cell = &previous_row[cell_index];

                match cell.cell_type {
                    CellType::Source => {
                        Self::update_cell(next_row, cell_index, true, cell.timeline_count);
                    }
                    CellType::Space if cell.illuminated => {
                        Self::update_cell(next_row, cell_index, true, cell.timeline_count);
                    }
                    CellType::Splitter => {
                        Self::update_cell(next_row, cell_index.wrapping_sub(1), true, cell.timeline_count);
                        Self::update_cell(next_row, cell_index, false, 0);
                        Self::update_cell(next_row, cell_index + 1, true, cell.timeline_count);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn timeline_count(&self) -> u64 {
        self.grid.iter()
            .last().unwrap().iter()
            .map(|cell| cell.timeline_count)
            .sum()
    }

    fn illuminated_split_count(row: &[Cell]) -> usize {
        row.iter()
            .filter(|cell| cell.cell_type == CellType::Splitter && cell.illuminated)
            .count()
    }

    fn update_cell(
        row: &mut[Cell],
        cell_index: usize,
        illuminated: bool,
        timeline_count: u64
    ) {
        let Some(cell) = row.get_mut(cell_index) else {
            return;
        };

        cell.illuminated = illuminated;
        cell.timeline_count = cell.timeline_count + timeline_count;
    }

    fn parse_line(line: &str) -> Result<Vec<Cell>, String> {
        line.chars()
            .map(|c| Cell::try_from(c))
            .collect::<Result<Vec<Cell>, String>>()
    }
}

impl TryFrom<&Vec<String>> for Manifold {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<Cell>> = value.iter()
            .map(|line| Self::parse_line(line))
            .collect::<Result<Vec<Vec<Cell>>, String>>()?;

        Ok(Self {grid})
    }
}

impl std::fmt::Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell.to_char())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
enum CellType {
    Source,
    Space,
    Splitter,
}

#[derive(Debug)]
struct Cell {
    cell_type: CellType,
    illuminated: bool,
    timeline_count: u64,
}

impl Cell {
    fn to_char(&self) -> char {
        match self.cell_type {
            CellType::Source => 'S',
            CellType::Space => if self.illuminated { '|' } else { '.' },
            CellType::Splitter => '^',
        }
    }
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        match cell_type {
            CellType::Source => Self {cell_type: cell_type, illuminated: true, timeline_count: 1},
            CellType::Space => Self {cell_type: cell_type, illuminated: false, timeline_count: 0},
            CellType::Splitter => Self {cell_type: cell_type, illuminated: false, timeline_count: 0},
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Cell::new(CellType::Source)),
            '.' => Ok(Cell::new(CellType::Space)),
            '^' => Ok(Cell::new(CellType::Splitter)),
            _ => Err(format!("Invalid cell character: {}", c)),
        }
    }
}