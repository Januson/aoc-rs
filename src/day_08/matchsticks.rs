fn count_code_characters(input: &str) -> u32 {
    input.len() as u32
}

fn count_memory_characters(s: &str) -> u32 {
    let mut count = 0;
    let mut i = 1;
    let chars: Vec<char> = s.chars().collect();

    while i < s.len() - 1 {
        if chars[i] == '\\' {
            i += if chars[i + 1] == 'x' { 4 } else { 2 };
        } else {
            i += 1;
        }
        count += 1;
    }

    count
}

fn count_escaped_characters(s: &str) -> u32 {
    let mut len = 0;
    for c in s.chars() {
        match c {
            '\\' | '\"' => len += 2,
            _ => len += 1,
        }
    }

    len + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_08/input.txt");

        let total: u64 = input.lines()
            .map(|line| {
                (count_code_characters(line) - count_memory_characters(line)) as u64
            })
            .sum();

        assert_eq!(total, 1350);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_08/input.txt");

        let total: u64 = input.lines()
            .map(|line| {
                (count_escaped_characters(line) - count_code_characters(line)) as u64
            })
            .sum();

        assert_eq!(total, 2085);
    }

    #[test]
    fn test_example() {
        let input = "\
            \"\"\n\
            \"abc\"\n\
            \"aaa\\\"aaa\"\n\
            \"\\x27\"\n\
            ";

        let total: u64 = input.lines()
            .map(|line| {
                (count_code_characters(line) - count_memory_characters(line)) as u64
            })
            .sum();

        assert_eq!(total, 12);
    }
    #[test]
    fn test_example2() {
        let input = "\
            \"\"\n\
            \"abc\"\n\
            \"aaa\\\"aaa\"\n\
            \"\\x27\"\n\
            ";

        let total: u64 = input.lines()
            .map(|line| {
                (count_escaped_characters(line) - count_code_characters(line)) as u64
            })
            .sum();

        assert_eq!(total, 19);
    }

    #[test]
    fn test_code_counting() {
        assert_eq!(count_code_characters(r#""""#), 2);
        assert_eq!(count_code_characters(r#""abc""#), 5);
        assert_eq!(count_code_characters(r#""aaa\"aaa""#), 10);
        assert_eq!(count_code_characters(r#""\x27""#), 6);
    }

    #[test]
    fn test_memory_counting() {
        assert_eq!(count_memory_characters(r#""""#), 0);
        assert_eq!(count_memory_characters(r#""abc""#), 3);
        assert_eq!(count_memory_characters(r#""aaa\"aaa""#), 7);
        assert_eq!(count_memory_characters(r#""\x27""#), 1);
        assert_eq!(count_memory_characters(r#""byc\x9dyxuafof\\\xa6uf\\axfozomj\\olh\x6a""#), 29);
        assert_eq!(count_memory_characters(r#""vvdnb\\x\\uhnxfw\"dpubfkxfmeuhnxisd""#), 32);
        assert_eq!(count_memory_characters(r#""\"i\x13r\"l""#), 6);
        assert_eq!(count_memory_characters(r#""qxfcsmh""#), 7);
        assert_eq!(count_memory_characters(r#""mdag\x0asnck\xc2ggj\"slb\"fjy""#), 21);
    }

    #[test]
    fn test_escaped_counting() {
        assert_eq!(count_escaped_characters(r#""""#), 6);
        assert_eq!(count_escaped_characters(r#""abc""#), 9);
        assert_eq!(count_escaped_characters(r#""aaa\"aaa""#), 16);
        assert_eq!(count_escaped_characters(r#""\x27""#), 11);
    }
}
