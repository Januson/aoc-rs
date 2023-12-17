use std::str::FromStr;

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

    fn distance(&self, other: &Point2D) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Galaxy {
    location: Point2D,
}

struct Observation {
    image: Vec<Vec<char>>,
}

impl Observation {
    fn galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies = Vec::new();
        for (y, row) in self.image.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if char == &'#' {
                    galaxies.push(Galaxy { location: Point2D::new(x as i32, y as i32) });
                }
            }
        }

        galaxies
    }

    fn distance(&self, expansion: i32) -> u64 {
        let galaxies = self.galaxies();

        let result = to_pairs(&galaxies);

        let distance = result.iter()
            .map(|(a, b)| {
                let empty_rows = self.empty_rows(&a.location, &b.location);
                let empty_columns = self.empty_columns(&a.location, &b.location);
                let expanded_space = (empty_columns + empty_rows) * expansion;
                let distance = a.location.distance(&b.location);
                distance + expanded_space - empty_rows - empty_columns
            })
            .map(|n| n as u64)
            .sum::<u64>();

        distance
    }

    fn empty_rows(&self, a: &Point2D, b: &Point2D) -> i32 {
        let mut empty = 0;
        let start = a.y.min(b.y) as usize;
        let end = a.y.max(b.y) as usize;
        for row in &self.image[start..end] {
            if row.iter().all(|char| char == &'.') {
                empty += 1;
            }
        }
        empty
    }

    fn empty_columns(&self, a: &Point2D, b: &Point2D) -> i32 {
        let mut empty = 0;
        let start = a.x.min(b.x);
        let end = a.x.max(b.x);
        for i in start..end {
            if (&self.image).iter().all(|line| line[i as usize] == '.') {
                empty += 1;
            }
        }

        empty
    }
}

