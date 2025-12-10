const MAX_POSITION: u8 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Safe {
    dial_position: u8,
    zero_position_count: u16,
    zero_pass_count: u16,
}

impl Safe {
    pub fn new() -> Self {
        Self {
            dial_position: 50,
            zero_position_count: 0,
            zero_pass_count: 0,
         }
    }

    pub fn zero_position_count(&self) -> u16 {
        self.zero_position_count
    }

    pub fn zero_pass_count(&self) -> u16 {
        self.zero_pass_count
    }

    pub fn apply_instructions(&mut self, instructions: Vec<String>) -> Result<(), String> {
        let rotations = Rotation::parse_all(&instructions)?;
        self.apply_rotations(rotations.iter().copied());
        Ok(())
    }

    fn apply_rotations<I: IntoIterator<Item = Rotation>>(&mut self, rotations: I) {
        for rotation in rotations {
            self.rotate(rotation);
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.offset_dial(rotation.offset());
    }

    fn offset_dial(&mut self, offset: i16) {
        let old_position = self.dial_position as i16;
        let complete_rotations = offset / MAX_POSITION as i16;
        let remaining_steps = offset % MAX_POSITION as i16;
        let new_position = self.dial_position as i16 + remaining_steps;
        let wrapped_position = new_position.rem_euclid(MAX_POSITION as i16);

        self.zero_pass_count += complete_rotations.abs() as u16;

        if (old_position != 0 &&new_position <= 0) || new_position >= MAX_POSITION as i16 {
            self.zero_pass_count += 1;
        }

        if wrapped_position == 0 {
            self.zero_position_count += 1;
        }

        self.dial_position = wrapped_position as u8;
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    steps: u16,
}

impl Rotation {
    fn offset(&self) -> i16 {
        self.direction.offset() as i16 * self.steps as i16
    }

    fn parse_all(instructions: &[String]) -> Result<Vec<Rotation>, String> {
        instructions
            .iter()
            .map(|s| Rotation::try_from(s.as_str()))
            .collect()
    }

    fn parse_steps(steps_str: &str, full_value: &str) -> Result<u16, String> {
        steps_str.parse::<u16>()
            .map_err(|e| format!("'{}' - {}", full_value, e))
    }
}

impl TryFrom<&str> for Rotation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(format!("Empty string: '{}'", value));
        }

        let (direction_str, steps_str) = value.split_at(1);
        let direction = Direction::try_from(direction_str)?;
        let steps = Rotation::parse_steps(steps_str, value)?;

        Ok(Self{
            direction: direction,
            steps: steps,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> i8 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

