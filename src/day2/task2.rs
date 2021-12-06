use std::str::FromStr;
use crate::util::AoCError;

pub fn calculate_position(steps: &Vec<Step>) -> Position {
    steps.iter().fold(Position::default(), |last_pos, step| last_pos + step)
}

#[derive(Debug)]
pub struct Step {
    direction: Direction,
    distance: isize,
}

impl FromStr for Step {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let direction = split.next().ok_or(AoCError::new("Direction part is missing".to_owned()))?.parse::<Direction>()?;
        let distance = split.next().ok_or(AoCError::new("Distance part is missing".to_owned()))?.parse::<isize>().map_err(|e| AoCError::new(e.to_string()))?;
        Ok(Step { direction, distance })
    }
}

#[derive(Debug)]
enum Direction {
    FORWARD,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            "forward" => Ok(Direction::FORWARD),
            _ => Err(AoCError::new(format!("{} is not valid Direction", s)))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Position {
    hor_pos: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    pub fn score(&self) -> isize {
        self.hor_pos * self.depth
    }

    fn move_horizontally(&self, distance: isize) -> Position {
        Position {
            hor_pos: self.hor_pos + distance,
            depth: self.depth + self.aim * distance,
            aim: self.aim,
        }
    }

    fn move_vertically(&self, distance: isize) -> Position {
        Position {
            hor_pos: self.hor_pos,
            depth: self.depth,
            aim: self.aim + distance
        }
    }
}

impl std::ops::Add<&Step> for Position{
    type Output = Position;

    fn add(self, rhs: &Step) -> Self::Output {
        match rhs.direction {
            Direction::FORWARD => self.move_horizontally(rhs.distance),
            Direction::UP => self.move_vertically(-rhs.distance),
            Direction::DOWN => self.move_vertically(rhs.distance),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::parse_iter;

    use super::*;

    #[test]
    fn should_calculate_position() {
        // given
        let test_data = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
        let steps = parse_iter(&mut test_data.into_iter()).unwrap();

        // when
        let result: Position = calculate_position(&steps);

        // then
        assert_eq!(result, Position { hor_pos: 15, depth: 60, aim: 10 });
        assert_eq!(result.score(), 900);
    }
}