fn to_pairs(galaxies: &Vec<Galaxy>) -> Vec<(&Galaxy, &Galaxy)> {
    let mut result: Vec<(&Galaxy, &Galaxy)> = Vec::new();
    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in &galaxies[i + 1..] {
            result.push((galaxy_a, galaxy_b));
        }
    }
    result
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let image = s.lines()
            .map(|line| line.chars().collect())
            .collect();

        Ok(Observation { image })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_11/input.txt");

        let observation = Observation::from_str(input).unwrap();

        let distance = observation.distance(2);

        assert_eq!(distance, 9312968);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_11/input.txt");

        let observation = Observation::from_str(input).unwrap();

        let distance = observation.distance(1_000_000);

        assert_eq!(distance, 597714117556);
    }

    #[test]
    fn example_expand() {
        let input = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
        ";

        let observation = Observation::from_str(input).unwrap();

        let galaxies = observation.galaxies();
        let distance = observation.distance(2);

        assert_eq!(galaxies.len(), 9);
        assert_eq!(distance, 374);
    }

    #[test]
    fn find_empty_rows() {
        let input = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
        ";

        let observation = Observation::from_str(input).unwrap();

        let galaxy_1 = Point2D::new(3, 0);
        let galaxy_2 = Point2D::new(7, 1);
        let galaxy_3 = Point2D::new(0, 2);
        let galaxy_4 = Point2D::new(6, 4);
        let galaxy_5 = Point2D::new(1, 5);
        let galaxy_6 = Point2D::new(9, 6);
        let galaxy_7 = Point2D::new(7, 8);
        let galaxy_8 = Point2D::new(0, 9);
        let galaxy_9 = Point2D::new(4, 9);

        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_2), 0);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_3), 0);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_4), 1);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_5), 1);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_6), 1);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_7), 2);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_8), 2);
        assert_eq!(observation.empty_rows(&galaxy_1, &galaxy_9), 2);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_3), 0);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_4), 1);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_5), 1);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_6), 1);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_7), 2);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_8), 2);
        assert_eq!(observation.empty_rows(&galaxy_2, &galaxy_9), 2);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_4), 1);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_5), 1);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_6), 1);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_7), 2);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_8), 2);
        assert_eq!(observation.empty_rows(&galaxy_3, &galaxy_9), 2);
        assert_eq!(observation.empty_rows(&galaxy_4, &galaxy_5), 0);
        assert_eq!(observation.empty_rows(&galaxy_4, &galaxy_6), 0);
        assert_eq!(observation.empty_rows(&galaxy_4, &galaxy_7), 1);
        assert_eq!(observation.empty_rows(&galaxy_4, &galaxy_8), 1);
        assert_eq!(observation.empty_rows(&galaxy_4, &galaxy_9), 1);
        assert_eq!(observation.empty_rows(&galaxy_5, &galaxy_6), 0);
        assert_eq!(observation.empty_rows(&galaxy_5, &galaxy_7), 1);
        assert_eq!(observation.empty_rows(&galaxy_5, &galaxy_8), 1);
        assert_eq!(observation.empty_rows(&galaxy_5, &galaxy_9), 1);
        assert_eq!(observation.empty_rows(&galaxy_6, &galaxy_7), 1);
        assert_eq!(observation.empty_rows(&galaxy_6, &galaxy_8), 1);
        assert_eq!(observation.empty_rows(&galaxy_6, &galaxy_9), 1);
        assert_eq!(observation.empty_rows(&galaxy_7, &galaxy_8), 0);
        assert_eq!(observation.empty_rows(&galaxy_7, &galaxy_9), 0);
        assert_eq!(observation.empty_rows(&galaxy_8, &galaxy_9), 0);
    }

    #[test]
    fn find_empty_columns() {
        let input = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
        ";

        let observation = Observation::from_str(input).unwrap();

        let galaxy_1 = Point2D::new(3, 0);
        let galaxy_2 = Point2D::new(7, 1);
        let galaxy_3 = Point2D::new(0, 2);
        let galaxy_4 = Point2D::new(6, 4);
        let galaxy_5 = Point2D::new(1, 5);
        let galaxy_6 = Point2D::new(9, 6);
        let galaxy_7 = Point2D::new(7, 8);
        let galaxy_8 = Point2D::new(0, 9);
        let galaxy_9 = Point2D::new(4, 9);

        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_2), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_3), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_4), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_5), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_6), 2);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_7), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_8), 1);
        assert_eq!(observation.empty_columns(&galaxy_1, &galaxy_9), 0);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_3), 2);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_4), 0);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_5), 2);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_6), 1);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_7), 0);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_8), 2);
        assert_eq!(observation.empty_columns(&galaxy_2, &galaxy_9), 1);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_4), 2);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_5), 0);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_6), 3);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_7), 2);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_8), 0);
        assert_eq!(observation.empty_columns(&galaxy_3, &galaxy_9), 1);
        assert_eq!(observation.empty_columns(&galaxy_4, &galaxy_5), 2);
        assert_eq!(observation.empty_columns(&galaxy_4, &galaxy_6), 1);
        assert_eq!(observation.empty_columns(&galaxy_4, &galaxy_7), 0);
        assert_eq!(observation.empty_columns(&galaxy_4, &galaxy_8), 2);
        assert_eq!(observation.empty_columns(&galaxy_4, &galaxy_9), 1);
        assert_eq!(observation.empty_columns(&galaxy_5, &galaxy_6), 3);
        assert_eq!(observation.empty_columns(&galaxy_5, &galaxy_7), 2);
        assert_eq!(observation.empty_columns(&galaxy_5, &galaxy_8), 0);
        assert_eq!(observation.empty_columns(&galaxy_5, &galaxy_9), 1);
        assert_eq!(observation.empty_columns(&galaxy_6, &galaxy_7), 1);
        assert_eq!(observation.empty_columns(&galaxy_6, &galaxy_8), 3);
        assert_eq!(observation.empty_columns(&galaxy_6, &galaxy_9), 2);
        assert_eq!(observation.empty_columns(&galaxy_7, &galaxy_8), 2);
        assert_eq!(observation.empty_columns(&galaxy_7, &galaxy_9), 1);
        assert_eq!(observation.empty_columns(&galaxy_8, &galaxy_9), 1);
    }
}
