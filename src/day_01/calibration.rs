fn find_calibration_value(line: &str) -> u32 {
    let first = line.chars()
        .filter(|c| c.is_numeric())
        .flat_map(|c| c.to_digit(10))
        .next().unwrap();
    let last = line.chars().rev()
        .filter(|c| c.is_numeric())
        .flat_map(|c| c.to_digit(10))
        .next().unwrap();
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use crate::day_01::calibration::find_calibration_value;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_01/input.txt");
        let total_calibration_value = input.lines().into_iter()
            .map(|line| find_calibration_value(line))
            .sum::<u32>();

        assert_eq!(total_calibration_value, 55712);
    }

    #[test]
    fn finds_calibration_value() {
        assert_eq!(14, find_calibration_value("1afasf4"));
        assert_eq!(14, find_calibration_value("af1a4sf"));
    }
}