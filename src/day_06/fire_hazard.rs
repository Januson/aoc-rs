use crate::utils::points_2d::{range_of, Point};
use std::collections::HashMap;
use std::str::FromStr;

struct LightGrid {
    grid: HashMap<Point, Light>,
}

impl LightGrid {
    fn new() -> Self {
        LightGrid { grid: HashMap::new() }
    }

    fn toggle(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnOn(start, end) => {
                let points = range_of(start..end);
                points.for_each(|point| {
                    self.grid.entry(point).or_insert(Light::new()).on();
                });
            }
            Instruction::TurnOff(start, end) => {
                let points = range_of(start..end);
                points.for_each(|point| {
                    self.grid.entry(point).or_insert(Light::new()).off();
                });
            }
            Instruction::Toggle(start, end) => {
                let points = range_of(start..end);
                points.for_each(|point| {
                    let light = self.grid.entry(point).or_insert(Light::new());
                    match light.state {
                        LightState::On => {
                            light.on();
                            light.on();
                            light.on();
                            light.off();
                        },
                        LightState::Off => {
                            light.on();
                            light.on();
                        },
                    };
                });
            }
        }
    }

    fn lights_on(&self) -> usize {
        self.grid.values()
            .filter(|&light| light.state == LightState::On)
            .count()
    }

    fn total_brightness(&self) -> u64 {
        self.grid.values()
            .map(|light| light.brightness as u64)
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct  Light {
    state: LightState,
    brightness: u32,
}

impl Light {
    fn new() -> Self {
        Light { state: LightState::Off, brightness: 0 }
    }

    fn on(&mut self) {
        self.brightness += 1;
        self.state = LightState::On;
    }

    fn off(&mut self) {
        if self.brightness > 0 {
            self.brightness -= 1;
        }
        self.state = LightState::Off;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LightState {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    TurnOn(Point, Point),
    TurnOff(Point, Point),
    Toggle(Point, Point),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        match parts[0] {
            "turn" => {
                let start: Point = parts[2].parse().expect("Expected a point");
                let end: Point = parts[4].parse().expect("Expected a point");

                match parts[1] {
                    "on" => Ok(Instruction::TurnOn(start, end)),
                    "off" => Ok(Instruction::TurnOff(start, end)),
                    _ => panic!("Invalid instruction"),
                }
            }
            "toggle" => {
                let start: Point = parts[1].parse().expect("Expected a point");
                let end: Point = parts[3].parse().expect("Expected a point");
                let instruction = Instruction::Toggle(start, end);
                Ok(instruction)
            },
            _ => panic!("Invalid instruction"),
        }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();

        let x = parts[0].parse::<i32>().expect("Expected a number");
        let y = parts[1].parse::<i32>().expect("Expected a number");

        Ok(Point::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_06/input.txt");

        let mut light_grid = LightGrid::new();
        input.lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .for_each(|instruction| light_grid.toggle(instruction));

        assert_eq!(light_grid.lights_on(), 400410);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_06/input.txt");

        let mut light_grid = LightGrid::new();
        input.lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .for_each(|instruction| light_grid.toggle(instruction));

        assert_eq!(light_grid.total_brightness(), 15343601);
    }

    #[test]
    fn test_on_instruction_parsing() {
        let input = "turn on 0,0 through 999,999";

        let instruction = Instruction::from_str(input).unwrap();

        assert_eq!(instruction, Instruction::TurnOn(Point::new(0, 0), Point::new(999, 999)));
    }

    #[test]
    fn test_off_instruction_parsing() {
        let input = "turn off 0,0 through 999,998";

        let instruction = Instruction::from_str(input).unwrap();

        assert_eq!(instruction, Instruction::TurnOff(Point::new(0, 0), Point::new(999, 998)));
    }

    #[test]
    fn test_toggle_instruction_parsing() {
        let input = "toggle 0,0 through 999,997";

        let instruction = Instruction::from_str(input).unwrap();

        assert_eq!(instruction, Instruction::Toggle(Point::new(0, 0), Point::new(999, 997)));
    }

    #[test]
    fn test_switching_lights() {
        let mut light_grid = LightGrid::new();

        let turn_on = Instruction::TurnOn(Point::new(0, 0), Point::new(4, 4));
        light_grid.toggle(turn_on);
        assert_eq!(light_grid.lights_on(), 25);

        let turn_off = Instruction::TurnOff(Point::new(0, 0), Point::new(2, 2));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.lights_on(), 16);

        let turn_off = Instruction::Toggle(Point::new(3, 3), Point::new(4, 4));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.lights_on(), 12);

        let turn_off = Instruction::Toggle(Point::new(1, 1), Point::new(2, 2));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.lights_on(), 16);
    }

    #[test]
    fn test_tuning_brightness() {
        let mut light_grid = LightGrid::new();

        let turn_on = Instruction::TurnOn(Point::new(0, 0), Point::new(4, 4));
        light_grid.toggle(turn_on);
        assert_eq!(light_grid.total_brightness(), 25);

        let turn_on = Instruction::TurnOn(Point::new(0, 0), Point::new(4, 4));
        light_grid.toggle(turn_on);
        assert_eq!(light_grid.total_brightness(), 50);

        let turn_off = Instruction::TurnOff(Point::new(0, 0), Point::new(2, 2));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.total_brightness(), 41);

        let turn_off = Instruction::Toggle(Point::new(3, 3), Point::new(4, 4));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.total_brightness(), 49);

        let turn_off = Instruction::Toggle(Point::new(1, 1), Point::new(2, 2));
        light_grid.toggle(turn_off);
        assert_eq!(light_grid.total_brightness(), 57);
    }
}
