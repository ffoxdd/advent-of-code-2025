use nalgebra::Vector3;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Playground {
    junction_boxes: Vec<JunctionBox>,
    circuit_collection: CircuitCollection,
}

impl Playground {
    fn new(junction_boxes: Vec<JunctionBox>) -> Self {
        let circuits = CircuitCollection::new(junction_boxes.len());
        Self {junction_boxes, circuit_collection: circuits}
    }

    pub fn circuits(&self) -> impl Iterator<Item = &HashSet<usize>> {
        self.circuit_collection.circuits()
    }

    pub fn closest_pairs(&self) -> Vec<(usize, usize)> {
        (0..self.junction_boxes.len())
            .tuple_combinations()
            .sorted_by(|pair1, pair2| self.compare_distances(*pair1, *pair2))
            .collect()
    }

    pub fn connect(&mut self, pair: (usize, usize)) {
        self.circuit_collection.merge(pair.0, pair.1);
    }

    pub fn x(&self, node: usize) -> i32 {
        self.junction_boxes[node].x()
    }

    fn compare_distances(&self, pair1: (usize, usize), pair2: (usize, usize)) -> Ordering {
        let distance1 = self.distance(pair1);
        let distance2 = self.distance(pair2);

        distance1.partial_cmp(&distance2).unwrap()
    }

    fn distance(&self, pair: (usize, usize)) -> f32 {
        let box1 = &self.junction_boxes[pair.0];
        let box2 = &self.junction_boxes[pair.1];

        box1.distance(box2)
    }
}

impl TryFrom<&Vec<String>> for Playground {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let junction_boxes: Result<Vec<JunctionBox>, String> = value.iter()
            .map(|line| JunctionBox::try_from(line))
            .collect();

        Ok(Self::new(junction_boxes?))
    }
}

#[derive(Debug)]
pub struct JunctionBox{
    position: Vector3<i32>,
}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl JunctionBox {
    pub fn x(&self) -> i32 {
        self.position.x
    }

    fn distance(&self, other: &JunctionBox) -> f32 {
        (self.position - other.position).cast::<f32>().norm()
    }
}

impl TryFrom<&String> for JunctionBox {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let coordinates: Vec<i32> = value
            .split(',')
            .map(|s| s.parse::<i32>().map_err(|e| format!("Parse error: {}", e)))
            .collect::<Result<Vec<i32>, String>>()?;

        let position = match coordinates.as_slice() {
            [x, y, z] => Vector3::new(*x, *y, *z),
            _ => return Err(format!("Expected 3 coordinates, got {}", coordinates.len())),
        };

        Ok(Self { position })
    }
}

#[derive(Debug)]
pub struct CircuitCollection {
    circuits: Vec<HashSet<usize>>,
    circuits_by_node: Vec<usize>,
}

impl CircuitCollection {
    fn new(node_count: usize) -> Self {
        Self {
            circuits: (0..node_count).map(|node| HashSet::from([node])).collect(),
            circuits_by_node: (0..node_count).collect(),
        }
    }

    pub fn circuits(&self) -> impl Iterator<Item = &HashSet<usize>> {
        self.circuits.iter().filter(|circuit| !circuit.is_empty())
    }

    pub fn connected(&self, node1: usize, node2: usize) -> bool {
        let circuit1_index = self.circuits_by_node[node1];
        let circuit1 = &self.circuits[circuit1_index];

        circuit1.contains(&node2)
    }

    pub fn merge(&mut self, node1: usize, node2: usize) {
        let circuit1_index = self.circuits_by_node[node1];
        let circuit2_index = self.circuits_by_node[node2];

        if circuit1_index == circuit2_index {
            return;
        }

        let circuit1 = self.circuits[circuit1_index].clone();

        self.circuits[circuit2_index].extend(circuit1.iter());
        self.circuits[circuit1_index].clear();

        for node in circuit1.iter() {
            self.circuits_by_node[*node] = circuit2_index;
        }
    }
}