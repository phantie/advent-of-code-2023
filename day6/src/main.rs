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
    use std::cmp::Ordering;

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

        fn binary_search(range_len: usize, f: impl Fn(usize) -> Ordering) -> Option<usize> {
            let length = range_len;
            let mut half = length / 2;
            let mut hind = length - 1;
            let mut lind = 0;
            let mut current = half;

            while lind <= hind {
                let cmp = f(current);

                match cmp {
                    Ordering::Equal => return Some(half),
                    Ordering::Less => lind = half + 1,
                    Ordering::Greater => hind = half - 1,
                }
                half = (hind + lind) / 2;
                current = half;
            }
            return None;
        }

        let start_idx = binary_search(race.time as usize, |current| {
            match (
                race.beats_record_with_hold(current as u64),
                race.beats_record_with_hold(current as u64 + 1),
            ) {
                (false, true) => Ordering::Equal,
                (false, false) => Ordering::Less, // ambiguous but works in example
                (true, true) => Ordering::Greater,
                (true, false) => Ordering::Greater,
            }
        })
        .unwrap()
            + 1;

        let end_idx = binary_search(race.time as usize - start_idx, |current| {
            match (
                race.beats_record_with_hold((current + start_idx) as u64),
                race.beats_record_with_hold((current + start_idx + 1) as u64),
            ) {
                (true, false) => Ordering::Equal,
                (true, true) => Ordering::Less,
                (false, true) => Ordering::Less,
                (false, false) => Ordering::Greater,
            }
        })
        .unwrap()
            + start_idx
            + 1;

        end_idx - start_idx
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
