fn look_and_say(n: &str) -> String {
    let r = n.chars()
        .fold(Vec::new(), |mut acc, digit| {
            if let Some((last_digit, count)) = acc.last_mut() {
                if *last_digit == digit {
                    *count += 1;
                } else {
                    acc.push((digit, 1u32));
                }
            } else {
                acc.push((digit, 1));
            }
            acc
        });

    let mut result = String::new();
    for (digit, count) in r.iter() {
        result.push_str(format!("{}{}", count, digit).as_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_10/input.txt");

        let mut result = input.trim().to_string();
        for _ in 0..40 {
            result = look_and_say(&result);
        }

        assert_eq!(format!("{}", result).len(), 252594);
    }

    #[test]
    fn test_numbers() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
    }
}
