use std::collections::HashSet;
use crate::day_03::robo_delivery::Direction::{East, North, South, West};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => North,
            'v' => South,
            '>' => East,
            '<' => West,
            _ => panic!("Invalid direction: {}", value),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y, }
    }

    fn move_in(&self, direction: &Direction) -> Self {
        match direction {
            North => Point::new(self.x, self.y - 1),
            South => Point::new(self.x, self.y + 1),
            East => Point::new(self.x + 1, self.y),
            West => Point::new(self.x - 1, self.y),
        }
    }
}

struct Santa {
    instructions: Vec<Direction>,
}

impl Santa {
    fn new(instructions: Vec<Direction>) -> Self {
        Santa { instructions }
    }

    fn delivery(&self) -> usize {
        let mut visited = HashSet::new();
        let mut current = Point::new(0, 0);
        visited.insert(current);

        self.instructions.iter()
            .for_each(|direction| {
                current = current.move_in(direction);
                visited.insert(current);
            });

        visited.len()
    }
}

fn parse_instructions(input: &str) -> Vec<Direction> {
    input.chars().map(|x| Direction::from(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_03/input.txt");

        let instructions = parse_instructions(input);
        let santa = Santa::new(instructions);

        assert_eq!(santa.delivery(), 2565);
    }

    #[test]
    fn direction_parsing() {
        let direction = Direction::from('^');

        assert_eq!(direction, North);
    }

    #[test]
    fn moving_points() {
        let mut point = Point::new(0, 0);

        point = point.move_in(&North);
        assert_eq!(point, Point::new(0, -1));

        point = point.move_in(&East);
        point = point.move_in(&East);
        assert_eq!(point, Point::new(2, -1));

        point = point.move_in(&South);
        point = point.move_in(&South);
        assert_eq!(point, Point::new(2, 1));

        point = point.move_in(&West);
        assert_eq!(point, Point::new(1, 1));
    }

}
