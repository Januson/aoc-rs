use std::collections::HashMap;

struct Directions(String);

impl Directions {
    fn new(input: &str) -> Self {
        Directions(input.to_string())
    }

    fn final_floor(&self) -> i32 {
        let frequencies = frequencies(&self.0);
        let up = frequencies.get('(');
        let down = frequencies.get(')');

        (up as i32) - (down as i32)
    }

    fn basement(&self) -> i32 {
        let reductions = reductions(0, self.0.chars(), |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        });

        reductions.iter().position(|&x| x == -1).unwrap_or(0) as i32 + 1
    }
}

struct Frequencies {
    frequencies: HashMap<char, u32>,
}

impl Frequencies {
    fn get(&self, c: char) -> u32 {
        *self.frequencies.get(&c).unwrap_or(&0)
    }
}

fn frequencies(text: &str) -> Frequencies {
    let frequencies = text.chars().fold(HashMap::new(), |mut acc, c| {
        let counter = acc.entry(c).or_insert(0);
        *counter += 1;
        acc
    });

    Frequencies { frequencies }
}

fn reductions<T, R, F>(init: R, iter: impl IntoIterator<Item = T>, mut func: F) -> Vec<R>
where
    T: Clone,
    R: Clone,
    F: FnMut(R, T) -> R,
{
    let mut results = Vec::new();
    let mut current = init;

    for item in iter {
        current = func(current, item);
        results.push(current.clone());
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_01/input.txt");

        let directions = Directions::new(input);

        assert_eq!(directions.final_floor(), 280);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_01/input.txt");

        let directions = Directions::new(input);

        assert_eq!(directions.basement(), 1797);
    }

    #[test]
    fn starts_at_ground_floor() {
        let directions = Directions::new("");
        assert_eq!(directions.final_floor(), 0);
    }

    #[test]
    fn climbing_floors() {
        let directions = Directions::new("((((");
        assert_eq!(directions.final_floor(), 4);
    }

    #[test]
    fn descending_floors() {
        let directions = Directions::new(")))");
        assert_eq!(directions.final_floor(), -3);
    }

    #[test]
    fn combinations() {
        let directions = Directions::new("()()(()");
        assert_eq!(directions.final_floor(), 1);
    }

    #[test]
    fn test_frequencies() {
        let frequencies = frequencies("aabddd");

        assert_eq!(frequencies.get('a'), 2);
        assert_eq!(frequencies.get('b'), 1);
        assert_eq!(frequencies.get('c'), 0);
        assert_eq!(frequencies.get('d'), 3);
    }

    #[test]
    fn test_reductions() {
        let directions = reductions(0, vec![1, 1, 1, 1], |acc, x| acc + x);
        assert_eq!(directions, vec![1, 2, 3, 4]);
    }

    #[test]
    fn find_basement() {
        let directions = Directions::new(")");
        assert_eq!(directions.basement(), 1);

        let directions = Directions::new("()())");
        assert_eq!(directions.basement(), 5);
    }
}
