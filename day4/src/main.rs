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

        let count = given.iter().filter(|v| winning.contains(*v)).count() as u32;

        if count == 0 || count == 1 {
            count
        } else {
            2u32.pow(count - 1)
        }
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 18653);
    }
}

mod part_two {
    pub fn part_two() -> u32 {
        calc(
            super::read_input()
                .map(Result::unwrap)
                .map(parse::process_line)
                .collect(),
        )
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

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 5921508);
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
    type Nums = Vec<u32>;
    type WinningNums = Nums;
    type GivenNums = Nums;

    fn parse_nums(input: &str) -> IResult<&str, Nums> {
        let (input, result) = separated_list1(alt((tag(" "), tag("  "))), complete::u32)(input)?;
        Ok((input, result))
    }

    fn parse_num_sets(input: &str) -> IResult<&str, (WinningNums, GivenNums)> {
        let (input, winning) = parse_nums(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, given) = parse_nums(input)?;
        Ok((input, (winning, given)))
    }

    fn parse_line(input: &str) -> IResult<&str, (Id, (WinningNums, GivenNums))> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = take_while1(|v| v == ' ')(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = alt((tag(":  "), tag(": ")))(input)?;
        let (input, nums) = parse_num_sets(input)?;
        Ok((input, (id, nums)))
    }

    pub fn process_line(input: String) -> Card {
        let input = input.replace("  ", " ");
        let i = input.as_str();

        let (_, card) = parse_line(i).unwrap();

        card
    }
}

fn main() {
    let result = part_one::part_one();
    println!("result: {result}");
    let result = part_two::part_two();
    println!("result: {result}");
}
