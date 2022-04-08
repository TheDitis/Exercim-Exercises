use strum::IntoEnumIterator;
use strum_macros::EnumIter;

///-------------------------------------------------------------------------------------------------
/// TURN
///-------------------------------------------------------------------------------------------------
pub enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Turn::Left),
            'R' => Ok(Turn::Right),
            _ => Err("invalid turn char!".to_string())
        }
    }
}

///-------------------------------------------------------------------------------------------------
/// DIRECTION
///-------------------------------------------------------------------------------------------------
#[derive(PartialEq, Debug, Clone, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
}


///-------------------------------------------------------------------------------------------------
/// ROBOT
///-------------------------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Robot {
    location: [i32; 2],
    facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { location: [x, y], facing: d }
    }

    #[must_use]
    pub fn turn_right(&mut self) -> Self {
        self.turn(Turn::Right)
    }

    #[must_use]
    pub fn turn_left(&mut self) -> Self {
        self.turn(Turn::Left)
    }

    pub fn turn(&mut self, turn_direction: Turn) -> Self {
        let all_directions: Vec<Direction> = match turn_direction {
            Turn::Left => Direction::iter().rev().collect(),
            Turn::Right => Direction::iter().collect(),
        };
        let i = all_directions.iter().position(|d| { d == &self.facing }).unwrap();
        self.facing = all_directions[(i + 1) % all_directions.len()].clone();
        self.clone()
    }

    #[must_use]
    pub fn advance(&mut self) -> Self {
        let (axis, movement) = match self.facing {
            Direction::North => (1, 1),
            Direction::East => (0, 1),
            Direction::South => (1, -1),
            Direction::West => (0, -1),
        };
        self.location[axis] += movement;
        self.clone()
    }

    #[must_use]
    pub fn instructions(&mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'L' | 'R' => { self.turn(Turn::try_from(c).unwrap()); },
                'A' => { let _ = self.advance(); },
                _ => (),
            }
        }
        self.clone()
    }

    pub fn position(&self) -> (i32, i32) { (self.location[0], self.location[1]) }

    pub fn direction(&self) -> &Direction { &self.facing }
}
