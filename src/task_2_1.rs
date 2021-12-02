use std::str::FromStr;
use crate::util::AoCError;

fn calculate_position(steps: &Vec<Step>) -> Position {
    steps.iter().fold(Position::default(), |last_pos, step| last_pos + step)
}

#[derive(Debug)]
struct Step {
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
struct Position {
    hor_pos: isize,
    depth: isize,
}

impl Position {
    fn score(&self) -> isize {
        self.hor_pos * self.depth
    }
}

impl std::ops::Add<&Step> for Position{
    type Output = Position;

    fn add(self, rhs: &Step) -> Self::Output {
        match rhs.direction {
            Direction::FORWARD => Position { hor_pos: self.hor_pos + rhs.distance, depth: self.depth},
            Direction::UP => Position { hor_pos: self.hor_pos, depth: self.depth - rhs.distance},
            Direction::DOWN => Position { hor_pos: self.hor_pos, depth: self.depth + rhs.distance},
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_file;
    use super::*;

    #[test]
    fn should_calculate_position() {
        // given
        let steps: Vec<Step> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter()
            .map(|s| s.parse::<Step>())
            .collect::<Result<Vec<Step>, AoCError>>()
            .unwrap();

        // when
        let result: Position = calculate_position(&steps);

        // then
        assert_eq!(result, Position { hor_pos: 15, depth: 10 });
        assert_eq!(result.score(), 150);
    }

    #[test]
    fn should_calculate_position_task() {
        // given
        let steps = read_file::<Step>("res/task_2.txt").unwrap();

        // when
        let result: Position = calculate_position(&steps);

        // then
        assert_eq!(result, Position { hor_pos: 1998, depth: 741 });
        assert_eq!(result.score(), 1480518);
    }
}