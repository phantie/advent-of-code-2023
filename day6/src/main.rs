use part_one::part_one;

use crate::part_two::part_two;

fn main() {
    let (_, races) = parse::parse_input(input()).unwrap();

    dbg!(part_one(races.clone())); // 219849
    dbg!(part_two(races)); // 29432455
}

mod part_one {
    pub fn part_one(races: Vec<crate::parse::Race>) -> usize {
        races
            .into_iter()
            .map(|race| race.ways_to_beat_the_record())
            .fold(1, |acc, x| acc * x)
    }
}

mod part_two {
    pub fn part_two(races: Vec<crate::parse::Race>) -> usize {
        let (times, distances) =
            races
                .into_iter()
                .fold((String::new(), String::new()), |(times, distances), x| {
                    (
                        format!("{times}{}", x.time),
                        format!("{distances}{}", x.distance),
                    )
                });

        let race = crate::parse::Race {
            time: times.parse().unwrap(),
            distance: distances.parse().unwrap(),
        };

        race.ways_to_beat_the_record()
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, space1},
        multi::separated_list1,
        IResult,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Race {
        pub time: u64,
        pub distance: u64,
    }

    impl Race {
        pub fn ways_to_beat_the_record(&self) -> usize {
            (0..self.time)
                .filter(|hold_millis| {
                    let time_remaining = self.time - hold_millis;
                    let speed = *hold_millis;
                    speed * time_remaining > self.distance
                })
                .count()
        }
    }

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
