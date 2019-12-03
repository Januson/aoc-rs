use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Intersection {
    point: Point,
    steps: u32,
}

struct Wire {
    path: HashMap<Point, usize>,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    U(u32),
    D(u32),
    R(u32),
    L(u32),
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }

    fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl Wire {
    fn new(path: &str) -> Wire {
        Wire {
            path: path.split(',')
                .map(|x| Direction::of(x))
                .fold((HashMap::new(), Point::new(0, 0), 0), |(mut path, mut current, mut index), direction| {
                    match direction {
                        Direction::U(steps) => {
                            for _ in 0..steps {
                                current = current.up();
                                index += 1;
                                path.entry(current).or_insert(index);
                            };
                        }
                        Direction::D(steps) => {
                            for _ in 0..steps {
                                current = current.down();
                                index += 1;
                                path.entry(current).or_insert(index);
                            };
                        }
                        Direction::R(steps) => {
                            for _ in 0..steps {
                                current = current.right();
                                index += 1;
                                path.entry(current).or_insert(index);
                            };
                        }
                        Direction::L(steps) => {
                            for _ in 0..steps {
                                current = current.left();
                                index += 1;
                                path.entry(current).or_insert(index);
                            };
                        }
                    };
                    (path, current, index)
                }).0,
        }
    }

    fn len(&self) -> usize {
        self.path.len()
    }

    fn intersections_with(&self, other: &Wire) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for (point, steps) in &self.path {
            match other.path.get(&point) {
                Some(other_steps) => {
                    intersections.push(Intersection::new(*point, steps + other_steps))
                },
                None => continue,
            }
        };
        intersections
    }
}

impl Intersection {
    fn new(point: Point, steps: usize) -> Intersection {
        Intersection {
            point,
            steps: steps as u32,
        }
    }

    fn distance(&self) -> u32 {
        self.point.distance()
    }

    fn steps(&self) -> u32 {
        self.steps
    }
}

impl Direction {
    fn of(direction: &str) -> Direction {
        match direction.chars().nth(0) {
            Some('U') => Direction::U(direction[1..].parse().expect("Expected positive integer!")),
            Some('D') => Direction::D(direction[1..].parse().expect("Expected positive integer!")),
            Some('R') => Direction::R(direction[1..].parse().expect("Expected positive integer!")),
            Some('L') => Direction::L(direction[1..].parse().expect("Expected positive integer!")),
            _ => panic!("aaaaaaaaaaaaaaaaaa"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_right_direction() {
        let direction = Direction::of("R12");

        assert_eq!(direction, Direction::R(12));
    }


    #[test]
    fn should_build_left_direction() {
        let direction = Direction::of("L112");

        assert_eq!(direction, Direction::L(112));
    }


    #[test]
    fn should_build_up_direction() {
        let direction = Direction::of("U122");

        assert_eq!(direction, Direction::U(122));
    }

    #[test]
    fn should_build_down_direction() {
        let direction = Direction::of("D11");

        assert_eq!(direction, Direction::D(11));
    }

    #[test]
    fn should_build_wire() {
        let wire = Wire::new("D2,R3,U4,L2");

        assert_eq!(wire.len(), 11);
    }

    #[test]
    fn should_find_intersections() {
        let wire = Wire::new("D2,R3,U4,L2");
        let wire2 = Wire::new("U1,R5");

        assert_eq!(
            wire.intersections_with(&wire2),
            vec![Intersection::new(Point::new(3, 1), 12)]);
    }

    #[test]
    fn should_find_intersections2() {
        let wire = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");

        let mut intersections = wire.intersections_with(&wire2);
        intersections.sort_by(|a, b| a.steps().cmp(&b.steps()));

        assert_eq!(
            intersections.iter().next().unwrap().steps(),
            610);
    }

    #[test]
    fn should_find_intersections3() {
        let wire = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        let mut intersections = wire.intersections_with(&wire2);
        intersections.sort_by(|a, b| a.steps().cmp(&b.steps()));

        assert_eq!(
            intersections.iter().next().unwrap().steps(),
            410);
    }

    #[test]
    fn first_part() {
        let mut input = include_str!("../../inputs/year_2019/day_03/input.txt")
            .lines();
        let wire = Wire::new(input.next().unwrap());
        let wire2 = Wire::new(input.next().unwrap());

        let mut intersections = wire.intersections_with(&wire2);
        intersections.sort_by(|a, b| a.distance().cmp(&b.distance()));

        assert_eq!(intersections.iter().next().unwrap().distance(), 227);
    }

    #[test]
    fn second_part() {
        let mut input = include_str!("../../inputs/year_2019/day_03/input.txt")
            .lines();
        let wire = Wire::new(input.next().unwrap());
        let wire2 = Wire::new(input.next().unwrap());

        let mut intersections = wire.intersections_with(&wire2);
        intersections.sort_by(|a, b| a.steps().cmp(&b.steps()));

        assert_eq!(intersections.iter().next().unwrap().steps(), 20286);
    }
}