struct Race {
    length: u64,
    record_distance: u64,
}

impl Race {
    fn new(length: u64, record_distance: u64) -> Self {
        Race { length, record_distance }
    }

    fn strategies(&self) -> Vec<u64> {
        let mut winning: Vec<u64> = Vec::new();
        for speed in 0..self.length {
            let remaining_time = self.length - speed;
            let travelled_distance = remaining_time * speed;
            if travelled_distance > self.record_distance {
                winning.push(speed);
            }
        }

        winning
    }
}

fn parse_row(input: Option<String>, label: &str) -> Vec<u64> {
    input.unwrap()
        .strip_prefix(label).unwrap()
        .split(' ').into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let mut input = include_str!("../../input/day_06/input.txt").lines();

        let lengths: Vec<u64> = parse_row(input.next().map(|x| x.to_string()), "Time:");
        let records: Vec<u64> = parse_row(input.next().map(|x| x.to_string()), "Distance:");

        let result = lengths.iter().zip(records.iter())
            .map(|(length, record)| Race::new(*length, *record))
            .map(|race| race.strategies().len())
            .fold(1, |a, b| a * b);

        assert_eq!(781200, result);
    }

    #[test]
    fn second_part() {
        let mut input = include_str!("../../input/day_06/input.txt").lines();

        let lengths: Vec<u64> = parse_row(input.next().map(|x| x.replace(' ', "")), "Time:");
        let records: Vec<u64> = parse_row(input.next().map(|x| x.replace(' ', "")), "Distance:");

        let result = lengths.iter().zip(records.iter())
            .map(|(length, record)| Race::new(*length, *record))
            .map(|race| race.strategies().len())
            .next().unwrap();

        assert_eq!(781200, result);
    }

    #[test]
    fn race_strategies() {
        let race = Race::new(7, 9);

        let strategies = race.strategies();

        assert_eq!(4, strategies.len());
        assert_eq!(2, strategies[0]);
        assert_eq!(3, strategies[1]);
        assert_eq!(4, strategies[2]);
        assert_eq!(5, strategies[3]);
    }
}
