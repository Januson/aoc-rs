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
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
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
                let location = Point2D::new(x as i32, y as i32);
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
    fn find_loop(&mut self) -> PipeLoop {
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

        PipeLoop { segments: path.iter().cloned().collect() }
    }
}

struct PipeLoop {
    segments: Vec<Point2D>,
}

impl PipeLoop {
    fn len(&self) -> usize {
        self.segments.len()
    }

    fn area(&self) -> i32 {
        let vertices = find_vertices(&self.segments);

        let area = polygon_area(&vertices);

        picks_theorem(area, self.segments.len() as i32)
    }
}

fn find_vertices(coordinates: &Vec<Point2D>) -> Vec<Point2D> {
    let mut vertices = Vec::new();

    if coordinates.len() < 3 {
        // Not enough points to form a polygon
        return vertices;
    }

    // Iterate through consecutive triplets of points
    for i in 0..coordinates.len() {
        let p1 = &coordinates[i];
        let p2 = &coordinates[(i + 1) % coordinates.len()];
        let p3 = &coordinates[(i + 2) % coordinates.len()];

        // Check if the points form a convex angle (left turn)
        if is_convex_angle(p1, p2, p3) {
            vertices.push(p2.clone());
        }
    }

    vertices
}

// Helper function to check if three points form a convex angle (left turn)
fn is_convex_angle(p1: &Point2D, p2: &Point2D, p3: &Point2D) -> bool {
    let a = p1.x * (p2.y - p3.y) +
        p2.x * (p3.y - p1.y) +
        p3.x * (p1.y - p2.y);

    a != 0
}

/// Uses Pick's Theorem to calculate the number of interior points
fn picks_theorem(area: i32, boundary_points: i32) -> i32 {
    area - boundary_points / 2 + 1
}

/// Use shoelace formula to calculate area of a polygon from it's vertices.
fn polygon_area(coordinates: &[Point2D]) -> i32 {
    let n = coordinates.len();
    if n < 3 {
        return 0; // Not enough points to form a polygon
    }

    let mut sum = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += coordinates[i].x * coordinates[j].y - coordinates[j].x * coordinates[i].y;
    }

    sum.abs() / 2
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
    fn second_part() {
        let input = include_str!("../../input/day_10/input.txt");

        let mut pipes = PipeMap::from_str(input).unwrap();

        let pipe_loop = pipes.find_loop();

        assert_eq!(pipe_loop.area(), 371);
    }

    #[test]
    fn example_distant_point() {
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
    fn example_inside_loop_simple() {
        let input = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....\n\
        ";

        let mut pipes = PipeMap::from_str(input).unwrap();

        let pipe_loop = pipes.find_loop();

        assert_eq!(pipe_loop.area(), 1);
    }

    #[test]
    fn example_inside_loop() {
        let input = "\
        ...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........\n\
        ";

        let mut pipes = PipeMap::from_str(input).unwrap();

        let pipe_loop = pipes.find_loop();

        assert_eq!((pipe_loop.area()), 4);
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

    #[test]
    fn detect_angle() {
        let p1 = Point2D::new(1, 1);
        let p2 = Point2D::new(2, 2);
        let p3 = Point2D::new(3, 1);

        assert!(is_convex_angle(&p1, &p2, &p3));
    }

    #[test]
    fn detect_not_angle() {
        let p1 = Point2D::new(1, 3);
        let p2 = Point2D::new(2, 3);
        let p3 = Point2D::new(3, 3);

        assert!(!is_convex_angle(&p1, &p2, &p3));
    }
}
