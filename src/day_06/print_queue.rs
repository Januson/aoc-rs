use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Map {
    guard: Point,
    tiles: HashMap<Point, Tile>,
}

impl Map {
    fn guard_route(&self) -> HashSet<Point> {
        let mut route = HashSet::new();
        let mut current = self.guard;
        let mut direction = match &self.tiles[&current] {
            Tile::Guard(direction) => *direction,
            _ => panic!("Could not find direction for guard"),
        };

        loop {
            let next_point = current.move_in(&direction);
            if let Some(next) = self.tiles.get(&next_point) {
                match next {
                    Tile::Obstacle => direction = direction.turn(),
                    _ => {
                        route.insert(current);
                        current = next_point;
                    }
                }
            } else {
                route.insert(current);
                break;
            }
        }

        route
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashMap<Point, Tile> = HashMap::new();
        let mut guard: Option<Point> = None;

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let point = Point::new(x as i32, y as i32);
                let tile: Tile = c.into();
                if tile.is_guard() {
                    guard = Some(point);
                }
                tiles.insert(point, tile);
            });
        });

        Ok(Map {
            guard: guard.unwrap(),
            tiles,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
    Guard(Direction),
}

impl Tile {
    fn is_guard(&self) -> bool {
        match self {
            Tile::Guard(_) => true,
            _ => false,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard(Direction::Up),
            '>' => Tile::Guard(Direction::Right),
            '<' => Tile::Guard(Direction::Left),
            'v' => Tile::Guard(Direction::Down),
            _ => panic!("Unknown tile type"),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn move_in(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_06/input.txt");

        let map = Map::from_str(input).unwrap();

        let route = map.guard_route();
        assert_eq!(route.len(), 4663);
    }

    #[test]
    fn map_parsing() {
        let map = Map::from_str(".#.\n.v.\n..#").unwrap();

        assert_eq!(map.guard, Point::new(1, 1));
        assert_eq!(map.tiles[&Point::new(0, 0)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(1, 0)], Tile::Obstacle);
        assert_eq!(map.tiles[&Point::new(2, 0)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(0, 1)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(1, 1)], Tile::Guard(Direction::Down));
        assert_eq!(map.tiles[&Point::new(2, 1)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(0, 2)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(1, 2)], Tile::Empty);
        assert_eq!(map.tiles[&Point::new(2, 2)], Tile::Obstacle);
    }

    #[test]
    fn test_guard_route() {
        let input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\n\
        ";
        let map = Map::from_str(input).unwrap();

        let route = map.guard_route();
        assert_eq!(route.len(), 41);
    }
}
