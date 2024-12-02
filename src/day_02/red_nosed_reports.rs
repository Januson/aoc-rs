use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Report(Vec<i32>);

impl Report {
    fn is_safe_dampened(&self) -> bool {
        if self.is_safe() {
            true
        } else {
            for i in 0..self.0.len() {
                let mut levels = self.0.clone();
                levels.remove(i);
                if Report(levels).is_safe() {
                    return true;
                }
            }
            false
        }
    }

    fn is_safe(&self) -> bool {
        let direction = self.0[1] - self.0[0];
        self.0.windows(2).all(|x| {
            let difference = x[1] - x[0];
            if (difference == 0) || (difference.abs() > 3) {
                return false;
            }
            if direction * difference > 0 {
                return true;
            }
            return false;
        })
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report(s.split_whitespace().map(|x| x.parse().unwrap()).collect()))
    }
}

fn parse_reports(input: &str) -> Vec<Report> {
    input.lines()
        .map(|line| {
            Report::from_str(line).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_02/input.txt");
        let reports = parse_reports(input);

        let safe_reports = reports.iter()
            .filter(|report| report.is_safe())
            .count();

        assert_eq!(246, safe_reports);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/day_02/input.txt");
        let reports = parse_reports(input);

        let safe_reports = reports.iter()
            .filter(|report| report.is_safe_dampened())
            .count();

        assert_eq!(318, safe_reports);
    }

    #[test]
    fn test_report_safety_decreasing() {
        let report = Report(vec![7, 6, 4, 2, 1]);
        assert_eq!(true, report.is_safe());
    }

    #[test]
    fn test_report_safety_increasing() {
        let report = Report(vec![1, 3, 6, 7, 9]);
        assert_eq!(true, report.is_safe());
    }

    #[test]
    fn test_report_unsafe_stagnation() {
        let report = Report(vec![1, 3, 3, 7, 9]);
        assert_eq!(false, report.is_safe());
    }

    #[test]
    fn test_report_unsafe_non_linear() {
        assert_eq!(false, Report(vec![1, 9]).is_safe());
        assert_eq!(false, Report(vec![9, 1]).is_safe());
    }

    #[test]
    fn test_report_unsafe_dampener() {
        assert_eq!(true, Report(vec![1, 9, 3]).is_safe_dampened());
        assert_eq!(false, Report(vec![1, 1, 9, 3]).is_safe_dampened());
        assert_eq!(false, Report(vec![1, 1, 9, 3]).is_safe_dampened());
        assert_eq!(true, Report(vec![7, 6, 4, 2, 1]).is_safe_dampened());
        assert_eq!(true, Report(vec![1, 3, 2, 4, 5]).is_safe_dampened());
        assert_eq!(true, Report(vec![8, 6, 4, 4, 1]).is_safe_dampened());
        assert_eq!(true, Report(vec![1, 3, 6, 7, 9]).is_safe_dampened());
        assert_eq!(false, Report(vec![9, 9, 7, 7, 1]).is_safe_dampened());
        assert_eq!(false, Report(vec![1, 2, 7, 8, 9]).is_safe_dampened());
        assert_eq!(false, Report(vec![9, 7, 6, 2, 1]).is_safe_dampened());
    }

    #[test]
    fn report_parsing() {
        let input = "1 2 3 4 5\n4 3 9 5";
        let expected_report_1 = Report(vec![1, 2, 3, 4, 5]);
        let expected_report_2 = Report(vec![4, 3, 9, 5]);

        let reports = parse_reports(input);

        assert_eq!(expected_report_1, reports[0]);
        assert_eq!(expected_report_2, reports[1]);
    }
}
