use std::collections::HashMap;

struct Directions(String);

impl Directions {
    fn new(input: &str) -> Self {
        Directions(input.to_string())
    }

    fn final_floor(&self) -> i32 {
        let frequencies = frequencies(&self.0);
        let up = frequencies.frequencies.get(&'(').unwrap_or(&0);
        let down = frequencies.frequencies.get(&')').unwrap_or(&0);

        (*up as i32) - (*down as i32)
    }
}

struct Frequencies {
    frequencies: HashMap<char, u32>,
}

fn frequencies(text: &str) -> Frequencies {
    let frequencies = text.chars().fold(HashMap::new(), |mut acc, c| {
        let counter = acc.entry(c).or_insert(0);
        *counter += 1;
        acc
    });

    Frequencies { frequencies }
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
}
