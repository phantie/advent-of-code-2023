mod part_one {
    use super::Race;

    pub fn part_one(races: Vec<Race>) -> usize {
        races
            .into_iter()
            .map(|race| race.ways_to_beat_the_record())
            .fold(1, |acc, x| acc * x)
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        let (_, races) = crate::parse::parse_input(crate::input()).unwrap();
        assert_eq!(part_one(races), 219849);
    }
}

mod part_two {
    use super::Race;

    pub fn part_two(races: Vec<Race>) -> usize {
        let (times, distances) = races.into_iter().fold(
            (String::new(), String::new()),
            |(times, distances), Race { time, distance }| {
                (
                    format!("{times}{}", time),
                    format!("{distances}{}", distance),
                )
            },
        );

        let race = Race {
            time: times.parse().unwrap(),
            distance: distances.parse().unwrap(),
        };

        let start_idx = (1..race.time)
            .find(|hold_millis| race.beats_record_with_hold(*hold_millis))
            .unwrap() as usize;

        let end_idx = (1..race.time)
            .rev()
            .find(|hold_millis| race.beats_record_with_hold(*hold_millis))
            .unwrap() as usize;

        end_idx - start_idx + 1
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        let (_, races) = crate::parse::parse_input(crate::input()).unwrap();
        assert_eq!(part_two(races), 29432455);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn ways_to_beat_the_record(&self) -> usize {
        (1..self.time)
            .filter(|hold_millis| self.beats_record_with_hold(*hold_millis))
            .count()
    }

    pub fn beats_record_with_hold(&self, hold_millis: u64) -> bool {
        let time_remaining = self.time - hold_millis;
        let speed = hold_millis;
        speed * time_remaining > self.distance
    }
}

mod parse {
    use super::Race;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, space1},
        multi::separated_list1,
        IResult,
    };

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = space1(input)?;
        let (input, times) = separated_list1(complete::space1, complete::u64)(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distances) = separated_list1(complete::space1, complete::u64)(input)?;

        Ok((
            input,
            times
                .into_iter()
                .zip(distances.into_iter())
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        ))
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

fn main() {
    let (_, races) = parse::parse_input(input()).unwrap();
    dbg!(part_one::part_one(races.clone()));
    dbg!(part_two::part_two(races));
}
