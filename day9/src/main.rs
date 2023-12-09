type SeqItem = i32;
type Seq = Vec<SeqItem>;

enum Predict {
    Prev,
    Next,
}

fn predict(mode: Predict, seq: Seq) -> SeqItem {
    std::iter::repeat(())
        .try_fold(vec![seq], |mut acc, ()| {
            let diff_seq = acc
                .last()
                .unwrap()
                .as_slice()
                .windows(2)
                .map(|window| {
                    let (l, r) = (window[0], window[1]);
                    let diff = r - l;
                    diff
                })
                .collect::<Vec<_>>();

            let all_zeroes = diff_seq.iter().all(|v| *v == 0);

            acc.push(diff_seq);

            if all_zeroes {
                Err(acc)
            } else {
                Ok(acc)
            }
        })
        .unwrap_err()
        .as_slice()
        .windows(2)
        .rev()
        .fold(0, |acc, window| {
            let (upper, _lower) = (&window[0], &window[1]);
            if matches!(mode, Predict::Next) {
                upper.last().unwrap() + acc
            } else {
                upper.first().unwrap() - acc
            }
        })
}

mod part_one {
    use super::*;

    pub fn part_one() -> SeqItem {
        read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .map(|seq| predict(Predict::Next, seq))
            .sum::<SeqItem>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 1637452029);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> SeqItem {
        read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .map(|seq| predict(Predict::Prev, seq))
            .sum::<SeqItem>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 908);
    }
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

fn parse_line(value: String) -> Seq {
    value
        .split(" ")
        .map(std::str::FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}

fn main() {
    let part_one = part_one::part_one(); // 1637452029
    dbg!(part_one);

    let part_two = part_two::part_two(); // 908
    dbg!(part_two);
}
