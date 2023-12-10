use std::str::FromStr;

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Mapper,
    soil_to_fertilizer: Mapper,
    fertilizer_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temperature: Mapper,
    temperature_to_humidity: Mapper,
    humidity_to_location: Mapper,
}

struct Seeds {
    start: u64,
    length: u64,
}

impl Seeds {
    fn new(start: u64, length: u64) -> Self {
        Seeds {
            start, length
        }
    }

    fn all(&self) -> Vec<u64> {
        let end = self.start + self.length;
        (self.start..end).collect::<Vec<u64>>()
    }
}

impl Almanac {
    fn seeds(&mut self) -> Vec<Seeds> {
        let mut seeds = self.seeds.iter();
        let mut all_seeds: Vec<Seeds> = Vec::new();
        while let (Some(start), Some(length)) = (seeds.next(), seeds.next()) {
            let seed = Seeds::new(*start, *length);
            all_seeds.push(seed);
        }

        all_seeds
    }

    fn locations_of(&mut self, seeds: &Vec<u64>) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::new();
        for seed in seeds {
            let soil = &self.seed_to_soil.get(*seed);
            let fertilizer = &self.soil_to_fertilizer.get(*soil);
            let water = &self.fertilizer_to_water.get(*fertilizer);
            let light = &self.water_to_light.get(*water);
            let temperature = &self.light_to_temperature.get(*light);
            let humidity = &self.temperature_to_humidity.get(*temperature);
            let location = &self.humidity_to_location.get(*humidity);

            locations.push(*location);
        }

        locations
    }

    fn smallest_location_of(&mut self, seeds: Vec<u64>) -> Option<u64> {
        let locations = self.locations_of(&seeds);
        match locations.iter().min() {
            None => None,
            Some(location) => Some(*location)
        }
    }

}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seeds: Vec<u64> = lines.next().iter()
            .flat_map(|line| line.strip_prefix("seeds: "))
            .flat_map(|line| line.split(' '))
            .map(|seed| seed.trim().parse::<u64>().unwrap())
            .collect();

        let mut mappers: Vec<Mapper> = Vec::new();
        let mut mapper: Option<Mapper> = None;
        while let Some(line) = lines.next() {
            if line.is_empty() {
                if let Some(m) = mapper.take() {
                    mappers.push(m);
                }
                mapper = Some(Mapper::new());
                lines.next();
                continue;
            }
            let mapping: Vec<u64> = line.split(' ')
                .map(|m| m.trim().parse::<u64>().unwrap())
                .collect();

            match mapper {
                None => panic!("Mapper is missing"),
                Some(ref mut m) => m.add(mapping[0], mapping[1], mapping[2]),
            };
        }
        if let Some(m) = mapper.take() {
            mappers.push(m);
        }

        let mut mappers = mappers.drain(0..mappers.len());
        Ok(
            Almanac {
                seeds,
                seed_to_soil: mappers.next().unwrap(),
                soil_to_fertilizer: mappers.next().unwrap(),
                fertilizer_to_water: mappers.next().unwrap(),
                water_to_light: mappers.next().unwrap(),
                light_to_temperature: mappers.next().unwrap(),
                temperature_to_humidity: mappers.next().unwrap(),
                humidity_to_location: mappers.next().unwrap(),
            }
        )
    }
}

struct Mapping {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl Mapping {
    fn new(dest_start: u64, source_start: u64, length: u64) -> Self {
        Mapping { dest_start, source_start, length }
    }

    fn maps_to(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source >= self.source_start + self.length {
            None
        } else {
            Some(self.dest_start + source - self.source_start)
        }
    }
}

struct Mapper {
    mappings: Vec<Mapping>,
}

impl Mapper {
    fn new() -> Self {
        Mapper {
            mappings: Vec::new(),
        }
    }

    fn add(&mut self, dest_start: u64, source_start: u64, length: u64) {
        self.mappings.push(Mapping::new(dest_start, source_start, length));
    }

    fn get(&mut self, source: u64) -> u64 {
        let dest = self.mappings.iter()
            .map(|mapping| mapping.maps_to(source))
            .filter(|value| value.is_some())
            .map(|value| value.unwrap())
            .next();

        match dest {
            None => source,
            Some(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_05/input.txt");
        let mut almanac = Almanac::from_str(input).unwrap();

        let seeds = almanac.seeds.clone();
        let locations = &almanac.locations_of(&seeds);

        assert_eq!(&535088217, locations.iter().min().unwrap());
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_05/input.txt");
        let mut almanac = Almanac::from_str(input).unwrap();

        let mut locations: Vec<Option<u64>> = Vec::new();
        let seeds = almanac.seeds();
        for seed in seeds {
            let location = almanac.smallest_location_of(seed.all());
            locations.push(location);
        }

        let smallest = locations.iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .min().unwrap();
        assert_eq!(51399228, smallest);
    }

    #[test]
    fn example() {
        let input = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
        ";
        let mut almanac = Almanac::from_str(input).unwrap();

        let seeds = almanac.seeds.clone();
        let locations = &almanac.locations_of(&seeds);

        assert_eq!(&35, locations.iter().min().unwrap());
    }

    #[test]
    fn mapper_creation() {
        let mut mapper = Mapper::new();
        mapper.add(50, 98, 2);
        mapper.add(49, 90, 1);

        assert_eq!(1, mapper.get(1));
        assert_eq!(50, mapper.get(98));
        assert_eq!(51, mapper.get(99));
        assert_eq!(49, mapper.get(90));
    }
}
