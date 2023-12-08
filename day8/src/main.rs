fn main() {
    dbg!(part_one::part_one());
    dbg!(part_two::part_two());
}

#[derive(Debug)]
pub struct Parsed {
    pub directions: Vec<Direction>,
    pub mappings: Mappings,
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

mod parse {
    use super::*;
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::newline,
        multi::separated_list1,
        IResult,
    };

    #[derive(Debug)]
    pub struct Mapping {
        current: String,
        left: String,
        right: String,
    }

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

        let mut walk_state = WalkState::new("AAA");

        for (i, direction) in parsed.directions.iter().cycle().enumerate() {
            walk_state.next(&parsed.mappings, *direction);

            if walk_state.as_ref() == "ZZZ" {
                return i + 1;
            }
        }

        unreachable!()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 20093);
    }
}

struct WalkState<'str> {
    current: &'str str,
}

impl<'str> WalkState<'str> {
    pub fn new(init: &'str str) -> Self {
        Self { current: init }
    }

    pub fn next(&mut self, mappings: &'str Mappings, direction: Direction) {
        self.current = mappings
            .get(self.current)
            .unwrap()
            .from_direction(direction);
    }
}

impl<'str> AsRef<str> for WalkState<'str> {
    fn as_ref(&self) -> &str {
        &self.current
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> usize {
        let (_, parsed) = parse::parse_input(input()).unwrap();

        let mut starts = parsed
            .mappings
            .keys()
            .filter(|key| key.ends_with("A"))
            .map(|key| (key.as_str(), WalkState::new(&key), 0))
            .collect::<Vec<_>>();

        for (_key, walk_state, iteration) in starts.iter_mut() {
            for (i, direction) in parsed.directions.iter().cycle().enumerate() {
                walk_state.next(&parsed.mappings, *direction);

                if walk_state.as_ref().ends_with("Z") {
                    *iteration = i;
                    break;
                }
            }
        }

        starts
            .into_iter()
            .map(|(_, _, iteration)| iteration + 1)
            .reduce(num::integer::lcm)
            .unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 22103062509257);
    }
}
