use std::str::FromStr;

use crate::day_02::game::Color::{Blue, Green, Red};

#[derive(Eq, PartialEq, Hash, Debug)]
enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, color) = s.trim().split_once(' ').unwrap();
        let value = match value.trim().parse::<u8>() {
            Ok(v) => v,
            Err(_) => panic!("Number was expected!")
        };
        match color.trim() {
            "red" => Ok(Red(value)),
            "green" => Ok(Green(value)),
            "blue" => Ok(Blue(value)),
            _ => Err(())
        }
    }
}

struct Game {
    id: u8,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u8, draws: Vec<Draw>) -> Self {
        Game {
            id,
            draws,
        }
    }

    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        for draw in &self.draws {
            if !draw.is_possible(red, green, blue) {
                return false
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, draws) = s.trim().split_once(':').unwrap();
        let (_, id) = label.trim().split_once(' ').unwrap();
        let id = match id.trim().parse::<u8>() {
            Ok(id) => id,
            Err(_) => panic!("Number was expected!"),
        };

        let draws = draws.split(';')
            .map(|s| Draw::from_str(s).unwrap())
            .collect();

        Ok(Game::new(id, draws))
    }
}

#[derive(Debug)]
struct Draw {
    stones: Vec<Color>,
}

impl Draw {
    fn new(stones: Vec<Color>) -> Self {
        Draw { stones }
    }

    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        for stone in &self.stones {
            let possible = match stone {
                Red(n) => n <= &red,
                Green(n) => n <= &green,
                Blue(n) => n <= &blue,
            }            ;

            if !possible {
                return false;
            }
        }

        true
    }
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(',')
            .map(|s| Color::from_str(s).unwrap())
            .collect::<Vec<Color>>();

        Ok(Draw::new(split))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_02/input.txt");
        let total = input.lines().into_iter()
            .map(|line| Game::from_str(line).unwrap())
            .filter(|game| game.is_possible(12, 13, 14))
            .map(|game| game.id as u32)
            .sum::<u32>();

        assert_eq!(total, 1867);
    }

    #[test]
    fn game_possibility() {
        let draws = vec![
            Draw::new(vec![Red(0), Green(0), Blue(0)]),
            Draw::new(vec![Red(1), Green(2), Blue(3)]),
            Draw::new(vec![Red(12), Green(13), Blue(14)]),
        ];
        let game = Game::new(1, draws);

        assert_eq!(true, game.is_possible(12, 13, 14));
        assert_eq!(false, game.is_possible(0, 0, 0));
        assert_eq!(false, game.is_possible(12, 13, 13));
    }

    #[test]
    fn color_parsing() {
        assert_eq!(Red(4), Color::from_str("4 red").unwrap());
        assert_eq!(Green(15), Color::from_str("15 green").unwrap());
        assert_eq!(Blue(1), Color::from_str("1 blue").unwrap());
    }

    #[test]
    fn draw_parsing() {
        let draw = Draw::from_str(" 3 red, 4 green, 16 blue").unwrap();
        assert_eq!(vec![Red(3), Green(4), Blue(16)], draw.stones);
    }

    #[test]
    fn game_parsing() {
        let game = Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap();

        assert_eq!(3, game.id);
        assert_eq!(3, game.draws.len());
        assert_eq!(vec![Green(5), Red(1)], game.draws[2].stones);
    }

}