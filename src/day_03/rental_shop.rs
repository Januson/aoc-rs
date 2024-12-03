#[derive(Debug)]
struct CorruptedMemory(String);

impl CorruptedMemory {
    fn product(&self) -> i64 {
        let res: i64 = self.uncorrupted().iter()
            .map(|x| x.product())
            .sum();
        res
    }

    fn uncorrupted(&self) -> Vec<Multiplication> {
        find_multiplications(&self.0)
    }
}

#[derive(Debug)]
struct Multiplication(i32, i32);

impl Multiplication {
    fn product(&self) -> i64 {
        (self.0 * self.1) as i64
    }
}

fn find_multiplications(input: &str) -> Vec<Multiplication> {
    let mut results = Vec::new();
    let mut state = State::Corrupted;
    let mut num1 = String::new();
    let mut num2 = String::new();

    for c in input.chars() {
        state = match state {
            State::Corrupted => {
                if c == 'm' {
                    State::MUL(c)
                } else {
                    State::Corrupted
                }
            }
            State::MUL(current) => {
                if current == 'm' && c == 'u' || current == 'u' && c == 'l' {
                    State::MUL(c)
                } else if current == 'l' && c == '(' {
                    State::OpenParen
                } else {
                    State::Corrupted
                }
            }
            State::OpenParen => {
                if c.is_ascii_digit() {
                    num1.push(c);
                    State::Multiplier
                } else {
                    State::Corrupted
                }
            }
            State::Multiplier => {
                if c.is_ascii_digit() && num1.len() < 3 {
                    num1.push(c);
                    State::Multiplier
                } else if c == ',' && !num1.is_empty() {
                    State::Comma
                } else {
                    num1.clear();
                    State::Corrupted
                }
            }
            State::Comma => {
                if c.is_ascii_digit() {
                    num2.push(c);
                    State::Multiplicand
                } else {
                    num1.clear();
                    State::Corrupted
                }
            }
            State::Multiplicand => {
                if c.is_ascii_digit() && num2.len() < 3 {
                    num2.push(c);
                    State::Multiplicand
                } else if c == ')' && !num2.is_empty() {
                    if let (Ok(n1), Ok(n2)) = (num1.parse(), num2.parse()) {
                        results.push(Multiplication(n1, n2));
                    }
                    num1.clear();
                    num2.clear();
                    State::Corrupted
                } else {
                    num1.clear();
                    num2.clear();
                    State::Corrupted
                }
            }
        };
    }

    results
}

#[derive(Debug, PartialEq)]
enum State {
    Corrupted,
    MUL(char),
    OpenParen,
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
    fn test_product() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let product = CorruptedMemory(input.to_string()).product();

        assert_eq!(product, 161);
    }
}
