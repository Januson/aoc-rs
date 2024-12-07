use std::str::FromStr;

struct TextBlock(Vec<String>);

impl FromStr for TextBlock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TextBlock(s.lines().map(|s| s.to_string()).collect()))
    }
}

impl TextBlock {
    fn all_lines(&self) -> Vec<String> {
        let mut all_lines = vec![];

        all_lines.extend(self.rows());
        all_lines.extend(self.columns());
        all_lines.extend(self.diagonals());

        all_lines
    }

    fn reversed(&self) -> Vec<String> {
        let mut reversed = self.0.clone();
        reversed.reverse();
        reversed
    }

    fn rows(&self) -> Vec<String> {
        self.0.clone()
    }

    fn columns(&self) -> Vec<String> {
        let mut columns = vec![];
        for column in 0..self.0.len() {
            let mut buffer = String::new();
            for row in &self.0 {
                let char = row.chars().nth(column).unwrap();
                buffer.push(char);
            }
            columns.push(buffer);
        }

        columns
    }

    fn diagonals(&self) -> Vec<String> {
        let mut diagonals = vec![];

        diagonals.extend(self.find_diagonals(&self.0[..]));
        diagonals.extend(self.find_diagonals(&self.reversed()[..]));

        diagonals
    }

    fn find_diagonals(&self, table: &[String]) -> Vec<String> {
        let length = table.len();
        let diagonal_lines = (length + length) - 1;
        let mid_point = (diagonal_lines / 2) + 1;
        let mut items_in_diagonal = 0;
        let mut output = Vec::new();

        for i in 1..=diagonal_lines {
            let mut items = String::new();
            let mut row_index;
            let mut column_index;

            if i <= mid_point {
                items_in_diagonal += 1;
                for j in 0..items_in_diagonal {
                    row_index = (i - j) - 1;
                    column_index = j;
                    items.push(table[row_index].chars().nth(column_index).unwrap());
                }
            } else {
                items_in_diagonal -= 1;
                for j in 0..items_in_diagonal {
                    row_index = (length - 1) - j;
                    column_index = (i - length) + j;
                    items.push(table[row_index].chars().nth(column_index).unwrap());
                }
            }

            output.push(items);
        }

        output
    }
}

struct CeresSearch {
    text: TextBlock,
}

impl CeresSearch {
    fn new(text: TextBlock) -> CeresSearch {
        CeresSearch { text }
    }

    fn search(&self, keys: &[&str]) -> i32 {
        let mut result = 0;
        let all_lines = self.text.all_lines();

        for line in all_lines {
            for key in keys {
                for _ in line.match_indices(key) {
                    result += 1;
                }
            }
        }

        result
    }

    fn count_x_mas(&self) -> usize {
        let grid: &[String] = &self.text.0;
        let rows = grid.len();
        let cols = grid[0].len();

        let mut count = 0;

        for x in 1..(rows - 1) {
            for y in 1..(cols - 1) {
                let center = Self::char_at(grid, x, y);
                let top_left = Self::char_at(grid, x - 1, y - 1);
                let top_right = Self::char_at(grid, x - 1, y + 1);
                let bottom_left = Self::char_at(grid, x + 1, y - 1);
                let bottom_right = Self::char_at(grid, x + 1, y + 1);

                if self.forms_mas(top_left, center, bottom_right) &&
                    self.forms_mas(top_right, center, bottom_left) {
                    count += 1;
                }
            }
        }

        count
    }

    fn char_at(grid: &[String], x: usize, y: usize) -> char {
        grid[x].chars().nth(y).unwrap_or('_')
    }

    fn forms_mas(&self, left: char, middle: char, right: char) -> bool {
        middle == 'A' && ((left == 'M' && right == 'S') || (left == 'S' && right == 'M'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_04/input.txt");
        let text: TextBlock = TextBlock::from_str(input).unwrap();
        let keys = vec!["XMAS", "SAMX"];

        let cerex = CeresSearch::new(text);

        assert_eq!(2562, cerex.search(&keys));
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/day_04/input.txt");
        let text: TextBlock = TextBlock::from_str(input).unwrap();

        let cerex = CeresSearch::new(text);

        assert_eq!(1902, cerex.count_x_mas());
    }

    #[test]
    fn test_get_all_lines() {
        let input = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ];
        let expected = vec![
            "abc",
            "def",
            "ghi",
            "adg",
            "beh",
            "cfi",
            "a",
            "db",
            "gec",
            "hf",
            "i",
            "g",
            "dh",
            "aei",
            "bf",
            "c",
        ];

        let text = TextBlock(input);

        assert_eq!(expected, text.all_lines());
    }

    #[test]
    fn test_get_rows() {
        let input = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ];
        let expected = vec![
            "abc",
            "def",
            "ghi",
        ];

        let text = TextBlock(input);

        assert_eq!(expected, text.rows());
    }

    #[test]
    fn test_get_columns() {
        let input = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ];
        let expected = vec![
            "adg",
            "beh",
            "cfi",
        ];

        let text = TextBlock(input);

        assert_eq!(expected, text.columns());
    }

    #[test]
    fn test_get_diagonals() {
        let input = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ];
        let expected = vec![
            "a",
            "db",
            "gec",
            "hf",
            "i",
        ];

        let text = TextBlock(vec![]);

        assert_eq!(expected, text.find_diagonals(&input[..]));
    }

    #[test]
    fn test_get_diagonals2() {
        let input = vec![
            "ghi".to_string(),
            "def".to_string(),
            "abc".to_string(),
        ];
        let expected = vec![
            "g",
            "dh",
            "aei",
            "bf",
            "c",
        ];

        let text = TextBlock(vec![]);

        assert_eq!(expected, text.find_diagonals(&input[..]));
    }

    #[test]
    fn test_text_parsing() {
        let input = "abc\ndef\nghi";
        let expected = vec![
            "abc",
            "def",
            "ghi",
        ];

        let text: TextBlock = TextBlock::from_str(input).unwrap();

        assert_eq!(expected, text.0);
    }

    #[test]
    fn test_text_reversing() {
        let input = vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
        ];
        let expected = vec![
            "ghi".to_string(),
            "def".to_string(),
            "abc".to_string(),
        ];

        let text: TextBlock = TextBlock(input);

        assert_eq!(expected, text.reversed());
    }

    #[test]
    fn test_cerex_search() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        let keys = vec!["XMAS", "SAMX"];

        let cerex = CeresSearch::new(TextBlock(input));

        assert_eq!(18, cerex.search(&keys[..]));
    }

    #[test]
    fn test_cerex_search2() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let cerex = CeresSearch::new(TextBlock(input));

        assert_eq!(9, cerex.count_x_mas());
    }
}
