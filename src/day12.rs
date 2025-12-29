use cp_sat::builder::{BoolVar, CpModelBuilder, LinearExpr};
use cp_sat::proto::CpSolverStatus;
use itertools::Itertools;

#[derive(Debug)]
pub struct TreeFarm {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
    shape_counts_by_region: Vec<Vec<usize>>,
}

impl TreeFarm {
    fn new(
        shapes: Vec<Shape>,
        regions: Vec<Region>,
        shape_counts_by_region: Vec<Vec<usize>>,
    ) -> Result<Self, String> {
        Self::validate_dimensions(&shapes, &regions, &shape_counts_by_region)?;

        Ok(Self {
            shapes,
            regions,
            shape_counts_by_region,
        })
    }

    pub fn valid_regions(&self) -> usize {
        self.problems()
            .filter(|problem| problem.has_solution())
            .count()
    }

    fn problems<'a>(&'a self) -> impl Iterator<Item = RegionProblem<'a>> {
        self.regions
            .iter()
            .zip(self.shape_counts_by_region.iter())
            .map(|(region, shape_counts)| {
                RegionProblem::new(&self.shapes, region, shape_counts)
            })
    }

    fn validate_dimensions(
        shapes: &[Shape],
        regions: &[Region],
        shape_counts_by_region: &[Vec<usize>],
    ) -> Result<(), String> {
        if regions.len() != shape_counts_by_region.len() {
            return Err(format!(
                "Mismatch: {} regions but {} shape-count rows",
                regions.len(),
                shape_counts_by_region.len()
            ));
        }

        let shape_count = shapes.len();

        for (i, counts) in shape_counts_by_region.iter().enumerate() {
            if counts.len() != shape_count {
                return Err(format!(
                    "Region {}: expected {} shape counts, got {}",
                    i,
                    shape_count,
                    counts.len()
                ));
            }
        }

        Ok(())
    }
}

impl TryFrom<&Vec<String>> for TreeFarm {
    type Error = String;

    fn try_from(lines: &Vec<String>) -> Result<Self, Self::Error> {
        let mut shapes = Vec::new();
        let mut regions = Vec::new();
        let mut shape_counts_by_region: Vec<Vec<usize>> = Vec::new();

        let mut index: usize = 0;

        while index < lines.len() {
            let line = lines[index].trim();

            // shape header, e.g. "0:"
            if line.ends_with(':') && line[..line.len() - 1].parse::<usize>().is_ok() {
                index += 1;
                let start = index;

                // collect grid lines
                while index < lines.len() {
                    let grid_line = lines[index].trim();

                    if grid_line.is_empty() {
                        break;
                    }

                    // stop if we hit a region line
                    if grid_line.contains('x') && grid_line.contains(':') {
                        break;
                    }

                    index += 1;
                }

                if index > start {
                    let grid_lines = &lines[start..index];
                    shapes.push(Shape::try_from(grid_lines)?);
                }

                // skip blank line if present
                if index < lines.len() && lines[index].trim().is_empty() {
                    index += 1;
                }

                continue;
            }

            // region line, e.g. "10x8: 2 0 4 ..."
            if line.contains('x') && line.contains(':') {
                let (_, counts_str) = line
                    .split_once(':')
                    .ok_or_else(|| format!("Invalid region line: {}", line))?;

                let shape_counts: Vec<usize> = counts_str
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().map_err(|e| format!("Invalid count: {}", e)))
                    .collect::<Result<_, String>>()?;

                let region = Region::try_from(line)?;
                regions.push(region);
                shape_counts_by_region.push(shape_counts);
                index += 1;
                continue;
            }

            index += 1;
        }

        Self::new(shapes, regions, shape_counts_by_region)
    }
}

struct RegionProblem<'a> {
    shapes: &'a [Shape],
    region: &'a Region,
    shape_counts: &'a [usize],
    placements_by_shape: Vec<Vec<Placement>>,
}

impl<'a> RegionProblem<'a> {
    fn sum(vars: &[BoolVar]) -> LinearExpr {
        vars.iter().fold(0.into(), |acc: LinearExpr, &v| acc + v)
    }

