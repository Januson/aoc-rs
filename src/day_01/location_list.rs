use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
struct LocationId(i32);

impl LocationId {
    fn distance_to(&self, other: &LocationId) -> i32 {
        (self.0 - other.0).abs()
    }
}

impl FromStr for LocationId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LocationId(s.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LocationList {
    locations: Vec<LocationId>,
}

impl LocationList {
    fn distance_to(&mut self, other: &mut LocationList) -> i32 {
        self.locations.sort();
        other.locations.sort();

        self.locations
            .iter()
            .zip(other.locations.iter())
            .map(|(a, b)| a.distance_to(b))
            .sum()
    }

    fn similarity_of(&self, location: &LocationId) -> i32 {
        let occurrences = self.locations.iter()
            .filter(|id| id == &location)
            .count();
        location.0 * occurrences as i32
    }

    fn similarity(&self, other: &LocationList) -> i32 {
        self.locations.iter()
            .map(|id| other.similarity_of(id))
            .sum()
    }
}

fn parse_location_lists(input: &str) -> (LocationList, LocationList) {
    let all_locations:Vec<(LocationId, LocationId)> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            (LocationId::from_str(parts[0]).unwrap(), LocationId::from_str(parts[1]).unwrap())
        })
        .collect();

    let locations_1 = all_locations.iter().map(|(a, _)| *a).collect();
    let locations_2 = all_locations.iter().map(|(_, b)| *b).collect();

    (LocationList { locations: locations_1 }, LocationList { locations: locations_2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_01/input.txt");
        let (mut list_1, mut list_2) = parse_location_lists(input);

        let total_distance = list_1.distance_to(&mut list_2);

        assert_eq!(total_distance, 1_873_376);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/day_01/input.txt");
        let (list_1, mut list_2) = parse_location_lists(input);

        let total_distance = list_1.similarity(&mut list_2);

        assert_eq!(total_distance, 18_997_088);
    }

    #[test]
    fn location_distance() {
        let location_1 = LocationId(123);
        let location_2 = LocationId(234);

        let distance = location_1.distance_to(&location_2);

        assert_eq!(distance, 111);
    }

    #[test]
    fn total_list_distance() {
        let mut list_1 = LocationList {
            locations: vec![LocationId(123), LocationId(234)],
        };
        let mut list_2 = LocationList {
            locations: vec![LocationId(234), LocationId(345)],
        };

        let distance = list_1.distance_to(&mut list_2);

        assert_eq!(distance, 222);
    }

    #[test]
    fn list_parsing() {
        let input = "123 234\n234   345";
        let expected_list_1 = LocationList {
            locations: vec![LocationId(123), LocationId(234)],
        };
        let expected_list_2 = LocationList {
            locations: vec![LocationId(234), LocationId(345)],
        };

        let (list_1, list_2) = parse_location_lists(input);

        assert_eq!(expected_list_1, list_1);
        assert_eq!(expected_list_2, list_2);
    }

    #[test]
    fn single_id_similarity() {
        let location_list = LocationList {
            locations: vec![LocationId(1), LocationId(3), LocationId(3), LocationId(3)],
        };

        assert_eq!(0, location_list.similarity_of(&LocationId(7)));
        assert_eq!(1, location_list.similarity_of(&LocationId(1)));
        assert_eq!(9, location_list.similarity_of(&LocationId(3)));
    }

    #[test]
    fn list_similarity() {
        let location_list_1 = LocationList {
            locations: vec![
                LocationId(3),
                LocationId(4),
                LocationId(2),
                LocationId(1),
                LocationId(3),
                LocationId(3)
            ],
        };
        let location_list_2 = LocationList {
            locations: vec![
                LocationId(4),
                LocationId(3),
                LocationId(5),
                LocationId(3),
                LocationId(9),
                LocationId(3)
            ],
        };

        assert_eq!(31, location_list_1.similarity(&location_list_2));
    }
}
