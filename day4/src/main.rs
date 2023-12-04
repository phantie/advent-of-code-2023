mod part_one {
    pub fn part_one() -> u32 {
        super::read_input()
            .map(Result::unwrap)
            .map(calc)
            .sum::<u32>()
    }

    use super::parse;

    fn calc(value: String) -> u32 {
        let (_id, (winning, given)) = parse::process_line(value);

        let q = given.iter().filter(|v| winning.contains(*v)).count() as u32;

        if q == 0 {
            0
        } else if q == 1 {
            1
        } else {
            2u32.pow(q - 1)
        }
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 535351);
    }
}

mod part_two {
    pub fn part_two() -> u32 {
        let q = calc(
            super::read_input()
                .map(Result::unwrap)
                .map(parse::process_line)
                .collect(),
        );
        dbg!(q);
        q
    }

    use super::parse;

    fn calc(cards: Vec<parse::Card>) -> u32 {
        let total = cards.len();

        let mut q = std::collections::HashMap::<u32, u32>::new();

        for card in cards.iter() {
            let (id, (winning, given)) = card;

            let winning_nums_count = given.iter().filter(|v| winning.contains(*v)).count() as u32;

            #[allow(unused_parens)]
            for _ in (0..(*q.get(id).unwrap_or(&0) + 1)) {
                for id in ((*id + 1)..(*id + 1 + winning_nums_count)) {
                    q.entry(id).and_modify(|v| *v = (*v) + 1).or_insert(1);
                }
            }
        }

        total as u32 + q.values().sum::<u32>()
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete,
        multi::separated_list1,
        IResult,
    };

    type Id = u32;
    pub type Card = (Id, (Vec<u32>, Vec<u32>));

    fn parse_game(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, result) = separated_list1(alt((tag(" "), tag("  "))), complete::u32)(input)?;
        Ok((input, result))
    }

    fn parse_games(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
        let (input, result) = separated_list1(tag(" | "), parse_game)(input)?;
        Ok((input, result))
    }

    fn parse_line(input: &str) -> IResult<&str, (Id, Vec<Vec<u32>>)> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = take_while1(|v| v == ' ')(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = alt((tag(":  "), tag(": ")))(input)?;
        let (input, game) = parse_games(input)?;
        Ok((input, (id, game)))
    }

    pub fn process_line(input: String) -> Card {
        let input = input.replace("  ", " ");
        let i = input.as_str();

        let (_, (id, game)) = parse_line(i).unwrap();

        let mut q = game.into_iter();

        let winning = q.next().unwrap();
        let given = q.next().unwrap();

        (id, (winning, given))
    }
}

fn main() {
    let result = part_one::part_one();
    println!("result: {result}");
    let result = part_two::part_two();
    println!("result: {result}");
}