    fn new(
        shapes: &'a [Shape],
        region: &'a Region,
        shape_counts: &'a [usize],
    ) -> Self {
        let placements_by_shape = Self::placements_by_shape(shapes, region);
        Self { shapes, region, shape_counts, placements_by_shape }
    }

    pub fn has_solution(&self) -> bool {
        if !self.has_capacity() {
            println!("No capacity - skipping");
            return false;
        }

        println!("Solving");
        self.solve()
    }

    fn solve(&self) -> bool {
        let mut model = CpModelBuilder::default();
        let placement_vars_by_shape = self.placement_vars_by_shape(&mut model);

        self.add_shape_count_constraints(&mut model, &placement_vars_by_shape);
        self.add_overlap_constraints(&mut model, &placement_vars_by_shape);

        let response = model.solve();
        matches!(response.status(), CpSolverStatus::Optimal | CpSolverStatus::Feasible)
    }

    fn required_cell_count(&self) -> usize {
        self.shapes
            .iter()
            .zip(self.shape_counts.iter())
            .map(|(shape, &count)| shape.covered_cell_count() * count)
            .sum()
    }

    fn has_capacity(&self) -> bool {
        self.region.cell_count() >= self.required_cell_count()
    }

    fn add_shape_count_constraints(
        &self,
        model: &mut CpModelBuilder,
        placement_vars_by_shape: &[Vec<BoolVar>],
    ) {
        for (vars, &count) in placement_vars_by_shape.iter().zip(self.shape_counts.iter()) {
            let sum = Self::sum(vars);
            model.add_eq(sum, count as i64);
        }
    }

    fn add_overlap_constraints(
        &self,
        model: &mut CpModelBuilder,
        placement_vars_by_shape: &[Vec<BoolVar>],
    ) {
        let placement_vars_by_coordinate = self.placement_vars_by_coordinate(placement_vars_by_shape);

        for (x, y) in self.region.coordinates() {
            let placement_vars = &placement_vars_by_coordinate[x][y];

            if placement_vars.is_empty() {
                continue;
            }

            let coordinate_expression = Self::sum(placement_vars);

            model.add_le(coordinate_expression, 1);
        }
    }

    fn placement_vars_by_shape(&self, model: &mut CpModelBuilder) -> Vec<Vec<BoolVar>> {
        self.placements_by_shape
            .iter()
            .map(|placements| placements.iter().map(|_| model.new_bool_var()).collect())
            .collect()
    }

    fn placement_vars_by_coordinate(
        &self,
        placement_vars_by_shape: &[Vec<BoolVar>],
    ) -> Vec<Vec<Vec<BoolVar>>> {
        self.placements_by_shape
            .iter()
            .zip(placement_vars_by_shape.iter())
            .flat_map(|(placements, vars)| placements.iter().zip(vars.iter()))
            .fold(
                vec![vec![Vec::new(); self.region.height]; self.region.width],
                |mut result, (placement, &var)| {
                    for (x, y) in placement.covered_coordinates() {
                        result[x][y].push(var);
                    }

                    result
                },
            )
    }

    fn placements_by_shape(shapes: &[Shape], region: &Region) -> Vec<Vec<Placement>> {
        shapes
            .iter()
            .map(|shape| Self::placements(shape, region))
            .collect()
    }

    fn placements(shape: &Shape, region: &Region) -> Vec<Placement> {
        shape.orientations()
            .into_iter()
            .flat_map(|orientation| {
                Self::coordinate_placements(&orientation, region)
                    .into_iter()
                    .map(move |(x, y)| Placement::new(orientation.clone(), x, y))
            })
            .collect()
    }

    fn coordinate_placements(shape: &Shape, region: &Region) -> Vec<(usize, usize)> {
        if shape.width() > region.width || shape.height() > region.height {
            return Vec::new();
        }

        (0..=region.width - shape.width())
            .cartesian_product(0..=region.height - shape.height())
            .collect()
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
}

impl Region {
    fn cell_count(&self) -> usize {
        self.width * self.height
    }

    fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width).cartesian_product(0..self.height)
    }
}

