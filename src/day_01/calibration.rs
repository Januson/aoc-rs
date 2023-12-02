fn find_calibration_value_with_words(line: &str) -> u32 {
    let sanitized = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    find_calibration_value(&sanitized)
}

fn find_calibration_value(line: &str) -> u32 {
    let numbers: Vec<u32> = line.chars()
        .filter(|c| c.is_numeric())
        .flat_map(|c| c.to_digit(10))
        .collect();

    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use crate::day_01::calibration::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_01/input.txt");
        let total_calibration_value = input.lines().into_iter()
            .map(|line| find_calibration_value(line))
            .sum::<u32>();

        assert_eq!(total_calibration_value, 55712);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_01/input.txt");
        let total_calibration_value = input.lines().into_iter()
            .map(|line| find_calibration_value_with_words(line) as u64)
            .sum::<u64>();

        assert_eq!(total_calibration_value, 55413);
    }

    #[test]
    fn finds_calibration_value() {
        assert_eq!(14, find_calibration_value("1afasf4"));
        assert_eq!(14, find_calibration_value("af1a4sf"));
    }

    #[test]
    fn should_consider_words() {
        assert_eq!(29, find_calibration_value_with_words("two1nine"));
        assert_eq!(83, find_calibration_value_with_words("eightwothree"));
        assert_eq!(13, find_calibration_value_with_words("abcone2threexyz"));
        assert_eq!(24, find_calibration_value_with_words("xtwone3four"));
        assert_eq!(42, find_calibration_value_with_words("4nineeightseven2"));
        assert_eq!(14, find_calibration_value_with_words("zoneight234"));
        assert_eq!(76, find_calibration_value_with_words("7pqrstsixteen"));
        assert_eq!(73, find_calibration_value_with_words("7pqrstsixeighthree"));
        assert_eq!(81, find_calibration_value_with_words("eightwone"));
    }

    #[test]
    fn full_example() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let result = lines.into_iter()
            .map(|line| find_calibration_value_with_words(line))
            .sum::<u32>();

        assert_eq!(281, result);
    }
}
