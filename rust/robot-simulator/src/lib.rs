// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot {
            x: self.x,
            y: self.y,
            d,
        }
    }

    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot {
            x: self.x,
            y: self.y,
            d,
        }
    }

    pub fn advance(self) -> Self {
        let mut robot = self.clone();
        match self.d {
            Direction::North => robot.y += 1,
            Direction::East => robot.x += 1,
            Direction::South => robot.y -= 1,
            Direction::West => robot.x -= 1,
        };
        robot
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self.clone();
        for c in instructions.chars() {
            match c {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                'A' => robot = robot.advance(),
                _ => continue,
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
