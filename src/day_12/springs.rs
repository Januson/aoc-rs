use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from(char: char) -> Self {
        match char {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Unknown spring type: {}!", char)
        }
    }
}

struct Springs(Vec<Spring>);

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars()
            // Append . to avoid bound checking
            .chain(vec!['.'])
            .map(|c| Spring::from(c))
            .collect();

        Ok(Springs(chars))
    }
}

struct Pattern(Vec<usize>);

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        Ok(Pattern(numbers))
    }
}

struct Record {
    springs: Springs,
    broken: Pattern,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((springs, broken)) = s.split_once(' ') {
            let springs = Springs::from_str(springs).unwrap();
            let broken = Pattern::from_str(broken).unwrap();

            return Ok(Record {
                springs,
                broken,
            });
        }
        Err(())
    }
}

impl Record {
    fn possible_solutions(&self) -> i32 {
        self.count_possible(&self.springs, &self.broken)
    }

    fn count_possible(&self, springs: &Springs, conditions: &Pattern) -> i32 {
        let mut current_states = HashMap::new();
        current_states.insert((0, 0, 0), 1);

        let mut next_states = HashMap::new();

        for spring in &springs.0 {
            for (&state, &num) in &current_states {
                let (mut ci, mut cc, mut expdot) = state;
                match spring {
                    Spring::Damaged | Spring::Unknown if ci < conditions.0.len() && expdot == 0 => {
                        if spring == &Spring::Unknown && cc == 0 {
                            *next_states.entry((ci, cc, expdot)).or_insert(0) += num;
                        }
                        cc += 1;
                        if cc == conditions.0[ci] {
                            ci = ci + 1;
                            cc = 0;
                            expdot = 1;
                        }
                        let next_state = (ci, cc, expdot);
                        *next_states.entry(next_state).or_insert(0) += num;
                    }
                    Spring::Operational | Spring::Unknown if cc == 0 => {
                        let next_state = (ci, cc, 0);
                        *next_states.entry(next_state).or_insert(0) += num;
                    }
                    _ => {}
                }
            }
            current_states = next_states.clone();
            next_states = HashMap::new();
        }

        // sum states that reached the end of the pattern
        let mut possible = 0;
        for (&state, &v) in &current_states {
            if state.0 == conditions.0.len() {
                possible += v;
            }
        }

        possible
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_12/input.txt");

        let result: i32 = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.possible_solutions())
            .sum();

        assert_eq!(result, 8631); // Too high
    }

    #[test]
    fn example() {
        let input = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
        ";

        let result: i32 = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.possible_solutions())
            .sum();

        assert_eq!(result, 21);
    }

    #[test]
    fn no_wiggle_solution() {
        let record = Record::from_str("#.#.### 1,1,3").unwrap();

        let solutions = record.possible_solutions();

        assert_eq!(solutions, 1);
    }

    #[test]
    fn one_solutions() {
        let record = Record::from_str("???.### 1,1,3").unwrap();

        let solutions = record.possible_solutions();

        assert_eq!(1, solutions);
    }

    #[test]
    fn four_solutions() {
        let record = Record::from_str(".??..??...?##. 1,1,3").unwrap();

        let solutions = record.possible_solutions();

        assert_eq!(4, solutions);
    }

    #[test]
    fn ten_solutions() {
        let record = Record::from_str("?###???????? 3,2,1").unwrap();

        let solutions = record.possible_solutions();

        assert_eq!(10, solutions);
    }

    #[test]
    fn random_solutions() {
        let record = Record::from_str("?.?#??.?#? 4,2").unwrap();

        let solutions = record.possible_solutions();

        assert_eq!(2, solutions);
    }

}
