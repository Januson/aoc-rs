use std::collections::VecDeque;
use std::str::FromStr;

struct Sequence {
    history: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = s.split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();

        Ok(Sequence {
            history,
        })
    }
}

impl Sequence {
    fn next(&self) -> i32 {
        let mut differences = self.build_differences();
        let mut difference = 0;
        for items in differences.iter_mut().rev() {
            let last = items.back().unwrap();
            difference = last + difference;
            items.push_back(difference);
        }

        *differences[0].back().unwrap()
    }

    fn prev(&self) -> i32 {
        let mut differences = self.build_differences();
        let mut difference = 0;

        for items in differences.iter_mut().rev() {
            let first = items.front().unwrap();
            difference = first - difference;
            items.push_front(difference);
        }

        differences[0][0]
    }

    fn build_differences(&self) -> Vec<VecDeque<i32>> {
        let mut current = VecDeque::from(self.history.clone());
        let mut stack = Vec::new();
        while !current.iter().all(|n| n == &0) {
            let next = current.iter().zip(current.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            stack.push(current);
            current = next;
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_09/input.txt").lines();

        let result: i32 = input
            .map(|line| Sequence::from_str(line).unwrap())
            .map(|seq| seq.next())
            .sum();

        assert_eq!(result, 1731106378);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_09/input.txt").lines();

        let result: i32 = input
            .map(|line| Sequence::from_str(line).unwrap())
            .map(|seq| seq.prev())
            .sum();

        assert_eq!(result, 1087);
    }

    #[test]
    fn sequence_parsing() {
        let sequence = Sequence::from_str("0 3 6 9 12 15");

        assert!(sequence.is_ok())
    }

    #[test]
    fn sequence_next() {
        let sequence = Sequence { history: vec![0, 3, 6, 9, 12, 15] };

        assert_eq!(18, sequence.next())
    }

    #[test]
    fn sequence_prev() {
        let sequence = Sequence { history: vec![0, 3, 6, 9, 12, 15] };

        assert_eq!(-3, sequence.prev())
    }
}
