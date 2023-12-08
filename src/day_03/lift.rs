use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Number {
    row: usize,
    start: usize,
    digits: Vec<char>,
}

impl Number {
    fn new(row: usize, start: usize) -> Self {
        Number {
            row,
            start,
            digits: Vec::new(),
        }
    }

    fn to_number(&self) -> u32 {
        self.digits.iter()
            .fold(0, |a, b| a * 10 + b.to_digit(10).unwrap())
    }

    fn neighbours(&self) -> Vec<Point> {
        let start = 0.max(self.start as i32 - 1) as usize;
        let end = self.start + self.digits.len() + 1;

        let above: Vec<Point> = if self.row == 0 {
            Vec::new()
        } else {
            let row_above = 0.max(self.row as i32 - 1) as usize;
            (start..end).into_iter()
                .map(|x| Point::new(x, row_above))
                .collect()
        };

        let around = vec![
            Point::new(start, self.row),
            Point::new(self.start + self.digits.len(), self.row),
        ];
        let below = (start..end).into_iter()
            .map(|x| Point::new(x, self.row + 1));

        above.into_iter()
            .chain(around.into_iter())
            .chain(below.into_iter())
            .collect::<Vec<Point>>()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Gear(u32, u32);

impl Gear {
    fn ratio(&self) -> u32 {
        self.0 * self.1
    }
}

struct Schematic {
    plan: Vec<Vec<char>>,
}

impl Schematic {
    fn new(plan: Vec<Vec<char>>) -> Self {
        Schematic {
            plan,
        }
    }

    fn part_numbers(&self) -> Vec<u32> {
        let all_numbers = self.all_numbers();

        all_numbers.iter()
            .filter(|number| self.is_part_number(*number))
            .map(|number| number.to_number())
            .collect()
    }

    fn gears(&self) -> Vec<Gear> {
        let all_numbers = self.all_numbers();
        let mut gears: HashMap<Point, Vec<&Number >> = HashMap::new();

        all_numbers.iter()
            .for_each(|number| self.add_gears(&mut gears, number));

        gears.values().into_iter()
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| Gear(numbers[0].to_number(), numbers[1].to_number()))
            .collect()
    }

    fn add_gears<'a>(&self, gears: &mut HashMap<Point, Vec<&'a Number>>, number: &'a Number) {
        for neighbour in number.neighbours() {
            if neighbour.y >= self.plan.len() { continue; }
            if neighbour.x >= self.plan[0].len() { continue; }
            let char = &self.plan[neighbour.y][neighbour.x];
            if *char == '*' {
                gears.entry(neighbour).or_insert(Vec::new()).push(number);
            }
        }
    }

    fn is_part_number(&self, number: &Number) -> bool {
        for neighbour in number.neighbours() {
            if neighbour.y >= self.plan.len() { continue; }
            if neighbour.x >= self.plan[0].len() { continue; }
            let char = &self.plan[neighbour.y][neighbour.x];
            if *char != '.' && !char.is_numeric() {
                return true;
            }
        }

        false
    }

    fn all_numbers(&self) -> Vec<Number> {
        let mut numbers: Vec<Number> = Vec::new();
        for (row_id, row) in self.plan.iter().enumerate() {
            let mut current_number: Option<Number> = None;
            for (id, char) in row.iter().enumerate() {
                if char.is_numeric() {
                    if current_number.is_none() {
                        current_number = Some(Number::new(row_id, id))
                    }
                    current_number = match current_number.take() {
                        Some(mut n) => {
                            n.digits.push(*char);
                            Some(n)
                        }
                        None => None,
                    };
                    continue;
                }

                if let Some(n) = current_number.take() {
                    numbers.push(n);
                }
            }

            if let Some(n) = current_number.take() {
                numbers.push(n);
            }
        }

        numbers
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plan = s.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Ok(Schematic::new(plan))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_03/input.txt");
        let schematic = Schematic::from_str(input).unwrap();

        let result = schematic.part_numbers().iter().sum::<u32>();

        assert_eq!(528819, result);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_03/input.txt");
        let schematic = Schematic::from_str(input).unwrap();

        let result = schematic.gears().iter()
            .map(|gear| gear.ratio())
            .sum::<u32>();

        assert_eq!(80403602, result);
    }

    #[test]
    fn example_parts() {
        let input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            .......755\n\
            ...$..*...\n\
            .664.598..\n\
        ";

        let schematic = Schematic::from_str(input).unwrap();

        let numbers = schematic.all_numbers();
        assert_eq!(467, numbers[0].to_number());
        assert_eq!(114, numbers[1].to_number());
        assert_eq!(35, numbers[2].to_number());
        assert_eq!(633, numbers[3].to_number());
        assert_eq!(617, numbers[4].to_number());
        assert_eq!(58, numbers[5].to_number());
        assert_eq!(592, numbers[6].to_number());
        assert_eq!(755, numbers[7].to_number());
        assert_eq!(664, numbers[8].to_number());
        assert_eq!(598, numbers[9].to_number());

        let part_numbers: Vec<&Number> = numbers.iter()
            .filter(|n| schematic.is_part_number(*n))
            .collect();
        assert_eq!(467, part_numbers[0].to_number());
        assert_eq!(35, part_numbers[1].to_number());
        assert_eq!(633, part_numbers[2].to_number());
        assert_eq!(617, part_numbers[3].to_number());
        assert_eq!(592, part_numbers[4].to_number());
        assert_eq!(755, part_numbers[5].to_number());
        assert_eq!(664, part_numbers[6].to_number());
        assert_eq!(598, part_numbers[7].to_number());

        let total = schematic.part_numbers().iter().sum::<u32>();
        assert_eq!(4361, total);
    }

    #[test]
    fn example_gears() {
        let input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            .......755\n\
            ...$..*...\n\
            .664.598..\n\
        ";

        let schematic = Schematic::from_str(input).unwrap();

        let gears: Vec<Gear> = schematic.gears();
        assert_eq!(true, gears.contains(&Gear(467, 35)));
        assert_eq!(true, gears.contains(&Gear(755, 598)));

        let total = schematic.gears().iter()
            .map(|gear| gear.ratio())
            .sum::<u32>();
        assert_eq!(467835, total);
    }

    #[test]
    fn top_left_corner_number() {
        let mut number = Number::new(0, 0);

        number.digits.push('4');
        number.digits.push('6');
        number.digits.push('7');

        let neighbours = number.neighbours();

        assert_eq!(neighbours, vec![
            Point::new(0, 0),
            Point::new(3, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
        ]);
    }

    #[test]
    fn right_corner_number() {
        let mut number = Number::new(1, 2);

        number.digits.push('3');
        number.digits.push('5');

        let neighbours = number.neighbours();

        assert_eq!(neighbours, vec![
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(1, 1),
            Point::new(4, 1),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 2),
        ]);
    }

    #[test]
    fn numbers() {
        let mut number1 = Number::new(1, 2);
        number1.digits.push('3');
        number1.digits.push('5');
        number1.digits.push('9');

        let mut number2 = Number::new(1, 2);
        number2.digits.push('2');
        number2.digits.push('3');
        number2.digits.push('9');

        let sum: u32 = vec![number1, number2].iter().map(|n| n.to_number()).sum();

        assert_eq!(598, sum);
    }

}
