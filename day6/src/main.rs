use part_one::part_one;

fn main() {
    let (_, races) = parse::parse_input(input()).unwrap();

    dbg!(part_one(races)); // 219849
}

mod part_one {
    pub fn part_one(races: Vec<crate::parse::Race>) -> usize {
        races
            .into_iter()
            .map(|race| race.ways_to_beat_the_record())
            .fold(1, |acc, x| acc * x)
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
        time: u32,
        distance: u32,
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
        let (input, times) = separated_list1(complete::space1, complete::u32)(input)?;
        let (input, _) = complete::newline(input)?;

        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distances) = separated_list1(complete::space1, complete::u32)(input)?;

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
