mod part_one {
    use super::parse::*;
    use super::*;

    pub fn part_one() -> u32 {
        read_input().map(Result::unwrap).map(id_if_possible).sum()
    }

    pub fn id_if_possible(v: String) -> u32 {
        let (id, games) = process_line(&v);

        let possible = games
            .into_iter()
            .map(|game| {
                game.into_iter().all(|CountColor { count, color }| {
                    (color == "red" && count <= 12)
                        || (color == "green" && count <= 13)
                        || (color == "blue" && count <= 14)
                })
            })
            .all(std::convert::identity);

        if possible {
            id
        } else {
            0
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn part_one() {
            assert_eq!(super::part_one(), 2617);
        }
    }
}

mod part_two {
    use super::parse::*;
    use super::*;

    pub fn part_two() -> u32 {
        read_input().map(Result::unwrap).map(mult_pow).sum()
    }

    pub fn mult_pow(v: String) -> u32 {
        let (_id, games) = process_line(&v);

        let mut color_count = std::collections::HashMap::<_, u32>::new();

        for game in games {
            for CountColor { count, color } in game {
                color_count
                    .entry(color)
                    .and_modify(|v| {
                        *v = (*v).max(count);
                    })
                    .or_insert(count);
            }
        }

        color_count.values().fold(1, |acc, x| acc * x)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn part_two() {
            assert_eq!(super::part_two(), 59795);
        }
    }
}

fn main() {
    dbg!(part_one::part_one()); // 2617
    dbg!(part_two::part_two()); // 59795
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

mod parse {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete, multi::separated_list1, IResult,
    };

    #[derive(Debug, PartialEq)]
    pub struct CountColor<'a> {
        pub count: u32,
        pub color: &'a str,
    }

    type Id = u32;
    type Game<'a> = Vec<CountColor<'a>>;
    type Games<'a> = Vec<Game<'a>>;

    fn parse_count_color(input: &str) -> IResult<&str, CountColor> {
        let (input, count) = complete::u32(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
        Ok((input, CountColor { count, color }))
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        let (input, result) = separated_list1(tag(", "), parse_count_color)(input)?;
        Ok((input, result))
    }

    fn parse_games(input: &str) -> IResult<&str, Games> {
        let (input, result) = separated_list1(tag("; "), parse_game)(input)?;
        Ok((input, result))
    }

    fn parse_line(input: &str) -> IResult<&str, (Id, Games)> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, game) = parse_games(input)?;
        Ok((input, (id, game)))
    }

    pub fn process_line(input: &str) -> (Id, Games) {
        let (_, result) = parse_line(input).unwrap();
        result
    }

    #[cfg(test)]
    #[test]
    fn test_parse_line() {
        let (remaining, (id, games)) =
            parse_line("Game 22: 3 red, 1 blue; 3 green, 1 red, 1 blue; 7 green, 2 blue").unwrap();

        assert!(remaining.is_empty());

        assert_eq!(id, 22);
        assert_eq!(
            games,
            vec![
                vec![
                    CountColor {
                        count: 3,
                        color: "red",
                    },
                    CountColor {
                        count: 1,
                        color: "blue",
                    },
                ],
                vec![
                    CountColor {
                        count: 3,
                        color: "green",
                    },
                    CountColor {
                        count: 1,
                        color: "red",
                    },
                    CountColor {
                        count: 1,
                        color: "blue",
                    },
                ],
                vec![
                    CountColor {
                        count: 7,
                        color: "green",
                    },
                    CountColor {
                        count: 2,
                        color: "blue",
                    },
                ],
            ]
        );
    }
}