impl TryFrom<&str> for Region {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (dimensions, _) = line
            .split_once(':')
            .ok_or_else(|| format!("Invalid region line: {}", line))?;

        let (width_str, height_str) = dimensions
            .split_once('x')
            .ok_or_else(|| format!("Invalid dimensions format: {}", dimensions))?;

        let width = width_str
            .trim()
            .parse::<usize>()
            .map_err(|e| format!("Invalid width: {}", e))?;

        let height = height_str
            .trim()
            .parse::<usize>()
            .map_err(|e| format!("Invalid height: {}", e))?;

        Ok(Self { width, height })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    grid: Vec<Vec<GridState>>,
    covered_coordinates: Vec<(usize, usize)>,
}

impl Shape {
    fn new(grid: Vec<Vec<GridState>>) -> Result<Self, String> {
        if grid.is_empty() {
            return Err("Grid cannot be empty".to_string());
        }

        if !Self::all_same_size(&grid) {
            return Err("All rows must have the same size".to_string());
        }

        if Self::degenerate(&grid) {
            return Err("Grid cannot have empty rows or columns".to_string());
        }

        let covered_coordinates = Self::compute_covered_coordinates(&grid);
        Ok(Self {
            grid,
            covered_coordinates,
        })
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn orientations(&self) -> Vec<Self> {
        // flips included
        vec![self.clone(), self.flip()]
            .into_iter()
            .flat_map(|shape| (0..4).map(move |i| shape.rotate(i)))
            .unique()
            .collect()
    }

    fn compute_covered_coordinates(grid: &[Vec<GridState>]) -> Vec<(usize, usize)> {
        grid.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &cell)| {
                    if cell != GridState::Present {
                        return None;
                    }
                    Some((x, y))
                })
            })
            .collect()
    }

    pub fn covered_coordinates(&self) -> &[(usize, usize)] {
        &self.covered_coordinates
    }

    fn covered_cell_count(&self) -> usize {
        self.covered_coordinates.len()
    }

    fn rotate(&self, count: u32) -> Self {
        (0..count).fold(self.clone(), |result, _| result.rotate_once())
    }

    fn flip(&self) -> Self {
        let flipped: Vec<Vec<GridState>> = self
            .grid
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();

        Self::new(flipped).unwrap()
    }

    fn rotate_once(&self) -> Self {
        let height = self.height();
        let width = self.width();

        let mut rotated = vec![vec![GridState::Empty; height]; width];

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                rotated[width - 1 - x][y] = cell;
            }
        }

        Self::new(rotated).unwrap()
    }

    fn all_same_size<T>(items: &[Vec<T>]) -> bool {
        if items.is_empty() {
            return true;
        }
        let first_size = items[0].len();
        items.iter().all(|item| item.len() == first_size)
    }

    fn degenerate(grid: &[Vec<GridState>]) -> bool {
        Self::has_empty_rows(grid) || Self::has_empty_columns(grid)
    }

    fn has_empty_rows(grid: &[Vec<GridState>]) -> bool {
        grid.iter()
            .any(|row| row.iter().all(|&cell| cell == GridState::Empty))
    }

    fn has_empty_columns(grid: &[Vec<GridState>]) -> bool {
        (0..grid[0].len()).any(|col| grid.iter().all(|row| row[col] == GridState::Empty))
    }
}

impl TryFrom<&[String]> for Shape {
    type Error = String;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<GridState>> = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.trim()
                    .chars()
                    .map(GridState::try_from)
                    .collect::<Result<_, String>>()
            })
            .collect::<Result<_, String>>()?;

        Self::new(grid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridState {
    Empty,
    Present,
}

impl TryFrom<char> for GridState {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(GridState::Present),
            '.' => Ok(GridState::Empty),
            _ => Err(format!("Invalid character in grid: {}", c)),
        }
    }
}

struct Placement {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Placement {
    fn new(shape: Shape, x: usize, y: usize) -> Self {
        Self { shape, x, y }
    }

    fn covered_coordinates(&self) -> Vec<(usize, usize)> {
        self.shape
            .covered_coordinates()
            .iter()
            .map(move |&(x, y)| (x + self.x, y + self.y))
            .collect()
    }
}
