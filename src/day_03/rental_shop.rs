use std::str::FromStr;

#[derive(Debug)]
struct CorruptedMemory(String);

impl CorruptedMemory {
    fn product(&self) -> i64 {
        let res: i64 = self.uncorrupted(false).iter()
            .map(|x| x.product())
            .sum();
        res
    }

    fn product_with_conditions(&self) -> i64 {
        let res: i64 = self.uncorrupted(true).iter()
            .map(|x| x.product())
            .sum();
        res
    }

    fn uncorrupted(&self, conditions: bool) -> Vec<Multiplication> {
        let mut enabled = true;
        find_patterns(&self.0).iter()
            .filter_map(|pattern| {
                match pattern.as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    &_ => {
                        if conditions && !enabled {
                            return None;
                        }
                        return Multiplication::from_str(pattern).ok()
                    },
                }
                return None;
            })
            .collect()

    }
}

#[derive(Debug)]
struct Multiplication(i32, i32);

impl Multiplication {
    fn product(&self) -> i64 {
        (self.0 * self.1) as i64
    }
}

impl FromStr for Multiplication {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(|c| c == '(' || c == ')' || c == ',')
            .filter_map(|part| part.parse().ok())
            .collect();

        if parts.len() == 2 {
            Ok(Multiplication(parts[0], parts[1]))
        } else {
            Err(())
        }
    }
}

fn find_patterns(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut state = State::Corrupted;
    let mut buffer = String::new();

    for c in input.chars() {
        state = match state {
            State::Corrupted => match c {
                'd' => State::D,
                'm' => State::Mul(c),
                _ => State::Corrupted,
            },
            State::D => match c {
                'o' => State::Do,
                _ => State::Corrupted,
            },
            State::Do => match c {
                '(' => {
                    State::DoStart
                }
                'n' => State::Dont,
                _ => State::Corrupted,
            },
            State::DoStart => match c {
                ')' => {
                    results.push("do()".to_string());
                    State::Corrupted
                }
                _ => State::Corrupted,
            },
            State::Dont => match c {
                '\'' => State::DontApostrophe,
                _ => State::Corrupted,
            },
            State::DontApostrophe => match c {
                't' => State::DontT,
                _ => State::Corrupted,
            },
            State::DontT => match c {
                '(' => {
                    State::DontTStart
                }
                _ => State::Corrupted,
            },
            State::DontTStart => match c {
                ')' => {
                    results.push("don't()".to_string());
                    State::Corrupted
                }
                _ => State::Corrupted,
            },
            State::Mul(current) => match c {
                'u' if current == 'm' => State::Mul('u'),
                'l' if current == 'u' => State::Mul('l'),
                '(' if current == 'l' => {
                    buffer.clear();
                    State::MulOpen
                }
                _ => State::Corrupted,
            },
            State::MulOpen => match c {
                '0'..='9' => {
                    buffer.push(c);
                    State::Multiplier
                }
                _ => State::Corrupted,
            },
            State::Multiplier => match c {
                '0'..='9' if buffer.len() < 3 => {
                    buffer.push(c);
                    State::Multiplier
                }
                ',' if !buffer.is_empty() => State::Comma,
                _ => State::Corrupted,
            },
            State::Comma => {
                buffer.push(',');
                match c {
                    '0'..='9' => {
                        buffer.push(c);
                        State::Multiplicand
                    }
                    _ => State::Corrupted,
                }
            }
            State::Multiplicand => match c {
                '0'..='9' if buffer.len() < 7 => { // Allow for a comma and up to 3 digits
                    buffer.push(c);
                    State::Multiplicand
                }
                ')' if buffer.contains(',') => {
                    buffer.push(')');
                    results.push(format!("mul({})", buffer));
                    State::Corrupted
                }
                _ => State::Corrupted,
            },
        };
    }

    results
}

#[derive(Debug, PartialEq)]
enum State {
    Corrupted,
    D,
    Do,
    DoStart,
    Dont,
    DontApostrophe,
    DontT,
    DontTStart,
    Mul(char),
    MulOpen,
    Multiplier,
    Comma,
    Multiplicand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_03/input.txt");

        let product = CorruptedMemory(input.to_string()).product();

        assert_eq!(product, 196826776);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/day_03/input.txt");

        let product = CorruptedMemory(input.to_string()).product_with_conditions();

        assert_eq!(product, 106780429);
    }

    #[test]
    fn test_product() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let product = CorruptedMemory(input.to_string()).product();

        assert_eq!(product, 161);
    }

    #[test]
    fn test_product_with_conditions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let product = CorruptedMemory(input.to_string()).product_with_conditions();

        assert_eq!(product, 48);
    }

    #[test]
    fn test_mul_parsing() {
        let input = "mul(2,42)";

        let result = Multiplication::from_str(input).unwrap();

        assert_eq!(84, result.product());
    }
}
