fn main() {
    dbg!(part_one::part_one()); // 20093
    dbg!(part_two::part_two()); // 22103062509257
}

mod parse {
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::newline,
        multi::separated_list1,
        IResult,
    };

    #[derive(Debug)]
    pub struct Parsed {
        pub directions: Vec<Direction>,
        pub mappings: Mappings,
    }

    #[derive(Debug)]
    pub struct Mapping {
        current: String,
        left: String,
        right: String,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        Left,
        Right,
    }

    #[derive(Debug)]
    pub struct Choices {
        left: String,
        right: String,
    }

    impl Choices {
        pub fn from_direction(&self, direction: Direction) -> &str {
            match direction {
                Direction::Left => self.left.as_str(),
                Direction::Right => self.right.as_str(),
            }
        }
    }

    impl From<char> for Direction {
        fn from(value: char) -> Self {
            match value {
                'L' => Self::Left,
                'R' => Self::Right,
                _ => unreachable!(),
            }
        }
    }

    type Mappings = std::collections::HashMap<String, Choices>;

    pub fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
        let (input, current) = take_until(" ")(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, left) = take_until(",")(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right) = take_until(")")(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((
            input,
            Mapping {
                current: current.into(),
                left: left.into(),
                right: right.into(),
            },
        ))
    }

    pub fn parse_input(input: &str) -> IResult<&str, Parsed> {
        let (input, directions) = take_until("\n")(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;

        let (input, mappings) = separated_list1(newline, parse_mapping)(input)?;

        let mappings = mappings
            .into_iter()
            .map(
                |Mapping {
                     current,
                     left,
                     right,
                 }| (current, Choices { left, right }),
            )
            .collect::<Mappings>();

        Ok((
            input,
            Parsed {
                mappings,
                directions: directions.chars().map(char::into).collect(),
            },
        ))
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

mod part_one {
    use super::*;

    pub fn part_one() -> usize {
        let (_, parsed) = parse::parse_input(input()).unwrap();

        let mut acc = "AAA";
        for (i, direction) in parsed.directions.iter().cycle().enumerate() {
            acc = parsed.mappings.get(acc).unwrap().from_direction(*direction);

            if acc == "ZZZ" {
                return i + 1;
            }
        }

        unreachable!()
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> u64 {
        let (_, parsed) = parse::parse_input(input()).unwrap();

        let mut starts = parsed
            .mappings
            .keys()
            .filter(|key| key.ends_with("A"))
            .map(|key| (key.as_str(), key.as_str(), 0))
            .collect::<Vec<_>>();

        for (_key, current, iteration) in starts.iter_mut() {
            for (i, direction) in parsed.directions.iter().cycle().enumerate() {
                *current = parsed
                    .mappings
                    .get(*current)
                    .unwrap()
                    .from_direction(*direction);

                if current.ends_with("Z") {
                    *iteration = i;
                    break;
                }
            }
        }

        starts
            .into_iter()
            .map(|(_, _, x)| x as u64 + 1)
            .reduce(num::integer::lcm)
            .unwrap()
    }
}
