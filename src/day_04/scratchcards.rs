use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Scratchcard {
    winning: HashSet<u8>,
    numbers: Vec<u8>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split([':', '|']).collect();
        let winning: HashSet<u8> = parts[1].split(' ')
            .map(|number| number.trim())
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u8>().unwrap())
            .collect();
        let numbers: Vec<u8> = parts[2].split(' ')
            .map(|number| number.trim())
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u8>().unwrap())
            .collect();

        Ok(Scratchcard { winning, numbers })
    }
}

impl Scratchcard {
    fn score(&self) -> u16 {
        let wins = self.numbers.iter()
            .filter(|n| self.winning.contains(n))
            .count();

        if wins == 0 {
            0
        } else {
            (0..wins - 1).into_iter()
                .fold(1u16, |a, _| a * 2)
        }
    }
}

fn total_score(input: &str) -> u32 {
    input.lines()
        .map(|line| Scratchcard::from_str(line).unwrap())
        .map(|card| card.score())
        .map(|score| score as u32)
        .sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_04/input.txt");

        let total = total_score(input);

        assert_eq!(32001, total);
    }

    #[test]
    fn example() {
        let input = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
        ";
        let total_score = total_score(input);

        assert_eq!(13, total_score);
    }

    #[test]
    fn card_score() {
        let mut winning: HashSet<u8> = HashSet::new();
        winning.insert(41);
        winning.insert(48);
        winning.insert(83);
        winning.insert(86);
        winning.insert(17);
        let card = Scratchcard {
            winning,
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(8, card.score());
    }
}
