#[cfg(test)]
mod tests {

    use crate::util::accumulate::AccumulateIterator;
    use crate::util::duplicates::DuplicateIterator;

    #[test]
    fn first_part() {
        let input = include_str!("../../inputs/year_2018/day_01/input.txt");
        let frequency: i32 = input.lines().into_iter()
            .map(|n| n.parse::<i32>().expect("Integer was expected!"))
            .sum();
        assert_eq!(frequency, 427);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../inputs/year_2018/day_01/input.txt");
        let frequency: i32 = input.lines().into_iter()
            .map(|n| n.parse::<i32>()
                .expect("Integer was expected!"))
            .cycle()
            .accumulate(|a, b| a + b)
            .duplicates()
            .nth(0)
            .expect("Not found");

        assert_eq!(frequency, 341);
    }

}
