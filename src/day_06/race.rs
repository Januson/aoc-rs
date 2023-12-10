use std::str::Lines;

struct Race {
    length: u32,
    record_distance: u32,
}

impl Race {
    fn new(length: u32, record_distance: u32) -> Self {
        Race { length, record_distance}
    }

    fn strategies(&self) -> Vec<u32> {
        let mut winning: Vec<u32> = Vec::new();
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

fn parse_row(input: &mut Lines, label: &str) -> Vec<u32> {
    input.next().unwrap()
        .strip_prefix(label).unwrap()
        .split(' ').into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let mut input = include_str!("../../input/day_06/input.txt").lines();

        let lengths: Vec<u32> = parse_row(&mut input, "Time:");
        let records: Vec<u32> = parse_row(&mut input, "Distance:");

        let result = lengths.iter().zip(records.iter())
            .map(|(length, record)| Race::new(*length, *record))
            .map(|race| race.strategies().len())
            .fold(1, |a, b| a * b);

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
