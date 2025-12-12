use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactoryFloor {
    grid: Vec<Vec<Cell>>,
}

impl FactoryFloor {
    const MAX_NEIGHBORS: usize = 3;

    pub fn roll_count(&self) -> usize {
        self.rolls().count()
    }

    pub fn accessible_roll_count(&self) -> usize {
        self.accessible_rolls().len()
    }

    pub fn remove_accessible_rolls(&mut self) {
        loop {
            let accessible_rolls = self.accessible_rolls();

            if accessible_rolls.is_empty() {
                break;
            }

            self.clear_cells(&accessible_rolls);
        }
    }

    fn rolls(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells()
            .filter(|&(_, _, cell)| cell == Cell::PaperRoll)
            .map(|(i, j, _)| (i, j))
    }

    fn accessible_rolls(&self) -> Vec<(usize, usize)> {
        self.rolls()
            .filter(|&(i, j)| self.is_accessible(i, j))
            .collect()
    }

    fn clear_cells(&mut self, positions: &[(usize, usize)]) {
        for &(i, j) in positions {
            self.clear_cell(i, j);
        }
    }

    fn clear_cell(&mut self, i: usize, j: usize) {
        self.grid[i][j] = Cell::Empty;
    }

    fn cells(&self) -> impl Iterator<Item = (usize, usize, Cell)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, &cell)| (i, j, cell))
            })
    }

    pub fn is_accessible(&self, i: usize, j: usize) -> bool {
        self.neighbor_count(i, j) <= Self::MAX_NEIGHBORS
    }

    fn neighbor_count(&self, i: usize, j: usize) -> usize {
        self.neighbors(i, j)
            .filter(|&cell| cell != Cell::Empty)
            .count()
    }

    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = Cell> {
        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1),  (1, 0),  (1, 1)
        ];

        OFFSETS.iter().filter_map(move |&(di, dj)| {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            self.cell_at(ni, nj)
        })
    }

    fn cell_at(&self, i: i32, j: i32) -> Option<Cell> {
        if !self.in_bounds(i, j) {
            return None;
        }

        Some(self.grid[i as usize][j as usize])
    }

    fn in_bounds(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.height() && j >= 0 && j < self.width()
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }
}

impl TryFrom<&Vec<String>> for FactoryFloor {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<Cell>> = value
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| Cell::try_from(c))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if grid.is_empty() {
            return Err("Grid cannot be empty".to_string());
        }

        Ok(Self { grid })
    }
}

impl fmt::Display for FactoryFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_row = 0;
        for (i, j, cell) in self.cells() {
            if i != current_row {
                writeln!(f)?;
                current_row = i;
            }
            match cell {
                Cell::PaperRoll if self.is_accessible(i, j) => write!(f, "x")?,
                _ => write!(f, "{}", cell)?,
            }
        }
        writeln!(f)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    PaperRoll,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::PaperRoll),
            _ => Err(format!("Invalid cell character: '{}'", c)),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::PaperRoll => write!(f, "@"),
        }
    }
}
