use std::ops::Range;
use crate::utils::direction::Direction;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y, }
    }

    pub fn move_in(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Point::new(self.x, self.y - 1),
            Direction::South => Point::new(self.x, self.y + 1),
            Direction::East => Point::new(self.x + 1, self.y),
            Direction::West => Point::new(self.x - 1, self.y),
        }
    }
}

pub fn range_of(range: Range<Point>) -> impl Iterator<Item = Point> {
    (range.start.y..=range.end.y)
        .flat_map(move |y|
            (range.start.x..=range.end.x)
                .map(move |x| Point::new(x, y))
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_range() {
        let start = Point::new(0, 0);
        let end = Point::new(2, 2);
        let range: Vec<_> = range_of(start..end).collect();

        assert_eq!(range, vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(2, 2),
        ]);
    }
}
