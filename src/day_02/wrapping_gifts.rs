use std::str::FromStr;

struct Gift {
    length: u32,
    width: u32,
    height: u32,
}

impl FromStr for Gift {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split('x').map(|x| x.parse().unwrap());

        Ok(Gift {
            length: dimensions.next().unwrap(),
            width: dimensions.next().unwrap(),
            height: dimensions.next().unwrap(),
        })
    }
}

impl Gift {
    fn required_paper(&self) -> u32 {
        let sides = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];

        let smallest_side = sides.iter().min().unwrap();

        2 * sides.iter().sum::<u32>() + smallest_side
    }

    fn required_ribbon(&self) -> u32 {
        let mut sides = vec![self.length, self.width, self.height];
        sides.sort();

        let ribbon = 2 * sides[0] + 2 * sides[1];
        let bow = self.length * self.width * self.height;

        ribbon + bow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_02/input.txt");

        let total_paper = input.lines()
            .map(|x| Gift::from_str(x).unwrap())
            .map(|gift| gift.required_paper())
            .sum::<u32>();

        assert_eq!(total_paper, 1606483);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_02/input.txt");

        let total_paper = input.lines()
            .map(|x| Gift::from_str(x).unwrap())
            .map(|gift| gift.required_ribbon())
            .sum::<u32>();

        assert_eq!(total_paper, 3842356);
    }

    #[test]
    fn gift_parsing() {
        let input = "2x3x4";

        let gift = Gift::from_str(input).unwrap();

        assert_eq!(gift.length, 2);
        assert_eq!(gift.width, 3);
        assert_eq!(gift.height, 4);
    }

    #[test]
    fn required_wrapping() {
        let input = "2x3x4";

        let gift = Gift::from_str(input).unwrap();

        assert_eq!(gift.required_paper(), 58);
    }

    #[test]
    fn required_ribbon() {
        let input = "2x3x4";

        let gift = Gift::from_str(input).unwrap();

        assert_eq!(gift.required_ribbon(), 34);
    }
}
