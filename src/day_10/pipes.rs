use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Point2D {
            x,
            y,
        }
    }

    fn next(&self, direction: &Direction) -> Point2D {
        match direction {
            Direction::Up => Point2D::new(self.x, self.y - 1),
            Direction::Down => Point2D::new(self.x, self.y + 1),
            Direction::Left => Point2D::new(self.x - 1, self.y),
            Direction::Right => Point2D::new(self.x + 1, self.y),
        }
    }

    fn is_next(&self, location: &Point2D, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.x == location.x && self.y - 1 == location.y,
            Direction::Down => self.x == location.x && self.y + 1 == location.y,
            Direction::Left => self.x - 1 == location.x && self.y == location.y,
            Direction::Right => self.x + 1 == location.x && self.y == location.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    TurnNorthEast,
    TurnNorthWest,
    TurnSouthEast,
    TurnSouthWest,
    Ground,
    Start,
}

impl Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::TurnNorthEast,
            'J' => Tile::TurnNorthWest,
            'F' => Tile::TurnSouthEast,
            '7' => Tile::TurnSouthWest,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unknown tile: {}!", value)
        }
    }

    fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    }

    fn is_ground(&self) -> bool {
        match self {
            Tile::Ground => true,
            _ => false,
        }
    }

    fn connects_to(&self, direction: &Direction) -> bool {
        match self {
            Tile::Vertical => {
                *direction == Direction::Up || *direction == Direction::Down
            }
            Tile::Horizontal => {
                *direction == Direction::Left || *direction == Direction::Right
            }
            Tile::TurnNorthEast => {
                *direction == Direction::Up || *direction == Direction::Right
            }
            Tile::TurnNorthWest => {
                *direction == Direction::Up || *direction == Direction::Left
            }
            Tile::TurnSouthEast => {
                *direction == Direction::Down || *direction == Direction::Right
            }
            Tile::TurnSouthWest => {
                *direction == Direction::Down || *direction == Direction::Left
            }
            _ => false
        }
    }

    fn leads_to(&self, from: &Direction) -> Option<Direction> {
        match self {
            Tile::Vertical => {
                match from {
                    Direction::Up => Some(Direction::Down),
                    Direction::Down => Some(Direction::Up),
                    _ => None
                }
            }
            Tile::Horizontal => {
                match from {
                    Direction::Left => Some(Direction::Right),
                    Direction::Right => Some(Direction::Left),
                    _ => None
                }
            }
            Tile::TurnNorthEast => {
                match from {
                    Direction::Up => Some(Direction::Right),
                    Direction::Right => Some(Direction::Up),
                    _ => None
                }
            }
            Tile::TurnNorthWest => {
                match from {
                    Direction::Up => Some(Direction::Left),
                    Direction::Left => Some(Direction::Up),
                    _ => None
                }
            }
            Tile::TurnSouthEast => {
                match from {
                    Direction::Down => Some(Direction::Right),
                    Direction::Right => Some(Direction::Down),
                    _ => None
                }
            }
            Tile::TurnSouthWest => {
                match from {
                    Direction::Down => Some(Direction::Left),
                    Direction::Left => Some(Direction::Down),
                    _ => None
                }
            }
            _ => None
        }
    }
}

struct PipeMap {
    start: Point2D,
    map: HashMap<Point2D, Tile>,
}

impl FromStr for PipeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut start: Option<Point2D> = None;
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let location = Point2D::new(x, y);
                let tile = Tile::from(char);
                if tile.is_start() {
                    start = Some(location.clone());
                }
                map.insert(location, tile);
            }
        }

        Ok(PipeMap { start: start.unwrap(), map })
    }
}

impl PipeMap {
    fn find_loop(&mut self) -> VecDeque<Point2D> {
        let mut path: VecDeque<Point2D> = VecDeque::new();
        let mut current = &self.start;

        let possible_directions = vec![
            Direction::Up, Direction::Down,
            Direction::Left, Direction::Right,
        ];

        let mut direction = possible_directions.iter()
            .flat_map(|dir| {
                let loc = current.next(dir);
                let tile = self.map.get(&loc).unwrap();
                if tile.connects_to(&dir.opposite()) {
                    Some(dir)
                } else { None }
            })
            .next().cloned().expect("Could not find next direction!");

        loop {
            let next = current.next(&direction);
            let next_tile = self.map.get(&next).unwrap();
            let next_direction = next_tile.leads_to(&direction.opposite());

            path.push_back(next);
            current = path.back().unwrap();

            if current == &self.start {
                break;
            }

            direction = next_direction.unwrap();
        }

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_10/input.txt");

        let mut pipes = PipeMap::from_str(input).unwrap();

        let pipe_loop = pipes.find_loop();

        assert_eq!(pipe_loop.len() / 2, 6886);
    }

    #[test]
    fn example() {
        let input = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....\n\
        ";

        let mut pipes = PipeMap::from_str(input).unwrap();

        let pipe_loop = pipes.find_loop();

        assert_eq!(pipes.start, Point2D::new(1, 1));
        assert_eq!(pipe_loop.len() / 2, 4);
    }

    #[test]
    fn tile_parsing() {
        assert_eq!(Tile::Vertical, Tile::from('|'));
        assert_eq!(Tile::Horizontal, Tile::from('-'));
        assert_eq!(Tile::TurnNorthEast, Tile::from('L'));
        assert_eq!(Tile::TurnNorthWest, Tile::from('J'));
        assert_eq!(Tile::TurnSouthEast, Tile::from('F'));
        assert_eq!(Tile::TurnSouthWest, Tile::from('7'));
        assert_eq!(Tile::Ground, Tile::from('.'));
        assert_eq!(Tile::Start, Tile::from('S'));
    }

    #[test]
    fn pipe_directions() {
        assert_eq!(Direction::Down, Tile::Vertical.leads_to(&Direction::Up).unwrap());
        assert_eq!(Direction::Up, Tile::Vertical.leads_to(&Direction::Down).unwrap());
        assert_eq!(Direction::Right, Tile::Horizontal.leads_to(&Direction::Left).unwrap());
        assert_eq!(Direction::Left, Tile::Horizontal.leads_to(&Direction::Right).unwrap());
        assert_eq!(Direction::Right, Tile::TurnNorthEast.leads_to(&Direction::Up).unwrap());
        assert_eq!(Direction::Up, Tile::TurnNorthEast.leads_to(&Direction::Right).unwrap());
        assert_eq!(Direction::Left, Tile::TurnNorthWest.leads_to(&Direction::Up).unwrap());
        assert_eq!(Direction::Up, Tile::TurnNorthWest.leads_to(&Direction::Left).unwrap());
        assert_eq!(Direction::Right, Tile::TurnSouthEast.leads_to(&Direction::Down).unwrap());
        assert_eq!(Direction::Down, Tile::TurnSouthEast.leads_to(&Direction::Right).unwrap());
        assert_eq!(Direction::Left, Tile::TurnSouthWest.leads_to(&Direction::Down).unwrap());
        assert_eq!(Direction::Down, Tile::TurnSouthWest.leads_to(&Direction::Left).unwrap());
    }

}
