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

struct Galaxy {
    location: Point2D,
}

struct Observation {
    image: Vec<Vec<char>>,
}

impl Observation {
    fn galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies = Vec::new();
        let expanded_galaxy = self.expanded();
        for (y, row) in expanded_galaxy.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if char == &'#' {
                    galaxies.push(Galaxy { location: Point2D::new(x as i32, y as i32) });
                }
            }
        }

        galaxies
    }

    fn distance(&self) -> i32 {
        let galaxies = self.galaxies();

        let mut result: Vec<(&Galaxy, &Galaxy)> = Vec::new();
        for (i, galaxy_a) in galaxies.iter().enumerate() {
            for galaxy_b in &galaxies[i + 1..] {
                result.push((galaxy_a, galaxy_b));
            }
        }

        let distance = result.iter()
            .map(|(a, b)| a.location.distance(&b.location))
            .sum::<i32>();

        distance
    }

    fn expanded(&self) -> Vec<Vec<char>> {
        self.expand_columns(&self.expand_rows(&self.image))
    }

    fn expand_rows(&self, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut to_expand = Vec::new();
        for (i, row) in image.iter().enumerate() {
            if row.iter().all(|char| char == &'.') {
                to_expand.push(i);
            }
        }

        let mut result = image.clone();
        for i in to_expand.iter().rev() {
            result.insert(*i, result[*i].clone());
        }

        result
    }

    fn expand_columns(&self, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut to_expand = Vec::new();
        for i in 0..image[0].len() {
            if image.iter().all(|line| line[i] == '.') {
                to_expand.push(i);
            }
        }

        let mut result = image.clone();
        for i in to_expand.iter().rev() {
            result.iter_mut().for_each(|line| line.insert(*i, '.'));
        }

        result
    }
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

        let distance = observation.distance();

        assert_eq!(distance, 9312968);
    }

    #[test]
    fn example_distant_point() {
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
        let distance = observation.distance();

        assert_eq!(galaxies.len(), 9);
        assert_eq!(distance, 374);
    }

}
