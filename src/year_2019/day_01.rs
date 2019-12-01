struct Module {
    mass: u64,
}

impl Module {
    fn new(mass: u64) -> Module {
        Module {
            mass,
        }
    }

    fn required_fuel(&self) -> u64 {
        self.fuel_for_mass(self.mass) as u64
    }

    fn required_total_fuel(&self) -> u64 {
        self.fuel(self.mass) as u64
    }

    fn fuel(&self, mass: u64) -> i64 {
        let fuel = self.fuel_for_mass(mass);
        if fuel <= 0 {
            0
        } else {
            fuel + self.fuel(fuel as u64) as i64
        }
    }

    fn fuel_for_mass(&self, mass: u64) -> i64 {
        (mass / 3) as i64 - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_of_evenly_divisible_mass() {
        let module = Module::new(12);

        assert_eq!(module.required_fuel(), 2);
    }

    #[test]
    fn fuel_of_unevenly_divisible_mass() {
        let module = Module::new(14);

        assert_eq!(module.required_fuel(), 2);
    }

    #[test]
    fn fuel_requires_no_fuel() {
        let module = Module::new(12);

        assert_eq!(module.required_total_fuel(), 2);
    }

    #[test]
    fn fuel_requires_more_fuel() {
        let module = Module::new(100756);

        assert_eq!(module.required_total_fuel(), 50346);
    }

    #[test]
    fn first_part() {
        let input = include_str!("../../inputs/year_2019/day_01/input.txt");
        let total_required_fuel = input.lines().into_iter()
            .map(|n| n.parse::<u64>().expect("Expected positive number"))
            .map(|mass| Module::new(mass))
            .map(|module| module.required_fuel())
            .sum::<u64>();

        assert_eq!(total_required_fuel, 3266288);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../inputs/year_2019/day_01/input.txt");
        let total_required_fuel = input.lines().into_iter()
            .map(|n| n.parse::<u64>().expect("Expected positive number"))
            .map(|mass| Module::new(mass))
            .map(|module| module.required_total_fuel())
            .sum::<u64>();

        assert_eq!(total_required_fuel, 4896582);
    }
}