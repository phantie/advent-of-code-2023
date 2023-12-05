mod part_one {
    use super::*;

    pub fn part_one() -> i64 {
        let (_, parsed) = parse::parse_input(input()).unwrap();

        parsed
            .seeds
            .clone()
            .into_iter()
            .map(|source| {
                parsed
                    .flowing()
                    .into_iter()
                    .fold(source, |dest, table| table.source_to_dest(dest))
            })
            .min()
            .unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 993500720);
    }
}

fn main() {
    let result = part_one::part_one();
    println!("result: {result}");
    let result = part_two::part_two();
    println!("result: {result}");
}

mod parse {
    use std::ops::Range;

    use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

    type Dest = i64;
    type Source = i64;

    #[derive(Debug, Clone)]
    pub struct Mapping {
        pub dest: Dest,
        pub source: Source,
        pub range: i64,
    }

    impl Mapping {
        pub fn source_to_dest(&self, source: Source) -> Option<Dest> {
            if self.source_range().contains(&source) {
                Some(source + (self.dest - self.source))
            } else {
                None
            }
        }

        pub fn source_range(&self) -> Range<i64> {
            self.source..self.source + self.range
        }
    }

    fn parse_range_line(input: &str) -> IResult<&str, Mapping> {
        let (input, dest) = complete::i64(input)?;
        let (input, _) = complete::space1(input)?;
        let (input, source) = complete::i64(input)?;
        let (input, _) = complete::space1(input)?;
        let (input, range) = complete::i64(input)?;
        Ok((
            input,
            Mapping {
                dest,
                source,
                range,
            },
        ))
    }

    fn parse_table<'i>(input: &'i str, name: &str) -> IResult<&'i str, Vec<Mapping>> {
        let (input, _) = tag(format!("{name} map:").as_str())(input)?;
        let (input, _) = complete::newline(input)?;
        let (input, mappings) = separated_list1(complete::newline, parse_range_line)(input)?;
        Ok((input, mappings))
    }

    type Seed = i64;

    #[derive(Debug)]
    pub struct Mappings {
        pub inner: Vec<Mapping>,
    }

    impl Mappings {
        pub fn new(inner: Vec<Mapping>) -> Self {
            Self { inner }
        }

        pub fn source_to_dest(&self, source: Source) -> Dest {
            let dest = self
                .inner
                .iter()
                .map(|mapping| mapping.source_to_dest(source))
                .find(Option::is_some);

            if dest.is_none() {
                source
            } else {
                dest.unwrap().unwrap()
            }
        }
    }

    impl From<Vec<Mapping>> for Mappings {
        fn from(value: Vec<Mapping>) -> Self {
            Self::new(value)
        }
    }

    #[derive(Debug)]
    pub struct Parsed {
        pub seeds: Vec<Seed>,
        seed_to_soil: Mappings,
        soil_to_fertilizer: Mappings,
        fertilizer_to_water: Mappings,
        water_to_light: Mappings,
        light_to_temperature: Mappings,
        temperature_to_humidity: Mappings,
        pub humidity_to_location: Mappings,
    }

    impl Parsed {
        pub fn flowing(&self) -> Vec<&Mappings> {
            vec![
                &self.seed_to_soil,
                &self.soil_to_fertilizer,
                &self.fertilizer_to_water,
                &self.water_to_light,
                &self.light_to_temperature,
                &self.temperature_to_humidity,
                &self.humidity_to_location,
            ]
        }
    }

    pub fn parse_input(input: &str) -> IResult<&str, Parsed> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(complete::space1, complete::i64)(input)?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, seed_to_soil) = parse_table(input, "seed-to-soil")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, soil_to_fertilizer) = parse_table(input, "soil-to-fertilizer")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, fertilizer_to_water) = parse_table(input, "fertilizer-to-water")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, water_to_light) = parse_table(input, "water-to-light")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, light_to_temperature) = parse_table(input, "light-to-temperature")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, temperature_to_humidity) = parse_table(input, "temperature-to-humidity")?;
        let (input, _) = complete::newline(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, humidity_to_location) = parse_table(input, "humidity-to-location")?;

        Ok((
            input,
            Parsed {
                seeds,
                seed_to_soil: seed_to_soil.into(),
                soil_to_fertilizer: soil_to_fertilizer.into(),
                fertilizer_to_water: fertilizer_to_water.into(),
                water_to_light: water_to_light.into(),
                light_to_temperature: light_to_temperature.into(),
                temperature_to_humidity: temperature_to_humidity.into(),
                humidity_to_location: humidity_to_location.into(),
            },
        ))
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

mod part_two {
    use super::*;

    pub fn part_two() -> i64 {
        let (_, parsed) = parse::parse_input(input()).unwrap();

        let ranges = parsed
            .seeds
            .chunks_exact(2)
            .map(|chunk| {
                let (start, range) = (chunk[0], chunk[1]);
                start..start + range
            })
            .collect::<Vec<_>>();

        ranges
            .into_iter()
            .flatten()
            .map(|source| {
                parsed
                    .flowing()
                    .into_iter()
                    .fold(source, |dest, table| table.source_to_dest(dest))
            })
            .min()
            .unwrap()
    }

    #[cfg(test)]
    #[ignore]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 4917124);
    }
}
