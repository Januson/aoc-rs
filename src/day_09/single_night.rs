use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Place(String);

struct Locations {
    locations: HashMap<Place, HashMap<Place, u32>>
}

impl Locations {
    fn connect(&mut self, from: Place, to: Place, distance: u32) {
        self.locations.entry(from.clone()).or_insert(HashMap::new()).insert(to.clone(), distance);
        self.locations.entry(to).or_insert(HashMap::new()).insert(from, distance);
    }
}

impl FromStr for Locations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locations = Locations {
            locations: HashMap::new()
        };
        s.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let from = Place(parts[0].to_string());
            let to = Place(parts[2].to_string());
            let distance = parts[4].parse().unwrap();

            locations.connect(from, to, distance);
        });

        Ok(locations)
    }
}

fn permutations<T>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + Ord,
{
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<T>> = vec![];
        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = items.clone();
            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

impl Locations {
    fn shortest_path(&self) -> u32 {
        let cities: Vec<_> = self.locations.keys().cloned().collect();
        let permutations = permutations(cities);
        let mut shortest = u32::MAX;
        for route in permutations {
            let distance = route.windows(2).map(|pair| {
                self.locations[&pair[0]][&pair[1]]
            }).sum();
            if distance < shortest {
                shortest = distance;
            }
        }

        shortest
    }

    fn longest_path(&self) -> u32 {
        let cities: Vec<_> = self.locations.keys().cloned().collect();
        let permutations = permutations(cities);
        let mut longest = 0;
        for route in permutations {
            let distance = route.windows(2).map(|pair| {
                self.locations[&pair[0]][&pair[1]]
            }).sum();
            if distance > longest {
                longest = distance;
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        London to Dublin = 464\n\
        London to Belfast = 518\n\
        Dublin to Belfast = 141\n\
        ";

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_09/input.txt");

        let locations: Locations = Locations::from_str(input).unwrap();

        assert_eq!(locations.shortest_path(), 117);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_09/input.txt");

        let locations: Locations = Locations::from_str(input).unwrap();

        assert_eq!(locations.longest_path(), 909);
    }

    #[test]
    fn parsing() {
        let input = EXAMPLE;

        let locations: Locations = Locations::from_str(input).unwrap();

        assert_eq!(locations.locations.len(), 3);
    }

    #[test]
    fn test_shortest_path() {
        let locations = Locations::from_str(EXAMPLE).unwrap();

        assert_eq!(locations.shortest_path(), 605);
    }

    #[test]
    fn test_longest_path() {
        let locations = Locations::from_str(EXAMPLE).unwrap();

        assert_eq!(locations.longest_path(), 982);
    }

    #[test]
    fn test_permutations() {
        let items = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        let result = permutations(items);

        assert_eq!(result, expected);
    }
}
