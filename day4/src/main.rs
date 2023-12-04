mod part_one {
    use super::parse::{process_line, Card};

    pub fn part_one() -> u32 {
        super::read_input()
            .map(Result::unwrap)
            .map(process_line)
            .map(calc_points)
            .sum::<u32>()
    }

    fn calc_points(card: Card) -> u32 {
        let count = card.winning_count();

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
    use super::parse::{process_line, Card};

    pub fn part_two() -> u32 {
        calc(
            super::read_input()
                .map(Result::unwrap)
                .map(process_line)
                .collect(),
        )
    }

    fn calc(cards: Vec<Card>) -> u32 {
        let total = cards.len() as u32;

        let mut copies = std::collections::HashMap::<u32, u32>::new();

        for card in cards.iter() {
            let id = card.id;

            let winning_count = card.winning_count();

            for _ in 0..*copies.get(&id).unwrap_or(&0) + 1 {
                for id in (id + 1)..(id + 1 + winning_count) {
                    copies.entry(id).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }

        total + copies.values().sum::<u32>()
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
    use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

    type Id = u32;

    #[derive(Debug)]
    pub struct Card {
        pub id: Id,
        pub winning_nums: WinningNums,
        pub given_nums: GivenNums,
    }

    impl Card {
        pub fn winning_count(&self) -> u32 {
            self.given_nums
                .iter()
                .filter(|v| self.winning_nums.contains(*v))
                .count() as u32
        }
    }

    type Nums = Vec<u32>;
    type WinningNums = Nums;
    type GivenNums = Nums;

    fn parse_nums(input: &str) -> IResult<&str, Nums> {
        let (input, result) = separated_list1(complete::space1, complete::u32)(input)?;
        Ok((input, result))
    }

    fn parse_num_sets(input: &str) -> IResult<&str, (WinningNums, GivenNums)> {
        let (input, winning) = parse_nums(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, _) = complete::space0(input)?;
        let (input, given) = parse_nums(input)?;
        Ok((input, (winning, given)))
    }

    fn parse_line(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = complete::space1(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = complete::space1(input)?;
        let (input, (winning_nums, given_nums)) = parse_num_sets(input)?;
        Ok((
            input,
            Card {
                id,
                winning_nums,
                given_nums,
            },
        ))
    }

    pub fn process_line(input: String) -> Card {
        let (_, card) = parse_line(&input).unwrap();
        card
    }

    #[cfg(test)]
    #[test]
    fn test_parse_line() {
        let (remaining, card) =
            parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into()).unwrap();
        assert!(remaining.is_empty());
        assert_eq!(card.id, 3);
        assert_eq!(card.winning_nums, vec![1, 21, 53, 59, 44]);
        assert_eq!(card.given_nums, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }
}

fn main() {
    let result = part_one::part_one();
    println!("result: {result}");
    let result = part_two::part_two();
    println!("result: {result}");
}
