use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
struct Springs(Vec<Spring>);

impl Springs {
    fn unfold(&self, n: usize) -> Self {
        Springs(Self::repeat(&self.0, n))
    }

    fn repeat(original: &Vec<Spring>, n: usize) -> Vec<Spring> {
        let separator = vec![Spring::Unknown];

        original
            .iter()
            .cloned()
            .chain(separator.iter().cloned())
            .cycle()
            .take(original.len() * n + n - 1)
            .collect()
    }
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars()
            .map(|c| Spring::from(c))
            .collect();

        Ok(Springs(chars))
    }
}

struct Pattern(Vec<usize>);

impl Pattern {
    fn unfold(&self, n: usize) -> Self {
        let unfolded = self.0.iter()
            .cloned()
            .cycle()
            .take(self.0.len() * n)
            .collect();

        Pattern(unfolded)
    }
}

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
    fn unfold(&self, n: usize) -> Self {
        let springs = self.springs.unfold(n);
        let broken = self.broken.unfold(n);
        Record { springs, broken }
    }

    fn possible_solutions(&self) -> u64 {
        self.count_possible(&self.springs, &self.broken)
    }

    fn count_possible(&self, springs: &Springs, conditions: &Pattern) -> u64 {
        let states = self.build_states(conditions);

        let mut next_states = HashMap::new();
        let mut current_states = HashMap::new();
        current_states.insert(0, 1);

        for spring in &springs.0 {
            for (state_key, state) in current_states {
                let next_state = state_key + 1;
                match spring {
                    Spring::Unknown => {
                        if next_state < states.len() {
                            *next_states.entry(next_state).or_insert(0) += state;
                        }

                        if states[state_key] == Spring::Operational {
                            *next_states.entry(state_key).or_insert(0) += state;
                        }
                    }
                    Spring::Operational => {
                        if next_state < states.len() && states[next_state] == Spring::Operational {
                            *next_states.entry(next_state).or_insert(0) += state;
                        }

                        if states[state_key] == Spring::Operational {
                            *next_states.entry(state_key).or_insert(0) += state;
                        }
                    }
                    Spring::Damaged => {
                        if next_state < states.len() && states[next_state] == Spring::Damaged {
                            *next_states.entry(next_state).or_insert(0) += state;
                        }
                    }
                }
            }
            current_states = next_states;
            next_states = HashMap::new();
        }

        current_states.get(&(states.len() - 1)).unwrap_or(&0) + current_states.get(&(states.len() - 2)).unwrap_or(&0)
    }

    fn build_states(&self, conditions: &Pattern) -> Vec<Spring> {
        let mut states = Vec::new();
        for condition in &conditions.0 {
            states.push(Spring::Operational);
            for _ in 0..*condition as i32 {
                states.push(Spring::Damaged);
            }
        }
        states.push(Spring::Operational);

        states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_12/input.txt");

        let result: u64 = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.possible_solutions())
            .sum();

        assert_eq!(result, 7407);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_12/input.txt");

        let result = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.unfold(5))
            .map(|record| record.possible_solutions())
            .sum::<u64>();

        assert_eq!(result, 30568243604962);
    }

    #[test]
    fn example_folded() {
        let input = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
        ";

        let result: u64 = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.possible_solutions())
            .sum();

        assert_eq!(result, 21);
    }

    #[test]
    fn example_unfolded() {
        let input = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
        ";

        let solutions: Vec<u64> = input.lines()
            .map(|line| Record::from_str(line).unwrap())
            .map(|record| record.unfold(5))
            .map(|record| record.possible_solutions())
            .collect();

        let result: u64 = solutions.iter().sum();

        assert_eq!(result, 525152);
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

        assert_eq!(10, record.possible_solutions());
    }

    #[test]
    fn unfold_record() {
        let record = Record::from_str(".# 1").unwrap();

        let unfolded = record.unfold(5);

        assert_eq!(unfolded.springs.0, vec![
            Spring::Operational, Spring::Damaged, Spring::Unknown,
            Spring::Operational, Spring::Damaged, Spring::Unknown,
            Spring::Operational, Spring::Damaged, Spring::Unknown,
            Spring::Operational, Spring::Damaged, Spring::Unknown,
            Spring::Operational, Spring::Damaged,
        ]);

        assert_eq!(unfolded.broken.0, vec![
            1, 1, 1, 1, 1,
        ]);
    }
}
