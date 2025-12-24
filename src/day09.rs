use itertools::Itertools;
use nalgebra::Vector2;

pub struct Floor {
    red_tiles: Vec<Tile>,
}

#[derive(Clone, Copy)]
pub enum Filter {
    All,
    ValidOnly,
}

impl Floor {
    pub fn largest_rectangle_area(&self, filter: Filter) -> u64 {
        self.red_tiles
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| matches!(filter, Filter::All) || self.valid_rectangle(a, b))
            .map(|(a, b)| Tile::rectangle_area(a, b))
            .max()
            .unwrap()
    }

    fn valid_rectangle(&self, corner1: &Tile, corner2: &Tile) -> bool {
        self.rectangle_edges(corner1, corner2)
            .all(|edge| self.valid_edge(edge))
    }

    fn rectangle_edges(&self, corner1: &Tile, corner2: &Tile) -> impl Iterator<Item = AxisEdge> {
        geometry::rectangle_corners(corner1.position, corner2.position)
            .into_iter()
            .circular_tuple_windows::<(_, _)>()
            .map(|(a, b)| AxisEdge::new(a, b))
    }

    fn valid_edge(&self, edge: AxisEdge) -> bool {
        self.edges()
            .filter(|test_edge| edge.within_open_span(*test_edge))
            .all(|test_edge| !edge.left_intersection(test_edge))
    }

    fn edges(&self) -> impl Iterator<Item = AxisEdge> + '_ {
        self.red_tiles
            .iter()
            .map(|t| t.position)
            .circular_tuple_windows::<(_, _)>()
            .map(|(a, b)| AxisEdge::new(a, b))
    }
}

impl TryFrom<&Vec<String>> for Floor {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let tiles: Vec<Tile> = value
            .iter()
            .map(|line| Tile::try_from(line.as_str()))
            .collect::<Result<Vec<Tile>, _>>()?;

        Ok(Floor { red_tiles: tiles })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    position: Vector2<i64>,
}

impl Tile {
    fn new(x: i64, y: i64) -> Self {
        Tile {
            position: Vector2::new(x, y),
        }
    }

    fn rectangle_area(a: &Tile, b: &Tile) -> u64 {
        let difference = (a.position - b.position).abs();
        ((difference.x + 1) * (difference.y + 1)) as u64
    }
}

impl TryFrom<&str> for Tile {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coordinates: Vec<i64> = value
            .split(',')
            .map(|c| {
                c.parse::<i64>()
                    .map_err(|e| format!("Failed to parse '{}': {}", c, e))
            })
            .collect::<Result<_, _>>()?;

        if coordinates.len() != 2 {
            return Err(format!(
                "Expected 2 coordinates, found {}",
                coordinates.len()
            ));
        }

        Ok(Tile::new(coordinates[0], coordinates[1]))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AxisEdge {
    a: Vector2<i64>,
    b: Vector2<i64>,
}

impl AxisEdge {
    fn new(a: Vector2<i64>, b: Vector2<i64>) -> Self {
        debug_assert!(a.x == b.x || a.y == b.y, "AxisEdge must be axis-aligned");
        Self { a, b }
    }

    fn within_open_span(self, test_edge: AxisEdge) -> bool {
        let test_point = test_edge.a;

        let (min, max, test_coordinate) = if self.a.x == self.b.x {
            (self.a.y.min(self.b.y), self.a.y.max(self.b.y), test_point.y)
        } else {
            (self.a.x.min(self.b.x), self.a.x.max(self.b.x), test_point.x)
        };

        min < test_coordinate && test_coordinate < max
    }

    fn left_intersection(self, test_edge: AxisEdge) -> bool {
        if !self.within_open_span(test_edge) {
            return false;
        }

        let orientation0 = Self::orientation(self.a, self.b, test_edge.a);
        let orientation1 = Self::orientation(self.a, self.b, test_edge.b);

        orientation0 <= 0 && orientation1 > 0
    }

    fn orientation(a: Vector2<i64>, b: Vector2<i64>, p: Vector2<i64>) -> i64 {
        let ab = b - a;
        let ap = p - a;

        ab.perp(&ap)
    }
}

mod geometry {
    use super::Vector2;

    pub(super) fn rectangle_corners(opposite_corner1: Vector2<i64>, opposite_corner2: Vector2<i64>) -> [Vector2<i64>; 4] {
        let (min_corner, max_corner) = bounding_box(opposite_corner1, opposite_corner2);

        [
            Vector2::new(min_corner.x, min_corner.y),
            Vector2::new(max_corner.x, min_corner.y),
            Vector2::new(max_corner.x, max_corner.y),
            Vector2::new(min_corner.x, max_corner.y),
        ]
    }

    fn bounding_box(a: Vector2<i64>, b: Vector2<i64>) -> (Vector2<i64>, Vector2<i64>) {
        (a.inf(&b), a.sup(&b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_edge_basic() {
        let floor = Floor {
            red_tiles: vec![
                Tile::new(0, 0),
                Tile::new(10, 0),
                Tile::new(10, 10),
                Tile::new(0, 10),
            ],
        };

        let test_cases = vec![
            // Valid edges - corners
            ((0, 0), (10, 0), true),
            ((10, 0), (10, 10), true),
            ((10, 10), (0, 10), true),
            ((0, 10), (0, 0), true),

            // Valid edges - partial
            ((5, 0), (10, 0), true),
            ((10, 5), (10, 10), true),
            ((10, 10), (5, 10), true),
            ((5, 10), (5, 0), true),

            // Valid edges - interior
            ((2, 0), (8, 0), true),
            ((10, 2), (10, 8), true),
            ((8, 10), (2, 10), true),
            ((0, 8), (0, 2), true),

            // Invalid edges - extend beyond
            ((0, 0), (15, 0), false),
            ((10, 0), (10, 15), false),
            ((10, 10), (-5, 10), false),
            ((0, 10), (0, -5), false),
        ];

        for ((ax, ay), (bx, by), expected) in test_cases {
            let edge = AxisEdge::new(Vector2::new(ax, ay), Vector2::new(bx, by));

            assert_eq!(
                floor.valid_edge(edge),
                expected,
                "Edge from ({}, {}) to ({}, {}) should be {}",
                ax, ay, bx, by, if expected { "valid" } else { "invalid" }
            );
        }
    }
}
